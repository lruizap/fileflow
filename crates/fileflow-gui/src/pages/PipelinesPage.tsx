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
            Ejecuta automatizaciones reutilizables guardadas en archivos JSON.
          </p>
        </div>
      </div>

      <div className="single-column">
        <PipelineCard
          loading={loading}
          configPath={configPath}
          setConfigPath={setConfigPath}
          runCommand={runCommand}
        />

        <article className="card info-card">
          <h2>¿Para qué sirve un pipeline?</h2>
          <p>
            Un pipeline permite encadenar varias acciones. Por ejemplo:
            sincronizar una carpeta, mover un archivo procesado y guardar una
            copia de seguridad.
          </p>

          <pre>{`{
  "name": "sync_demo",
  "steps": [
    {
      "action": "sync",
      "args": ["--src", "./docs", "--dst", "./backup", "--recursive"]
    }
  ]
}`}</pre>
        </article>
      </div>
    </section>
  );
}
