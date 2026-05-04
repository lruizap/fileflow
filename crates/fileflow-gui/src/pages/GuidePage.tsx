export function GuidePage() {
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
            flotante de progreso cuando una operación tarda más de 10 segundos.
          </p>

          <ul className="guide-list">
            <li>Verás el proceso activo.</li>
            <li>Verás el archivo que se está tratando.</li>
            <li>Podrás cancelar operaciones largas.</li>
            <li>Recibirás alertas de éxito o error.</li>
          </ul>
        </article>

        <article className="card info-card guide-wide">
          <h2>5. Flujo recomendado</h2>
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
    </section>
  );
}
