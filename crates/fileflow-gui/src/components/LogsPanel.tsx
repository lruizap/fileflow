type Props = {
  logs: string[];
};

export function LogsPanel({ logs }: Props) {
  return (
    <article className="card logs-card">
      <div className="card-header">
        <div>
          <h2>Logs</h2>
          <p>Resultado de la última ejecución.</p>
        </div>
        <span className="badge">engine</span>
      </div>

      <div className="terminal">
        {logs.map((log, index) => (
          <p key={`${log}-${index}`}>{log}</p>
        ))}
      </div>
    </article>
  );
}
