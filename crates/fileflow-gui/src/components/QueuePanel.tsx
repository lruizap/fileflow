import type { JobPriority, ManagedJob } from "../types";

type Props = {
  jobs: ManagedJob[];
  runningCount: number;
  queuedCount: number;
  concurrencyLimit: number;
  defaultPriority: JobPriority;
  cancellingJobIds: number[];
  onSetConcurrencyLimit: (limit: number) => void;
  onSetDefaultPriority: (priority: JobPriority) => void;
  onUpdatePriority: (jobId: number, priority: JobPriority) => void;
  onCancel: (jobId: number) => void;
};

const PRIORITIES: Array<{ value: JobPriority; label: string }> = [
  { value: "low", label: "Baja" },
  { value: "normal", label: "Normal" },
  { value: "high", label: "Alta" },
  { value: "critical", label: "Crítica" },
];

export function QueuePanel({
  jobs,
  runningCount,
  queuedCount,
  concurrencyLimit,
  defaultPriority,
  cancellingJobIds,
  onSetConcurrencyLimit,
  onSetDefaultPriority,
  onUpdatePriority,
  onCancel,
}: Props) {
  const visibleJobs = jobs
    .filter((job) => job.status === "RUNNING" || job.status === "QUEUED")
    .slice(0, 6);

  return (
    <section className="queue-panel">
      <div className="queue-controls">
        <div>
          <span>Simultáneos</span>
          <strong>
            {runningCount}/{concurrencyLimit}
          </strong>
        </div>

        <label>
          Límite
          <input
            type="number"
            min={1}
            max={8}
            value={concurrencyLimit}
            onChange={(event) =>
              onSetConcurrencyLimit(Number(event.target.value))
            }
          />
        </label>

        <label>
          Prioridad nueva
          <select
            value={defaultPriority}
            onChange={(event) =>
              onSetDefaultPriority(event.target.value as JobPriority)
            }
          >
            {PRIORITIES.map((priority) => (
              <option key={priority.value} value={priority.value}>
                {priority.label}
              </option>
            ))}
          </select>
        </label>

        <div>
          <span>En cola</span>
          <strong>{queuedCount}</strong>
        </div>
      </div>

      {visibleJobs.length > 0 && (
        <div className="queue-list">
          {visibleJobs.map((job) => (
            <article key={job.id} className="queue-item">
              <div>
                <strong>{job.label}</strong>
                <span>{job.status}</span>
              </div>

              <select
                value={job.priority}
                disabled={job.status !== "QUEUED"}
                onChange={(event) =>
                  onUpdatePriority(job.id, event.target.value as JobPriority)
                }
                aria-label={`Prioridad de ${job.label}`}
              >
                {PRIORITIES.map((priority) => (
                  <option key={priority.value} value={priority.value}>
                    {priority.label}
                  </option>
                ))}
              </select>

              <button
                type="button"
                className="ghost-btn"
                onClick={() => onCancel(job.id)}
                disabled={cancellingJobIds.includes(job.id)}
              >
                {cancellingJobIds.includes(job.id) ? "Cancelando" : "Cancelar"}
              </button>
            </article>
          ))}
        </div>
      )}
    </section>
  );
}
