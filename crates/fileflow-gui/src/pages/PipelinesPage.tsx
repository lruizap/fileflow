import { PipelineLibrary } from "../components/PipelineLibrary";
import { PipelineBuilder } from "../features/pipeline-builder/PipelineBuilder";
import { PipelineCard } from "../features/PipelineCard";
import type { RunCommand, SavedPipeline } from "../types";

type Props = {
  loading: boolean;
  configPath: string;
  pipelines: SavedPipeline[];
  setConfigPath: (value: string) => void;
  onRememberPipeline: (path: string, name?: string) => void;
  onForgetPipeline: (id: string) => void;
  runCommand: RunCommand;
};

export function PipelinesPage({
  loading,
  configPath,
  pipelines,
  setConfigPath,
  onRememberPipeline,
  onForgetPipeline,
  runCommand,
}: Props) {
  function selectPipeline(path: string) {
    setConfigPath(path);
    onRememberPipeline(path);
  }

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
          onRememberPipeline={onRememberPipeline}
        />

        <PipelineLibrary
          loading={loading}
          pipelines={pipelines}
          onSelect={selectPipeline}
          onForget={onForgetPipeline}
          runCommand={runCommand}
        />

        <PipelineCard
          loading={loading}
          configPath={configPath}
          setConfigPath={setConfigPath}
          onRememberPipeline={onRememberPipeline}
          runCommand={runCommand}
        />
      </div>
    </section>
  );
}
