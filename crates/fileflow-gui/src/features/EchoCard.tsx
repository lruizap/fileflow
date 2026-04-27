import { ActionCard } from "../components/ActionCard";
import type { RunCommand } from "../types";

type Props = {
  loading: boolean;
  runCommand: RunCommand;
};

export function EchoCard({ loading, runCommand }: Props) {
  return (
    <ActionCard
      title="Prueba rápida"
      description="Comprueba que el motor responde correctamente."
      badge="echo"
      className="action-card"
    >
      <button
        className="primary-btn"
        disabled={loading}
        onClick={() => runCommand("run_echo")}
      >
        {loading ? "Ejecutando..." : "Run Echo"}
      </button>
    </ActionCard>
  );
}
