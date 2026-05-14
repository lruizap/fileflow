import type {
  PipelineActionType,
  PipelineConfig,
  PipelineStepDraft,
} from "../../types";

export const ACTION_LABELS: Record<PipelineActionType, string> = {
  echo: "Comprobar funcionamiento",
  copy: "Copiar archivo",
  move: "Mover archivo",
  sync: "Sincronizar carpetas",
};

export function createStep(
  action: PipelineActionType = "sync",
): PipelineStepDraft {
  return {
    id: crypto.randomUUID(),
    action,
    src: "",
    dst: "",
    recursive: action === "sync",
    deleteExtra: false,
    overwrite: false,
    dryRun: false,
  };
}

export function stepToConfig(step: PipelineStepDraft) {
  if (step.action === "echo") {
    return { action: "echo", args: [] };
  }

  const args = ["--src", step.src, "--dst", step.dst];

  if (step.action === "sync" && step.recursive) args.push("--recursive");
  if (step.action === "sync" && step.deleteExtra) args.push("--delete-extra");
  if (step.action === "sync" && step.dryRun) args.push("--dry-run");
  if (step.overwrite) args.push("--overwrite");

  return {
    action: step.action,
    args,
  };
}

export function configStepToDraft(step: {
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
    dryRun: step.args.includes("--dry-run"),
  };
}

export function buildPipelineConfig(
  name: string,
  steps: PipelineStepDraft[],
): PipelineConfig {
  return {
    name: name.trim() || "pipeline",
    steps: steps.map(stepToConfig),
  };
}
