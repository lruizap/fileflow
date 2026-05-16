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
            FileFlow muestra el historial, logs técnicos y una barra flotante
            independiente por cada proceso activo o en cola.
          </p>

          <ul className="guide-list">
            <li>Verás cada proceso con su propio porcentaje.</li>
            <li>Verás el archivo o paso que se está tratando.</li>
            <li>Podrás cancelar cada proceso por separado.</li>
            <li>Recibirás alertas de éxito o error.</li>
          </ul>
        </article>

        <article className="card info-card guide-wide">
          <h2>5. Cola avanzada y procesos simultáneos</h2>
          <p>
            La versión 0.6.0 permite ejecutar varios procesos a la vez. El
            límite simultáneo es editable desde el panel superior: por defecto
            son 2 procesos y el máximo permitido es 8.
          </p>

          <ul className="guide-list">
            <li>
              <strong>Límite:</strong> controla cuántos trabajos pueden correr
              al mismo tiempo.
            </li>
            <li>
              <strong>Prioridad nueva:</strong> define la prioridad con la que
              se encolan las próximas acciones.
            </li>
            <li>
              <strong>Prioridad por job:</strong> los trabajos en cola pueden
              cambiar entre baja, normal, alta y crítica antes de arrancar.
            </li>
            <li>
              <strong>Cola:</strong> cuando se alcanza el límite, los nuevos
              procesos esperan y arrancan automáticamente según prioridad.
            </li>
          </ul>
        </article>

        <article className="card info-card guide-wide">
          <h2>6. Crear y guardar pipelines</h2>
          <p>
            En la pantalla de Pipelines puedes crear automatizaciones sin
            escribir JSON a mano. Solo tienes que añadir pasos, elegir archivos
            o carpetas, guardar el JSON y ejecutarlo cuando quieras desde la
            biblioteca.
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
          <h2>7. Vigilar carpetas</h2>
          <p>
            La pantalla Vigilar ejecuta un pipeline cuando detecta cambios en
            una carpeta. Es útil para procesar descargas, backups o bandejas de
            entrada locales.
          </p>

          <ol className="help-steps">
            <li>Elige la carpeta a vigilar.</li>
            <li>Selecciona el pipeline JSON que quieres ejecutar.</li>
            <li>Ajusta subcarpetas y debounce si la carpeta cambia mucho.</li>
            <li>Inicia la vigilancia y detenla desde la misma pantalla.</li>
          </ol>
        </article>

        <article className="card info-card guide-wide">
          <h2>8. Flujo recomendado</h2>
          <ol className="help-steps">
            <li>Entra en Acciones.</li>
            <li>Selecciona archivo o carpeta con los botones.</li>
            <li>Marca las opciones necesarias.</li>
            <li>Ejecuta la acción.</li>
            <li>Revisa la alerta, el historial y los logs.</li>
            <li>
              Si hay operaciones largas, usa sus barras flotantes para seguir el
              progreso o cancelar solo el proceso necesario.
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
