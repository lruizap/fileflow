import { useState } from "react";

export function GuidePage() {
  const [showPipelineExamples, setShowPipelineExamples] = useState(false);

  return (
    <section className="page-section">
      <div className="page-title">
        <span>📘</span>
        <div>
          <h2>Guía rápida de FileFlow</h2>
          <p>
            Aprende qué hace cada parte del programa y cómo usarlo sin tocar la
            terminal.
          </p>
        </div>
      </div>

      <section className="guide-grid">
        <article className="card info-card">
          <h2>1. Acciones rápidas</h2>
          <p>
            En la pantalla de Acciones puedes ejecutar tareas individuales. Es
            la forma más sencilla de usar FileFlow.
          </p>

          <ul className="guide-list">
            <li>
              <strong>Comprobar funcionamiento:</strong> prueba que el motor de
              FileFlow responde.
            </li>
            <li>
              <strong>Copiar archivo:</strong> crea una copia de un archivo sin
              borrar el original.
            </li>
            <li>
              <strong>Mover archivo:</strong> traslada un archivo y elimina el
              original.
            </li>
            <li>
              <strong>Sincronizar carpetas:</strong> actualiza una carpeta
              destino con el contenido de una carpeta origen.
            </li>
          </ul>
        </article>

        <article className="card info-card">
          <h2>2. Sincronización</h2>
          <p>
            La sincronización es útil para copias de seguridad, carpetas de
            trabajo o mantener dos ubicaciones actualizadas.
          </p>

          <ul className="guide-list">
            <li>
              <strong>Incluir subcarpetas:</strong> copia también carpetas
              internas.
            </li>
            <li>
              <strong>Sobrescribir:</strong> reemplaza archivos existentes.
            </li>
            <li>
              <strong>Borrar extras:</strong> elimina del destino archivos que
              ya no existen en origen.
            </li>
          </ul>
        </article>

        <article className="card info-card">
          <h2>3. Pipelines JSON</h2>
          <p>
            Un pipeline permite guardar varias acciones encadenadas en un
            archivo JSON. Así puedes reutilizar automatizaciones sin
            configurarlas cada vez.
          </p>

          <pre>{`{
  "name": "backup_docs",
  "steps": [
    {
      "action": "sync",
      "args": ["--src", "./docs", "--dst", "./backup", "--recursive"]
    }
  ]
}`}</pre>
        </article>

        <article className="card info-card">
          <h2>4. Actividad, logs y progreso</h2>
          <p>
            FileFlow muestra el historial de acciones, logs técnicos y una barra
            flotante de progreso cuando una operación tarda más de 30 segundos.
          </p>

          <ul className="guide-list">
            <li>Verás el proceso activo.</li>
            <li>Verás el archivo que se está tratando.</li>
            <li>Podrás cancelar operaciones largas.</li>
            <li>Recibirás alertas de éxito o error.</li>
          </ul>
        </article>

        <article className="card info-card guide-wide">
          <h2>5. Crear pipelines desde la interfaz</h2>
          <p>
            En la pantalla de Pipelines puedes crear automatizaciones sin
            escribir JSON a mano. Solo tienes que añadir pasos, elegir archivos
            o carpetas, guardar el JSON y ejecutarlo cuando quieras.
          </p>

          <ol className="help-steps">
            <li>Entra en la pantalla Pipelines.</li>
            <li>Pulsa añadir copiar, mover, sync o prueba.</li>
            <li>Configura cada paso con sus archivos o carpetas.</li>
            <li>Guarda el pipeline como archivo JSON.</li>
            <li>Ejecuta la automatización desde la propia app.</li>
          </ol>

          <button
            className="secondary-btn guide-examples-btn"
            type="button"
            onClick={() => setShowPipelineExamples(true)}
          >
            Ver ejemplos de pipelines
          </button>
        </article>

        <article className="card info-card guide-wide">
          <h2>6. Flujo recomendado</h2>
          <ol className="help-steps">
            <li>Entra en Acciones.</li>
            <li>Selecciona archivo o carpeta con los botones.</li>
            <li>Marca las opciones necesarias.</li>
            <li>Ejecuta la acción.</li>
            <li>Revisa la alerta, el historial y los logs.</li>
            <li>
              Si una operación tarda mucho, usa la barra flotante para seguir el
              progreso o cancelarla.
            </li>
          </ol>
        </article>
      </section>

      {showPipelineExamples && (
        <div className="modal-backdrop">
          <section className="modal pipeline-examples-modal">
            <button
              className="modal-close"
              type="button"
              onClick={() => setShowPipelineExamples(false)}
            >
              ×
            </button>

            <div className="modal-icon">🔗</div>

            <h2>Ejemplos de pipelines</h2>
            <p>
              Estos ejemplos muestran cómo FileFlow puede encadenar acciones
              para automatizar tareas repetitivas.
            </p>

            <div className="pipeline-example-list">
              <article>
                <h3>Ejemplo 1: Backup de documentos</h3>
                <p>
                  Sincroniza una carpeta de documentos con una carpeta de
                  backup, incluyendo subcarpetas.
                </p>

                <pre>{`{
  "name": "backup_documentos",
  "steps": [
    {
      "action": "sync",
      "args": [
        "--src",
        "C:/Users/Lucas/Documents",
        "--dst",
        "D:/Backups/Documents",
        "--recursive",
        "--overwrite"
      ]
    }
  ]
}`}</pre>
              </article>

              <article>
                <h3>Ejemplo 2: Ordenar archivo procesado</h3>
                <p>
                  Copia un archivo importante a una carpeta de backup y después
                  mueve el original a una carpeta de procesados.
                </p>

                <pre>{`{
  "name": "procesar_archivo",
  "steps": [
    {
      "action": "copy",
      "args": [
        "--src",
        "C:/Trabajo/informe.pdf",
        "--dst",
        "D:/Backup/informe.pdf",
        "--overwrite"
      ]
    },
    {
      "action": "move",
      "args": [
        "--src",
        "C:/Trabajo/informe.pdf",
        "--dst",
        "C:/Trabajo/procesados/informe.pdf",
        "--overwrite"
      ]
    }
  ]
}`}</pre>
              </article>
            </div>

            <button
              className="primary-btn"
              type="button"
              onClick={() => setShowPipelineExamples(false)}
            >
              Entendido
            </button>
          </section>
        </div>
      )}
    </section>
  );
}
