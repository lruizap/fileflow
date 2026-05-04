import { HistoryPanel } from "../components/HistoryPanel";
import { LogsPanel } from "../components/LogsPanel";
import type { HistoryItem } from "../types";

type Props = {
  logs: string[];
  history: HistoryItem[];
  onClearLogs: () => void;
  onClearHistory: () => void;
};

export function ActivityPage({
  logs,
  history,
  onClearLogs,
  onClearHistory,
}: Props) {
  return (
    <section className="page-section">
      <div className="page-title">
        <span>📊</span>
        <div>
          <h2>Actividad</h2>
          <p>
            Consulta las últimas acciones ejecutadas y los registros técnicos.
          </p>
        </div>
      </div>

      <section className="grid">
        <HistoryPanel history={history} onClear={onClearHistory} />
        <LogsPanel logs={logs} onClear={onClearLogs} />
      </section>
    </section>
  );
}
