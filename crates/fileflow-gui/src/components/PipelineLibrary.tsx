import type { RunCommand, SavedPipeline } from "../types";

type Props = {
  loading: boolean;
  pipelines: SavedPipeline[];
  onSelect: (path: string) => void;
  onForget: (id: string) => void;
  runCommand: RunCommand;
};

export function PipelineLibrary({
  loading,
  pipelines,
  onSelect,
  onForget,
  runCommand,
}: Props) {
  return (
    <article className="card pipeline-library-card">
      <div className="card-header compact-header">
        <div>
          <h2>Biblioteca de automatizaciones</h2>
          <p>Pipelines guardados y recientes para ejecutarlos sin buscarlos.</p>
        </div>
        <span className="badge">v0.5.0</span>
      </div>

      {pipelines.length === 0 ? (
        <div className="empty-state compact-empty">
          Guarda o carga un pipeline para añadirlo a la biblioteca.
        </div>
      ) : (
        <div className="pipeline-library-list">
          {pipelines.map((pipeline) => (
            <div className="pipeline-library-item" key={pipeline.id}>
              <button
                className="pipeline-library-main"
                type="button"
                onClick={() => onSelect(pipeline.path)}
              >
                <strong>{pipeline.name}</strong>
                <span>{pipeline.path}</span>
                <small>{pipeline.updatedAt}</small>
              </button>

              <div className="pipeline-library-actions">
                <button
                  className="secondary-btn"
                  type="button"
                  disabled={loading}
                  onClick={() =>
                    runCommand(
                      "validate_config",
                      { path: pipeline.path },
                      `Validar ${pipeline.name}`,
                    )
                  }
                >
                  Validar
                </button>

                <button
                  className="primary-btn"
                  type="button"
                  disabled={loading}
                  onClick={() =>
                    runCommand(
                      "run_config",
                      { path: pipeline.path },
                      `Ejecutar ${pipeline.name}`,
                    )
                  }
                >
                  Ejecutar
                </button>

                <button
                  className="ghost-btn"
                  type="button"
                  onClick={() => onForget(pipeline.id)}
                >
                  Quitar
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </article>
  );
}

