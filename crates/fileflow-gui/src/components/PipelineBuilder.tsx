import { useMemo, useState } from "react";
import {
  pickDirectory,
  pickFile,
  pickJsonFile,
  pickSaveJsonFile,
  readPipelineJson,
  savePipelineJson,
} from "../api/fileflow";
import type {
  PipelineActionType,
  PipelineConfig,
  PipelineStepDraft,
  RunCommand,
} from "../types";

type Props = {
  loading: boolean;
  runCommand: RunCommand;
  setConfigPath: (value: string) => void;
};

const ACTION_LABELS: Record<PipelineActionType, string> = {
  echo: "Comprobar funcionamiento",
  copy: "Copiar archivo",
  move: "Mover archivo",
  sync: "Sincronizar carpetas",
};

function createStep(action: PipelineActionType = "sync"): PipelineStepDraft {
  return {
    id: crypto.randomUUID(),
    action,
    src: "",
    dst: "",
    recursive: action === "sync",
    deleteExtra: false,
    overwrite: false,
  };
}

function stepToConfig(step: PipelineStepDraft) {
  if (step.action === "echo") {
    return { action: "echo", args: [] };
  }

  const args = ["--src", step.src, "--dst", step.dst];

  if (step.action === "sync" && step.recursive) args.push("--recursive");
  if (step.action === "sync" && step.deleteExtra) args.push("--delete-extra");
  if (step.overwrite) args.push("--overwrite");

  return {
    action: step.action,
    args,
  };
}

function configStepToDraft(step: {
  action: string;
  args: string[];
}): PipelineStepDraft {
  const action = ["echo", "copy", "move", "sync"].includes(step.action)
    ? (step.action as PipelineActionType)
    : "echo";

  const getValue = (flag: string) => {
    const index = step.args.indexOf(flag);
    return index >= 0 ? (step.args[index + 1] ?? "") : "";
  };

  return {
    id: crypto.randomUUID(),
    action,
    src: getValue("--src"),
    dst: getValue("--dst"),
    recursive: step.args.includes("--recursive"),
    deleteExtra: step.args.includes("--delete-extra"),
    overwrite: step.args.includes("--overwrite"),
  };
}

