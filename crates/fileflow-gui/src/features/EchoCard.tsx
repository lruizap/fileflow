import { ActionCard } from "../components/ActionCard";
import type { RunCommand } from "../types";

type Props = {
  loading: boolean;
  runCommand: RunCommand;
};

export function EchoCard({ loading, runCommand }: Props) {
  return (
    <ActionCard
      title="Comprobar funcionamiento"
      description="Ejecuta una prueba rápida para verificar que FileFlow responde correctamente."
      badge="test"
      className="action-card"
    >
      <button
        className="primary-btn"
        disabled={loading}
        onClick={() => runCommand("run_echo", undefined, "Prueba rápida")}
      >
        {loading ? "Ejecutando..." : "Ejecutar prueba"}
      </button>
    </ActionCard>
  );
}
