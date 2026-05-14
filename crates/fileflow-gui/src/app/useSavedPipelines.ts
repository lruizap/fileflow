import type { SavedPipeline } from "../types";
import { usePersistentState } from "./usePersistentState";

function pipelineId(path: string) {
  return path.trim().toLowerCase();
}

function pathName(path: string) {
  return path.split(/[\\/]/).pop()?.replace(/\.json$/i, "") || "pipeline";
}

export function useSavedPipelines() {
  const [pipelines, setPipelines] = usePersistentState<SavedPipeline[]>(
    "fileflow.savedPipelines.v0.5.0",
    [],
  );

  function rememberPipeline(path: string, name = pathName(path)) {
    const trimmedPath = path.trim();
    if (!trimmedPath) return;

    const nextPipeline: SavedPipeline = {
      id: pipelineId(trimmedPath),
      name: name.trim() || pathName(trimmedPath),
      path: trimmedPath,
      updatedAt: new Date().toLocaleString(),
    };

    setPipelines((prev) => [
      nextPipeline,
      ...prev.filter((item) => item.id !== nextPipeline.id),
    ]);
  }

  function forgetPipeline(id: string) {
    setPipelines((prev) => prev.filter((item) => item.id !== id));
  }

  return {
    pipelines,
    rememberPipeline,
    forgetPipeline,
  };
}

