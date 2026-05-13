import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import { invokeFileFlow } from "../api/fileflow";
import type {
  HistoryItem,
  ProgressPayload,
  RunCommand,
  ToastState,
} from "../types";

export function useFileFlowRunner() {
  const [status, setStatus] = useState("READY");
  const [logs, setLogs] = useState<string[]>([
    "FileFlow GUI inicializada.",
    "Selecciona una acción para comenzar.",
  ]);

  const [history, setHistory] = useState<HistoryItem[]>([]);
  const [toast, setToast] = useState<ToastState>(null);
  const [loading, setLoading] = useState(false);

  const [progress, setProgress] = useState<ProgressPayload | null>(null);
  const [showProgress, setShowProgress] = useState(false);
  const [cancelling, setCancelling] = useState(false);

  useEffect(() => {
    if (!("__TAURI_INTERNALS__" in window)) {
      return;
    }

    let active = true;
    let unlistenProgress: (() => void) | null = null;

    listen<ProgressPayload>("fileflow-progress", (event) =>
      setProgress(event.payload),
    )
      .then((unlisten) => {
        if (active) {
          unlistenProgress = unlisten;
        } else {
          unlisten();
        }
      })
      .catch((err) => {
        console.warn("No se pudo escuchar el progreso de FileFlow.", err);
      });

    return () => {
      active = false;
      unlistenProgress?.();
    };
  }, []);

  useEffect(() => {
    if (!loading) {
      setShowProgress(false);
      return;
    }

    const timer = window.setTimeout(() => setShowProgress(true), 1_000);
    return () => window.clearTimeout(timer);
  }, [loading]);

  function pushHistory(label: string, command: string, status: string) {
    const item: HistoryItem = {
      id: crypto.randomUUID(),
      label,
      command,
      status,
      createdAt: new Date().toLocaleTimeString(),
    };

    setHistory((prev) => [item, ...prev].slice(0, 20));
  }

  function showToast(toast: ToastState) {
    setToast(toast);
    window.setTimeout(() => setToast(null), 4500);
  }

  const runCommand: RunCommand = async (command, args, label = command) => {
    setLoading(true);
    setShowProgress(false);
    setProgress(null);
    setCancelling(false);
    setStatus("RUNNING");
    setLogs((prev) => [`Ejecutando: ${label}`, ...prev]);

    try {
      const result = await invokeFileFlow(command, args);

      setStatus(result.status);
      setLogs(result.logs);
      pushHistory(label, command, result.status);

      if (result.status.includes("SUCCESS")) {
        showToast({
          type: "success",
          message: `${label} completado correctamente.`,
        });
      } else if (result.status.includes("CANCELLED")) {
        showToast({ type: "info", message: "Operación cancelada." });
      } else {
        showToast({ type: "error", message: `${label} terminó con errores.` });
      }
    } catch (err) {
      const errorMessage =
        typeof err === "string" ? err : JSON.stringify(err, null, 2);

      setStatus("ERROR");
      setLogs(["Error ejecutando comando:", errorMessage]);
      pushHistory(label, command, "ERROR");

      showToast({ type: "error", message: `No se pudo ejecutar ${label}.` });
    } finally {
      setLoading(false);
      setCancelling(false);
      window.setTimeout(() => setProgress(null), 1500);
    }
  };

  async function cancelCurrentJob() {
    setCancelling(true);

    try {
      await invoke("cancel_current_job");
      showToast({ type: "info", message: "Cancelando operación..." });
    } catch (err) {
      showToast({
        type: "error",
        message: `No se pudo cancelar: ${String(err)}`,
      });
    }
  }

  return {
    status,
    logs,
    history,
    toast,
    loading,
    progress,
    showProgress,
    cancelling,
    runCommand,
    cancelCurrentJob,
    setToast,
    setLogs,
    setHistory,
  };
}
