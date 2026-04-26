import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import "./App.css";

type GuiRunResult = {
  status: string;
  logs: string[];
};

function App() {
  const [showIntro, setShowIntro] = useState(true);

  const [copySrc, setCopySrc] = useState("");
  const [copyDst, setCopyDst] = useState("");
  const [moveSrc, setMoveSrc] = useState("");
  const [moveDst, setMoveDst] = useState("");

  const [syncSrc, setSyncSrc] = useState("");
  const [syncDst, setSyncDst] = useState("");
  const [configPath, setConfigPath] = useState("");

  const [recursive, setRecursive] = useState(true);
  const [deleteExtra, setDeleteExtra] = useState(false);
  const [overwrite, setOverwrite] = useState(false);

  const [status, setStatus] = useState("READY");
  const [logs, setLogs] = useState<string[]>([
    "FileFlow GUI inicializada.",
    "Selecciona una acción para comenzar.",
  ]);
  const [loading, setLoading] = useState(false);

  async function runCommand<T extends Record<string, unknown>>(
    command: string,
    args?: T,
  ) {
    setLoading(true);
    setStatus("RUNNING");
    setLogs((prev) => [`Ejecutando: ${command}`, ...prev]);

    try {
      const result = await invoke<GuiRunResult>(command, args ?? {});
      setStatus(result.status);
      setLogs(result.logs);
    } catch (err) {
      setStatus("ERROR");
      setLogs([
        "Error ejecutando comando:",
        typeof err === "string" ? err : JSON.stringify(err, null, 2),
      ]);
    } finally {
      setLoading(false);
    }
  }

  async function pickFile(setter: (value: string) => void) {
    const selected = await open({
      multiple: false,
      directory: false,
    });

    if (typeof selected === "string") {
      setter(selected);
    }
  }

  async function pickJsonFile() {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: "Pipeline JSON",
          extensions: ["json"],
        },
      ],
    });

    if (typeof selected === "string") {
      setConfigPath(selected);
    }
  }

  async function pickDirectory(setter: (value: string) => void) {
    const selected = await open({
      multiple: false,
      directory: true,
    });

    if (typeof selected === "string") {
      setter(selected);
    }
  }

  async function pickDestinationFile(setter: (value: string) => void) {
    const selected = await save({
      filters: [
        {
          name: "Todos los archivos",
          extensions: ["*"],
        },
      ],
    });

    if (typeof selected === "string") {
      setter(selected);
    }
  }
  const statusClass = status.includes("SUCCESS")
    ? "success"
    : status.includes("FAILED") || status.includes("ERROR")
      ? "error"
      : status.includes("RUNNING")
        ? "running"
        : "ready";

  return (
    <main className="app">
      {showIntro && (
        <div className="modal-backdrop">
          <section className="modal help-modal">
            <button
              className="modal-close"
              type="button"
              onClick={() => setShowIntro(false)}
            >
              ×
            </button>

            <div className="modal-icon">⚡</div>

            <h2>¿Qué es FileFlow?</h2>

            <p>
              FileFlow es una herramienta local para automatizar tareas de
              archivos: copiar, mover, sincronizar carpetas y ejecutar procesos
              guardados en JSON. La idea es que puedas hacer operaciones
              repetitivas sin escribir comandos largos cada vez.
            </p>

            <div className="help-section">
              <h3>Funciones principales</h3>

              <div className="help-list">
                <div>
                  <strong>Echo</strong>
                  <span>
                    Acción de prueba. Sirve para comprobar que el motor interno
                    de FileFlow funciona correctamente.
                  </span>
                </div>

                <div>
                  <strong>Copy</strong>
                  <span>
                    Copia un archivo desde una ruta origen a una ruta destino.
                    Puedes usar “Sobrescribir destino” si el archivo ya existe.
                  </span>
                </div>

                <div>
                  <strong>Move</strong>
                  <span>
                    Mueve un archivo desde una ubicación a otra. Es útil para
                    ordenar descargas, documentos o archivos procesados.
                  </span>
                </div>

                <div>
                  <strong>Sync</strong>
                  <span>
                    Sincroniza una carpeta origen con una carpeta destino. Puede
                    copiar subcarpetas si activas “Recursivo” y eliminar
                    archivos extra si activas “Borrar extras”.
                  </span>
                </div>

                <div>
                  <strong>Pipeline JSON</strong>
                  <span>
                    Ejecuta una automatización guardada en un archivo JSON. Un
                    pipeline puede encadenar varias acciones como sync, move o
                    copy.
                  </span>
                </div>

                <div>
                  <strong>Logs</strong>
                  <span>
                    Muestra el resultado de cada ejecución: estado final, pasos
                    realizados y posibles errores.
                  </span>
                </div>
              </div>
            </div>

            <div className="help-section">
              <h3>Cómo usarlo</h3>

              <ol className="help-steps">
                <li>Elige la acción que quieres ejecutar.</li>
                <li>
                  Selecciona archivos o carpetas con los botones laterales.
                </li>
                <li>
                  Marca opciones como recursivo, sobrescribir o borrar extras.
                </li>
                <li>Pulsa ejecutar.</li>
                <li>Revisa el resultado en el panel de logs.</li>
              </ol>
            </div>

            <button className="primary-btn" onClick={() => setShowIntro(false)}>
              Entendido
            </button>
          </section>
        </div>
      )}

      <section className="hero">
        <div>
          <p className="eyebrow">FileFlow GUI</p>
          <h1>Automatización local de archivos</h1>
          <p className="subtitle">
            Elige archivos y carpetas, ejecuta acciones y controla el resultado
            desde una interfaz visual.
          </p>
        </div>

        <div className={`status-pill ${statusClass}`}>
          <span></span>
          {status}
        </div>
      </section>

      <section className="grid">
        <article className="card action-card">
          <div className="card-header">
            <div>
              <h2>Prueba rápida</h2>
              <p>Comprueba que el motor responde correctamente.</p>
            </div>
            <span className="badge">echo</span>
          </div>

          <button
            className="primary-btn"
            disabled={loading}
            onClick={() => runCommand("run_echo")}
          >
            {loading ? "Ejecutando..." : "Run Echo"}
          </button>
        </article>

        <article className="card">
          <div className="card-header">
            <div>
              <h2>Copiar archivo</h2>
              <p>Selecciona un archivo origen y una ruta destino.</p>
            </div>
            <span className="badge">copy</span>
          </div>

          <PathInput
            label="Archivo origen"
            value={copySrc}
            onChange={setCopySrc}
            buttonText="Elegir archivo"
            onPick={() => pickFile(setCopySrc)}
          />

          <PathInput
            label="Archivo destino"
            value={copyDst}
            onChange={setCopyDst}
            buttonText="Guardar como"
            onPick={() => pickDestinationFile(setCopyDst)}
          />

          <label className="check solo">
            <input
              type="checkbox"
              checked={overwrite}
              onChange={(e) => setOverwrite(e.target.checked)}
            />
            Sobrescribir destino
          </label>

          <button
            className="primary-btn"
            disabled={loading || !copySrc || !copyDst}
            onClick={() =>
              runCommand("run_copy", {
                src: copySrc,
                dst: copyDst,
                overwrite,
              })
            }
          >
            Ejecutar Copy
          </button>
        </article>

        <article className="card">
          <div className="card-header">
            <div>
              <h2>Mover archivo</h2>
              <p>Mueve un archivo a otra ubicación.</p>
            </div>
            <span className="badge">move</span>
          </div>

          <PathInput
            label="Archivo origen"
            value={moveSrc}
            onChange={setMoveSrc}
            buttonText="Elegir archivo"
            onPick={() => pickFile(setMoveSrc)}
          />

          <PathInput
            label="Archivo destino"
            value={moveDst}
            onChange={setMoveDst}
            buttonText="Guardar como"
            onPick={() => pickDestinationFile(setMoveDst)}
          />

          <label className="check solo">
            <input
              type="checkbox"
              checked={overwrite}
              onChange={(e) => setOverwrite(e.target.checked)}
            />
            Sobrescribir destino
          </label>

          <button
            className="primary-btn"
            disabled={loading || !moveSrc || !moveDst}
            onClick={() =>
              runCommand("run_move", {
                src: moveSrc,
                dst: moveDst,
                overwrite,
              })
            }
          >
            Ejecutar Move
          </button>
        </article>

        <article className="card">
          <div className="card-header">
            <div>
              <h2>Sincronizar carpetas</h2>
              <p>Copia archivos desde origen a destino usando sync.</p>
            </div>
            <span className="badge">sync</span>
          </div>

          <PathInput
            label="Carpeta origen"
            value={syncSrc}
            onChange={setSyncSrc}
            buttonText="Elegir carpeta"
            onPick={() => pickDirectory(setSyncSrc)}
          />

          <PathInput
            label="Carpeta destino"
            value={syncDst}
            onChange={setSyncDst}
            buttonText="Elegir carpeta"
            onPick={() => pickDirectory(setSyncDst)}
          />

          <div className="options">
            <label className="check">
              <input
                type="checkbox"
                checked={recursive}
                onChange={(e) => setRecursive(e.target.checked)}
              />
              Recursivo
            </label>

            <label className="check">
              <input
                type="checkbox"
                checked={deleteExtra}
                onChange={(e) => setDeleteExtra(e.target.checked)}
              />
              Borrar extras
            </label>

            <label className="check">
              <input
                type="checkbox"
                checked={overwrite}
                onChange={(e) => setOverwrite(e.target.checked)}
              />
              Sobrescribir
            </label>
          </div>

          <button
            className="primary-btn"
            disabled={loading || !syncSrc || !syncDst}
            onClick={() =>
              runCommand("run_sync", {
                src: syncSrc,
                dst: syncDst,
                recursive,
                delete_extra: deleteExtra,
                overwrite,
              })
            }
          >
            Ejecutar Sync
          </button>
        </article>

        <article className="card">
          <div className="card-header">
            <div>
              <h2>Pipeline JSON</h2>
              <p>Valida o ejecuta una automatización guardada.</p>
            </div>
            <span className="badge">json</span>
          </div>

          <PathInput
            label="Archivo JSON"
            value={configPath}
            onChange={setConfigPath}
            buttonText="Elegir JSON"
            onPick={pickJsonFile}
          />

          <div className="button-row">
            <button
              className="secondary-btn"
              disabled={loading || !configPath}
              onClick={() =>
                runCommand("validate_config", { path: configPath })
              }
            >
              Validar
            </button>

            <button
              className="primary-btn"
              disabled={loading || !configPath}
              onClick={() => runCommand("run_config", { path: configPath })}
            >
              Ejecutar
            </button>
          </div>
        </article>

        <article className="card logs-card">
          <div className="card-header">
            <div>
              <h2>Logs</h2>
              <p>Resultado de la última ejecución.</p>
            </div>
            <span className="badge">engine</span>
          </div>

          <div className="terminal">
            {logs.map((log, index) => (
              <p key={`${log}-${index}`}>{log}</p>
            ))}
          </div>
        </article>
      </section>

      <button
        className="floating-help-btn"
        type="button"
        onClick={() => setShowIntro(true)}
        title="Abrir ayuda"
      >
        ?
      </button>
    </main>
  );
}

type PathInputProps = {
  label: string;
  value: string;
  buttonText: string;
  onChange: (value: string) => void;
  onPick: () => void;
};

function PathInput({
  label,
  value,
  buttonText,
  onChange,
  onPick,
}: PathInputProps) {
  return (
    <label>
      {label}
      <div className="path-row">
        <input value={value} onChange={(e) => onChange(e.target.value)} />
        <button type="button" className="small-btn" onClick={onPick}>
          {buttonText}
        </button>
      </div>
    </label>
  );
}

export default App;
