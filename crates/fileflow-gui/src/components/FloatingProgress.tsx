import type { JobPriority, ManagedJob } from "../types";

type Props = {
  jobs: ManagedJob[];
  visible: boolean;
  cancellingJobIds: number[];
  onCancel: (jobId: number) => void;
  onUpdatePriority: (jobId: number, priority: JobPriority) => void;
};

const PRIORITY_LABELS: Record<JobPriority, string> = {
  low: "Baja",
  normal: "Normal",
  high: "Alta",
  critical: "Crítica",
};

function formatSeconds(seconds: number | null | undefined) {
  if (seconds === null || seconds === undefined) return "Calculando...";

  if (seconds < 60) return `${seconds}s`;

  const minutes = Math.floor(seconds / 60);
  const rest = seconds % 60;

  if (minutes < 60) return `${minutes}m ${rest}s`;

  const hours = Math.floor(minutes / 60);
  const min = minutes % 60;

  return `${hours}h ${min}m`;
}

function formatBytes(bytes: number) {
  if (bytes >= 1024 ** 3) return `${(bytes / 1024 ** 3).toFixed(2)} GB`;
  if (bytes >= 1024 ** 2) return `${(bytes / 1024 ** 2).toFixed(2)} MB`;
  if (bytes >= 1024) return `${(bytes / 1024).toFixed(2)} KB`;
  return `${bytes} B`;
}

export function FloatingProgress({
  jobs,
  visible,
  cancellingJobIds,
  onCancel,
  onUpdatePriority,
}: Props) {
  if (!visible || jobs.length === 0) return null;

  return (
    <aside className="floating-progress">
      <div className="progress-stack-title">
        <strong>Procesos activos</strong>
        <span>{jobs.length}</span>
      </div>

      <div className="progress-stack">
        {jobs.map((job) => (
          <ProgressJob
            key={job.id}
            job={job}
            cancelling={cancellingJobIds.includes(job.id)}
            onCancel={onCancel}
            onUpdatePriority={onUpdatePriority}
          />
        ))}
      </div>
    </aside>
  );
}

type ProgressJobProps = {
  job: ManagedJob;
  cancelling: boolean;
  onCancel: (jobId: number) => void;
  onUpdatePriority: (jobId: number, priority: JobPriority) => void;
};

function ProgressJob({
  job,
  cancelling,
  onCancel,
  onUpdatePriority,
}: ProgressJobProps) {
  const progress = job.progress;
  const percent = Math.max(0, Math.min(100, progress?.percent ?? 0));
  const isQueued = job.status === "QUEUED";

  return (
    <article className={`progress-job progress-job-${job.status.toLowerCase()}`}>
      <div className="progress-top">
        <span className="progress-dot" />
        <strong>{progress?.action ?? job.label}</strong>
        <span className="progress-status">{job.status}</span>
      </div>

      <p className="progress-file">
        {isQueued ? "Esperando hueco de ejecución" : progress?.file ?? "Preparando operación..."}
      </p>

      <div className="progress-bar">
        <div style={{ width: `${percent}%` }} />
      </div>

      <div className="progress-meta">
        <span>{percent.toFixed(1)}%</span>
        <span>
          {formatBytes(progress?.current ?? 0)} / {formatBytes(progress?.total ?? 1)}
        </span>
      </div>

      <div className="progress-time">
        <span>Transcurrido: {formatSeconds(progress?.elapsedSeconds)}</span>
        <span>Restante: {formatSeconds(progress?.etaSeconds)}</span>
      </div>

      <div className="progress-actions">
        <select
          value={job.priority}
          disabled={!isQueued}
          onChange={(event) =>
            onUpdatePriority(job.id, event.target.value as JobPriority)
          }
          aria-label={`Prioridad de ${job.label}`}
        >
          {(Object.keys(PRIORITY_LABELS) as JobPriority[]).map((priority) => (
            <option key={priority} value={priority}>
              {PRIORITY_LABELS[priority]}
            </option>
          ))}
        </select>

        <button
          className="cancel-progress-btn"
          type="button"
          onClick={() => onCancel(job.id)}
          disabled={cancelling}
        >
          {cancelling ? "Cancelando..." : "Cancelar"}
        </button>
      </div>
    </article>
  );
}
