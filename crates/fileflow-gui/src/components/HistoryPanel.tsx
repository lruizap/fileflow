import type { HistoryItem } from "../types";

type Props = {
  history: HistoryItem[];
  onClear: () => void;
};

export function HistoryPanel({ history, onClear }: Props) {
  return (
    <article className="card history-card compact-history-card">
      <div className="card-header compact-header">
        <div>
          <h2>Historial</h2>
          <p>Últimas acciones de esta sesión.</p>
        </div>
        <span className="badge">actividad</span>
      </div>

      {history.length === 0 ? (
        <div className="empty-state compact-empty">
          Todavía no has ejecutado ninguna acción.
        </div>
      ) : (
        <>
          <div className="history-list compact-history-list">
            {history.map((item) => (
              <div key={item.id} className="history-item">
                <div>
                  <strong>{item.label}</strong>
                  <span>{item.createdAt}</span>
                </div>
                <em className={item.status.includes("SUCCESS") ? "ok" : "bad"}>
                  {item.status}
                </em>
              </div>
            ))}
          </div>

          <button className="secondary-btn" type="button" onClick={onClear}>
            Limpiar historial
          </button>
        </>
      )}
    </article>
  );
}
