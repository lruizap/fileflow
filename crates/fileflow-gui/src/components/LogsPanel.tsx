type Props = {
  logs: string[];
  onClear: () => void;
};

export function LogsPanel({ logs, onClear }: Props) {
  return (
    <article className="card logs-card">
      <div className="card-header">
        <div>
          <h2>Registro de ejecución</h2>
          <p>Aquí verás los pasos realizados y posibles errores.</p>
        </div>
        <span className="badge">logs</span>
      </div>

      <div className="terminal">
        {logs.map((log, index) => (
          <p key={`${log}-${index}`}>{log}</p>
        ))}
      </div>

      <button
        className="secondary-btn clear-btn"
        type="button"
        onClick={onClear}
      >
        Limpiar logs
      </button>
    </article>
  );
}