export function PipelineBuilder({ loading, runCommand, setConfigPath }: Props) {
  const [name, setName] = useState("mi_automatizacion");
  const [steps, setSteps] = useState<PipelineStepDraft[]>([createStep("sync")]);
  const [savedPath, setSavedPath] = useState("");

  const config: PipelineConfig = useMemo(
    () => ({
      name: name.trim() || "pipeline",
      steps: steps.map(stepToConfig),
    }),
    [name, steps],
  );

  const jsonPreview = JSON.stringify(config, null, 2);

  function updateStep(id: string, patch: Partial<PipelineStepDraft>) {
    setSteps((prev) =>
      prev.map((step) => (step.id === id ? { ...step, ...patch } : step)),
    );
  }

  function removeStep(id: string) {
    setSteps((prev) => prev.filter((step) => step.id !== id));
  }

  function moveStep(id: string, direction: "up" | "down") {
    setSteps((prev) => {
      const index = prev.findIndex((step) => step.id === id);
      if (index < 0) return prev;

      const target = direction === "up" ? index - 1 : index + 1;
      if (target < 0 || target >= prev.length) return prev;

      const next = [...prev];
      const [item] = next.splice(index, 1);
      next.splice(target, 0, item);
      return next;
    });
  }

  async function saveJson() {
    const selected = savedPath || (await pickSaveJsonFile());
    if (!selected) return;

    const finalPath = selected.endsWith(".json")
      ? selected
      : `${selected}.json`;

    await savePipelineJson(finalPath, jsonPreview);
    setSavedPath(finalPath);
    setConfigPath(finalPath);
  }

  async function loadJson() {
    const selected = await pickJsonFile();
    if (!selected) return;

    const raw = await readPipelineJson(selected);
    const parsed = JSON.parse(raw) as PipelineConfig;

    setName(parsed.name || "pipeline");
    setSteps(parsed.steps.map(configStepToDraft));
    setSavedPath(selected);
    setConfigPath(selected);
  }

  async function executeCurrentPipeline() {
    if (!savedPath) {
      await saveJson();
    }

    const pathToRun = savedPath;
    if (!pathToRun) return;

    await runCommand(
      "run_config",
      { path: pathToRun },
      "Ejecutar automatización visual",
    );
  }

  return (
    <article className="card pipeline-builder-card">
      <div className="card-header">
        <div>
          <h2>Editor visual de pipelines</h2>
          <p>
            Crea automatizaciones sin escribir JSON manualmente. Añade pasos,
            guarda el archivo y ejecútalo.
          </p>
        </div>
        <span className="badge">v0.4.0</span>
      </div>

      <label>
        Nombre de la automatización
        <input value={name} onChange={(e) => setName(e.target.value)} />
      </label>

      <div className="pipeline-toolbar">
        <button
          className="secondary-btn"
          type="button"
          onClick={() => setSteps((prev) => [...prev, createStep("copy")])}
        >
          Añadir copiar
        </button>

        <button
          className="secondary-btn"
          type="button"
          onClick={() => setSteps((prev) => [...prev, createStep("move")])}
        >
          Añadir mover
        </button>

        <button
          className="secondary-btn"
          type="button"
          onClick={() => setSteps((prev) => [...prev, createStep("sync")])}
        >
          Añadir sync
        </button>

        <button
          className="secondary-btn"
          type="button"
          onClick={() => setSteps((prev) => [...prev, createStep("echo")])}
        >
          Añadir prueba
        </button>
      </div>

      <div className="pipeline-steps">
        {steps.map((step, index) => (
          <div className="pipeline-step" key={step.id}>
            <div className="pipeline-step-header">
              <strong>
                Paso {index + 1}: {ACTION_LABELS[step.action]}
              </strong>

              <div className="step-buttons">
                <button type="button" onClick={() => moveStep(step.id, "up")}>
                  ↑
                </button>
                <button type="button" onClick={() => moveStep(step.id, "down")}>
                  ↓
                </button>
                <button type="button" onClick={() => removeStep(step.id)}>
                  Eliminar
                </button>
              </div>
            </div>

            <label>
              Tipo de acción
              <select
                value={step.action}
                onChange={(e) =>
                  updateStep(step.id, {
                    action: e.target.value as PipelineActionType,
                  })
                }
              >
                <option value="echo">Comprobar funcionamiento</option>
                <option value="copy">Copiar archivo</option>
                <option value="move">Mover archivo</option>
                <option value="sync">Sincronizar carpetas</option>
              </select>
            </label>

            {step.action !== "echo" && (
              <>
                <PathSelector
                  label={
                    step.action === "sync" ? "Carpeta origen" : "Archivo origen"
                  }
                  value={step.src}
                  onChange={(value) => updateStep(step.id, { src: value })}
                  mode={step.action === "sync" ? "directory" : "file"}
                />

                <PathSelector
                  label={
                    step.action === "sync"
                      ? "Carpeta destino"
                      : "Archivo destino"
                  }
                  value={step.dst}
                  onChange={(value) => updateStep(step.id, { dst: value })}
                  mode={step.action === "sync" ? "directory" : "file"}
                />

                <div className="options">
                  {step.action === "sync" && (
                    <>
                      <label className="check">
                        <input
                          type="checkbox"
                          checked={step.recursive}
                          onChange={(e) =>
                            updateStep(step.id, { recursive: e.target.checked })
                          }
                        />
                        Incluir subcarpetas
                      </label>

                      <label className="check">
                        <input
                          type="checkbox"
                          checked={step.deleteExtra}
                          onChange={(e) =>
                            updateStep(step.id, {
                              deleteExtra: e.target.checked,
                            })
                          }
                        />
                        Borrar extras
                      </label>
                    </>
                  )}

                  <label className="check">
                    <input
                      type="checkbox"
                      checked={step.overwrite}
                      onChange={(e) =>
                        updateStep(step.id, { overwrite: e.target.checked })
                      }
                    />
                    Sobrescribir
                  </label>
                </div>
              </>
            )}
          </div>
        ))}
      </div>

      <div className="pipeline-preview">
        <div className="pipeline-preview-header">
          <strong>JSON generado</strong>
          {savedPath && <span>{savedPath}</span>}
        </div>
        <pre>{jsonPreview}</pre>
      </div>

      <div className="button-row pipeline-actions">
        <button className="secondary-btn" type="button" onClick={loadJson}>
          Cargar JSON
        </button>

        <button className="secondary-btn" type="button" onClick={saveJson}>
          Guardar JSON
        </button>

        <button
          className="primary-btn"
          type="button"
          disabled={loading || steps.length === 0}
          onClick={executeCurrentPipeline}
        >
          Ejecutar pipeline
        </button>
      </div>
    </article>
  );
}

type PathSelectorProps = {
  label: string;
  value: string;
  mode: "file" | "directory";
  onChange: (value: string) => void;
};

function PathSelector({ label, value, mode, onChange }: PathSelectorProps) {
  async function selectPath() {
    const selected =
      mode === "directory" ? await pickDirectory() : await pickFile();
    if (selected) onChange(selected);
  }

  return (
    <label>
      {label}
      <div className="path-row">
        <input value={value} onChange={(e) => onChange(e.target.value)} />
        <button type="button" className="small-btn" onClick={selectPath}>
          Elegir
        </button>
      </div>
    </label>
  );
}
