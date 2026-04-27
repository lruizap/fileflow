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

        <div className="help-section">
          <h3>Funciones principales</h3>

          <div className="help-list">
            <div>
              <strong>Comprobar funcionamiento</strong>
              <span>
                Ejecuta una prueba rápida para verificar que el motor interno de
                FileFlow responde correctamente.
              </span>
            </div>

            <div>
              <strong>Copiar archivo</strong>
              <span>
                Duplica un archivo en otra ubicación. El archivo original se
                conserva.
              </span>
            </div>

            <div>
              <strong>Mover archivo</strong>
              <span>
                Traslada un archivo a otra ubicación. El original desaparece de
                la carpeta inicial.
              </span>
            </div>

            <div>
              <strong>Sincronizar carpetas</strong>
              <span>
                Actualiza una carpeta destino con el contenido de una carpeta
                origen. Puede incluir subcarpetas, sobrescribir y borrar extras.
              </span>
            </div>

            <div>
              <strong>Automatización JSON</strong>
              <span>
                Permite ejecutar una secuencia de acciones guardadas en un
                archivo JSON reutilizable.
              </span>
            </div>

            <div>
              <strong>Historial y logs</strong>
              <span>
                El historial muestra qué acciones has ejecutado. Los logs
                muestran el detalle técnico de cada operación.
              </span>
            </div>

            <div>
              <strong>Alertas visuales</strong>
              <span>
                Después de cada ejecución verás una notificación indicando si la
                acción terminó correctamente o si hubo un error.
              </span>
            </div>

            <div>
              <strong>Limpiar logs</strong>
              <span>
                Permite limpiar el panel de registro para revisar mejor la
                siguiente ejecución.
              </span>
            </div>
          </div>
        </div>

        <div className="help-section">
          <h3>Cómo usarlo</h3>

          <ol className="help-steps">
            <li>Elige la acción que quieres ejecutar.</li>
            <li>Selecciona archivos o carpetas con los botones.</li>
            <li>Marca opciones como incluir subcarpetas o sobrescribir.</li>
            <li>Pulsa el botón principal de la acción.</li>
            <li>Revisa la alerta, el historial y los logs.</li>
          </ol>
        </div>

        <button className="primary-btn" onClick={onClose}>
          Entendido
        </button>
      </section>
    </div>
  );
}
