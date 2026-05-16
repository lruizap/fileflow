import { useEffect, useMemo, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";

import {
  cancelJob as cancelQueuedJob,
  getQueueState,
  invokeFileFlow,
  setConcurrencyLimit as saveConcurrencyLimit,
  updateJobPriority as saveJobPriority,
} from "../api/fileflow";
import { usePersistentState } from "./usePersistentState";
import type {
  HistoryItem,
  JobPriority,
  ManagedJob,
  ProgressPayload,
  QueueState,
  RunCommand,
  ToastState,
} from "../types";

const EMPTY_QUEUE: QueueState = {
  jobs: [],
  concurrencyLimit: 2,
  runningCount: 0,
  queuedCount: 0,
};

const FINAL_STATUSES = new Set(["SUCCESS", "FAILED", "CANCELLED"]);
const STATUS_RANK = {
  QUEUED: 0,
  RUNNING: 1,
  SUCCESS: 2,
  FAILED: 2,
  CANCELLED: 2,
} as const;

export function useFileFlowRunner() {
  const [queue, setQueue] = useState<QueueState>(EMPTY_QUEUE);
  const [status, setStatus] = useState("READY");
  const [logs, setLogs] = useState<string[]>([
    "FileFlow inicializado.",
    "Selecciona una acción para comenzar.",
  ]);

  const [history, setHistory] = usePersistentState<HistoryItem[]>(
    "fileflow.history.v0.6.0",
    [],
  );
  const [defaultPriority, setDefaultPriority] =
    usePersistentState<JobPriority>("fileflow.defaultPriority.v0.6.0", "normal");
  const [toast, setToast] = useState<ToastState>(null);
  const [cancellingJobIds, setCancellingJobIds] = useState<number[]>([]);
  const completedJobIds = useRef<Set<number>>(new Set());

  const activeJobs = useMemo(
    () =>
      queue.jobs.filter(
        (job) => job.status === "RUNNING" || job.status === "QUEUED",
      ),
    [queue.jobs],
  );
  const loading = activeJobs.length > 0;

  useEffect(() => {
    getQueueState()
      .then(applyQueueState)
      .catch(() => undefined);
  }, []);

  useEffect(() => {
    const tauriInternals = (
      window as Window & {
        __TAURI_INTERNALS__?: { transformCallback?: unknown };
      }
    ).__TAURI_INTERNALS__;

    if (typeof tauriInternals?.transformCallback !== "function") {
      return;
    }

    const progressListener = listen<ProgressPayload>(
      "fileflow-progress",
      (event) => {
        setQueue((current) => ({
          ...current,
          jobs: current.jobs.map((job) =>
            job.id === event.payload.jobId
              ? { ...job, progress: event.payload }
              : job,
          ),
        }));
      },
    );

    const jobListener = listen<ManagedJob>("fileflow-job-updated", (event) => {
      upsertJob(event.payload);
      handleCompletedJob(event.payload);
    });

    return () => {
      progressListener.then((unlisten) => unlisten()).catch(() => undefined);
      jobListener.then((unlisten) => unlisten()).catch(() => undefined);
    };
  }, []);

  useEffect(() => {
    if (queue.runningCount > 0 || queue.queuedCount > 0) {
      setStatus(`${queue.runningCount} RUNNING / ${queue.queuedCount} QUEUED`);
      return;
    }

    const lastFinished = queue.jobs.find((job) =>
      FINAL_STATUSES.has(job.status),
    );
    setStatus(lastFinished?.status ?? "READY");
  }, [queue]);

  function applyQueueState(nextQueue: QueueState) {
    setQueue(nextQueue);
  }

  function upsertJob(job: ManagedJob) {
    setQueue((current) => {
      const existingJob = current.jobs.find((item) => item.id === job.id);
      const shouldReplace =
        !existingJob || STATUS_RANK[job.status] >= STATUS_RANK[existingJob.status];
      const nextJob = shouldReplace ? mergeJob(existingJob, job) : existingJob;
      const exists = Boolean(existingJob);
      const jobs = exists
        ? current.jobs.map((item) => (item.id === job.id ? nextJob : item))
        : [job, ...current.jobs];

      return recalculateQueue({ ...current, jobs });
    });

    if (job.logs.length > 0) {
      setLogs(job.logs);
    }

    setCancellingJobIds((ids) => ids.filter((id) => id !== job.id));
  }

  function mergeJob(
    current: ManagedJob | undefined,
    incoming: ManagedJob,
  ): ManagedJob {
    if (!current) return incoming;

    return {
      ...current,
      ...incoming,
      progress: incoming.progress ?? current.progress,
      logs: incoming.logs.length > 0 ? incoming.logs : current.logs,
      error: incoming.error ?? current.error,
    };
  }

  function handleCompletedJob(job: ManagedJob) {
    if (!FINAL_STATUSES.has(job.status) || completedJobIds.current.has(job.id)) {
      return;
    }

    completedJobIds.current.add(job.id);
    pushHistory(job.label, job.command, job.status);

    if (job.status === "SUCCESS") {
      showToast({ type: "success", message: `${job.label} completado.` });
    } else if (job.status === "CANCELLED") {
      showToast({ type: "info", message: `${job.label} cancelado.` });
    } else {
      showToast({ type: "error", message: `${job.label} terminó con errores.` });
    }
  }

  function recalculateQueue(nextQueue: QueueState): QueueState {
    const runningCount = nextQueue.jobs.filter(
      (job) => job.status === "RUNNING",
    ).length;
    const queuedCount = nextQueue.jobs.filter(
      (job) => job.status === "QUEUED",
    ).length;

    return {
      ...nextQueue,
      jobs: [...nextQueue.jobs].sort((left, right) => right.id - left.id),
      runningCount,
      queuedCount,
    };
  }

  function pushHistory(label: string, command: string, status: string) {
    const item: HistoryItem = {
      id: crypto.randomUUID(),
      label,
      command,
      status,
      createdAt: new Date().toLocaleTimeString(),
    };

    setHistory((prev) => [item, ...prev].slice(0, 30));
  }

  function showToast(toast: ToastState) {
    setToast(toast);
    window.setTimeout(() => setToast(null), 4500);
  }

  const runCommand: RunCommand = async (
    command,
    args,
    label = command,
    priority,
  ) => {
    try {
      const job = await invokeFileFlow(
        command,
        args,
        priority ?? defaultPriority,
      );

      if (job.id === 0 && FINAL_STATUSES.has(job.status)) {
        handleInstantJob(job);
        return;
      }

      upsertJob(job);
      if (job.status === "QUEUED" || job.status === "RUNNING") {
        showToast({ type: "info", message: `${label} añadido a la cola.` });
      } else {
        handleCompletedJob(job);
      }
    } catch (err) {
      const errorMessage =
        typeof err === "string" ? err : JSON.stringify(err, null, 2);

      setStatus("ERROR");
      setLogs(["Error encolando comando:", errorMessage]);
      pushHistory(label, command, "ERROR");

      showToast({ type: "error", message: `No se pudo encolar ${label}.` });
    }
  };

  function handleInstantJob(job: ManagedJob) {
    if (job.logs.length > 0) {
      setLogs(job.logs);
    }

    setStatus(job.status);
    pushHistory(job.label, job.command, job.status);

    if (job.status === "SUCCESS") {
      showToast({ type: "success", message: `${job.label} completado.` });
    } else if (job.status === "CANCELLED") {
      showToast({ type: "info", message: `${job.label} cancelado.` });
    } else {
      showToast({ type: "error", message: `${job.label} terminó con errores.` });
    }
  }

  async function cancelJob(jobId: number) {
    setCancellingJobIds((ids) =>
      ids.includes(jobId) ? ids : [...ids, jobId],
    );

    try {
      const job = await cancelQueuedJob(jobId);
      upsertJob(job);
      showToast({ type: "info", message: "Cancelando proceso..." });
    } catch (err) {
      setCancellingJobIds((ids) => ids.filter((id) => id !== jobId));
      showToast({
        type: "error",
        message: `No se pudo cancelar: ${String(err)}`,
      });
    }
  }

  async function setConcurrencyLimit(limit: number) {
    try {
      const nextQueue = await saveConcurrencyLimit(limit);
      applyQueueState(nextQueue);
    } catch (err) {
      showToast({
        type: "error",
        message: `No se pudo cambiar el límite: ${String(err)}`,
      });
    }
  }

  async function updateJobPriority(jobId: number, priority: JobPriority) {
    try {
      const job = await saveJobPriority(jobId, priority);
      upsertJob(job);
    } catch (err) {
      showToast({
        type: "error",
        message: `No se pudo cambiar la prioridad: ${String(err)}`,
      });
    }
  }

  return {
    status,
    logs,
    history,
    toast,
    loading,
    queue,
    activeJobs,
    defaultPriority,
    cancellingJobIds,
    runCommand,
    cancelJob,
    setConcurrencyLimit,
    setDefaultPriority,
    updateJobPriority,
    setToast,
    setLogs,
    setHistory,
  };
}
