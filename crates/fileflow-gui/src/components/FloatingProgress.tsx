import type { ProgressPayload } from "../types";

type Props = {
  progress: ProgressPayload | null;
  visible: boolean;
  onCancel: () => void;
  cancelling: boolean;
};

function formatSeconds(seconds: number | null) {
  if (seconds === null) return "Calculando...";

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
  progress,
  visible,
  onCancel,
  cancelling,
}: Props) {
  if (!visible || !progress) return null;

  const percent = Math.max(0, Math.min(100, progress.percent));

  return (
    <aside className="floating-progress">
      <div className="progress-top">
        <span className="progress-dot" />
        <strong>{progress.action}</strong>
      </div>

      <p className="progress-file">{progress.file}</p>

      <div className="progress-bar">
        <div style={{ width: `${percent}%` }} />
      </div>

      <div className="progress-meta">
        <span>{percent.toFixed(1)}%</span>
        <span>
          {formatBytes(progress.current)} / {formatBytes(progress.total)}
        </span>
      </div>

      <div className="progress-time">
        <span>Transcurrido: {formatSeconds(progress.elapsedSeconds)}</span>
        <span>Restante: {formatSeconds(progress.etaSeconds)}</span>
      </div>

      <button
        className="cancel-progress-btn"
        type="button"
        onClick={onCancel}
        disabled={cancelling}
      >
        {cancelling ? "Cancelando..." : "Cancelar operación"}
      </button>
    </aside>
  );
}
