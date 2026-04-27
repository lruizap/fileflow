import { ActionCard } from "../components/ActionCard";
import { PathInput } from "../components/PathInput";
import { pickJsonFile } from "../api/fileflow";
import type { RunCommand } from "../types";

type Props = {
  loading: boolean;
  configPath: string;
  setConfigPath: (value: string) => void;
  runCommand: RunCommand;
};

export function PipelineCard({
  loading,
  configPath,
  setConfigPath,
  runCommand,
}: Props) {
  async function selectJson() {
    const selected = await pickJsonFile();
    if (selected) setConfigPath(selected);
  }

  return (
    <ActionCard
      title="Pipeline JSON"
      description="Valida o ejecuta una automatización guardada."
      badge="json"
    >
      <PathInput
        label="Archivo JSON"
        value={configPath}
        onChange={setConfigPath}
        buttonText="Elegir JSON"
        onPick={selectJson}
      />

      <div className="button-row">
        <button
          className="secondary-btn"
          disabled={loading || !configPath}
          onClick={() => runCommand("validate_config", { path: configPath })}
        >
          Validar
        </button>

        <button
          className="primary-btn"
          disabled={loading || !configPath}
          onClick={() => runCommand("run_config", { path: configPath })}
        >
          Ejecutar
        </button>
      </div>
    </ActionCard>
  );
}
