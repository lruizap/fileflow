import { GitHubLink } from "./GitHubLink";

type Props = {
  onClose: () => void;
};

export function HelpModal({ onClose }: Props) {
  return (
    <div className="modal-backdrop">
      <section className="modal help-modal">
        <button className="modal-close" type="button" onClick={onClose}>
          ×
        </button>

        <div className="modal-icon">⚡</div>

        <h2>¿Qué es FileFlow?</h2>

        <p>
          FileFlow es una aplicación local para automatizar tareas de archivos:
          copiar, mover, sincronizar carpetas y ejecutar automatizaciones
          guardadas en JSON. Está pensada para ahorrar tiempo en tareas
          repetitivas sin depender de la nube.
        </p>

        <div className="github-popup">
          <div>
            <strong>Repositorio oficial</strong>
            <span>
              Revisa actualizaciones, releases y documentación del proyecto.
            </span>
          </div>
          <GitHubLink compact />
        </div>

        <div className="help-section">
          <h3>Funciones principales</h3>

          <div className="help-list">
            <div>
              <strong>Acciones rápidas</strong>
              <span>Copy, move, sync y prueba rápida del motor.</span>
            </div>

            <div>
              <strong>Pipelines JSON</strong>
              <span>
                Automatizaciones reutilizables guardadas en archivos JSON.
              </span>
            </div>

            <div>
              <strong>Actividad</strong>
              <span>Historial, logs, progreso y cancelación de procesos.</span>
            </div>

            <div>
              <strong>GitHub oficial</strong>
              <span>Acceso rápido para descargar o actualizar versiones.</span>
            </div>
          </div>
        </div>

        <button className="primary-btn" onClick={onClose}>
          Entendido
        </button>
      </section>
    </div>
  );
}
