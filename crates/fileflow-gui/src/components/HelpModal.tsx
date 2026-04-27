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
          FileFlow es una herramienta local para automatizar tareas de archivos:
          copiar, mover, sincronizar carpetas y ejecutar procesos guardados en
          JSON.
        </p>

        <div className="help-section">
          <h3>Funciones principales</h3>

          <div className="help-list">
            <div>
              <strong>Echo</strong>
              <span>
                Acción de prueba para comprobar que el motor responde.
              </span>
            </div>

            <div>
              <strong>Copy</strong>
              <span>
                Copia un archivo desde una ruta origen a una ruta destino.
              </span>
            </div>

            <div>
              <strong>Move</strong>
              <span>Mueve un archivo desde una ubicación a otra.</span>
            </div>

            <div>
              <strong>Sync</strong>
              <span>
                Sincroniza carpetas. Puede funcionar en modo recursivo y borrar
                archivos extra.
              </span>
            </div>

            <div>
              <strong>Pipeline JSON</strong>
              <span>
                Ejecuta una automatización guardada en un archivo JSON.
              </span>
            </div>

            <div>
              <strong>Logs</strong>
              <span>Muestra pasos, resultado final y posibles errores.</span>
            </div>
          </div>
        </div>

        <div className="help-section">
          <h3>Cómo usarlo</h3>

          <ol className="help-steps">
            <li>Elige la acción que quieres ejecutar.</li>
            <li>Selecciona archivos o carpetas con los botones.</li>
            <li>Marca opciones como recursivo o sobrescribir.</li>
            <li>Pulsa ejecutar.</li>
            <li>Revisa el resultado en logs.</li>
          </ol>
        </div>

        <button className="primary-btn" onClick={onClose}>
          Entendido
        </button>
      </section>
    </div>
  );
}
