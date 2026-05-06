import { PipelineBuilder } from "../features/pipeline-builder/PipelineBuilder";
import { PipelineCard } from "../features/PipelineCard";
import type { RunCommand } from "../types";

type Props = {
  loading: boolean;
  configPath: string;
  setConfigPath: (value: string) => void;
  runCommand: RunCommand;
};

export function PipelinesPage({
  loading,
  configPath,
  setConfigPath,
  runCommand,
}: Props) {
  return (
    <section className="page-section">
      <div className="page-title">
        <span>🔗</span>
        <div>
          <h2>Pipelines JSON</h2>
          <p>
            Ejecuta automatizaciones existentes o crea una nueva desde el editor
            visual.
          </p>
        </div>
      </div>

      <div className="single-column wide-column">
        <PipelineBuilder
          loading={loading}
          runCommand={runCommand}
          setConfigPath={setConfigPath}
        />

        <PipelineCard
          loading={loading}
          configPath={configPath}
          setConfigPath={setConfigPath}
          runCommand={runCommand}
        />
      </div>
    </section>
  );
}
