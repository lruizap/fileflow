import { useMemo, useState } from "react";

import {
  pickJsonFile,
  pickSaveJsonFile,
  readPipelineJson,
  savePipelineJson,
} from "../../api/fileflow";

import type {
  PipelineActionType,
  PipelineConfig,
  PipelineStepDraft,
  RunCommand,
} from "../../types";

import {
  buildPipelineConfig,
  configStepToDraft,
  createStep,
} from "./pipelineMapper";

import { PipelinePreview } from "./PipelinePreview";
import { PipelineStepCard } from "./PipelineStepCard";
import { PipelineToolbar } from "./PipelineToolbar";

type Props = {
  loading: boolean;
  runCommand: RunCommand;
  setConfigPath: (value: string) => void;
  onRememberPipeline: (path: string, name?: string) => void;
};

export function PipelineBuilder({
  loading,
  runCommand,
  setConfigPath,
  onRememberPipeline,
}: Props) {
  const [name, setName] = useState("mi_automatizacion");
  const [steps, setSteps] = useState<PipelineStepDraft[]>([createStep("sync")]);
  const [savedPath, setSavedPath] = useState("");

  const config = useMemo(() => buildPipelineConfig(name, steps), [name, steps]);

  const jsonPreview = useMemo(() => JSON.stringify(config, null, 2), [config]);

  function addStep(action: PipelineActionType) {
    setSteps((prev) => [...prev, createStep(action)]);
  }

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

  async function saveJson(): Promise<string | null> {
    const selected = savedPath || (await pickSaveJsonFile());
    if (!selected) return null;

    const finalPath = selected.endsWith(".json")
      ? selected
      : `${selected}.json`;

    await savePipelineJson(finalPath, jsonPreview);

    setSavedPath(finalPath);
    setConfigPath(finalPath);
    onRememberPipeline(finalPath, config.name);

    return finalPath;
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
    onRememberPipeline(selected, parsed.name);
  }

  async function executeCurrentPipeline() {
    const pathToRun = savedPath || (await saveJson());
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
            guarda el archivo y ejecútalo cuando quieras.
          </p>
        </div>
        <span className="badge">v0.5.0</span>
      </div>

      <label>
        Nombre de la automatización
        <input value={name} onChange={(e) => setName(e.target.value)} />
      </label>

      <PipelineToolbar onAddStep={addStep} />

      <div className="pipeline-steps">
        {steps.map((step, index) => (
          <PipelineStepCard
            key={step.id}
            step={step}
            index={index}
            onUpdate={updateStep}
            onRemove={removeStep}
            onMove={moveStep}
          />
        ))}
      </div>

      <PipelinePreview jsonPreview={jsonPreview} savedPath={savedPath} />

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
