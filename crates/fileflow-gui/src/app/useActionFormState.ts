import { usePersistentState } from "./usePersistentState";

type FormState = {
  copySrc: string;
  copyDst: string;
  moveSrc: string;
  moveDst: string;
  syncSrc: string;
  syncDst: string;
  configPath: string;
  watchPath: string;
  recursive: boolean;
  deleteExtra: boolean;
  overwrite: boolean;
  dryRun: boolean;
  watchRecursive: boolean;
  watchDebounceMs: number;
};

const DEFAULT_FORM_STATE: FormState = {
  copySrc: "",
  copyDst: "",
  moveSrc: "",
  moveDst: "",
  syncSrc: "",
  syncDst: "",
  configPath: "",
  watchPath: "",
  recursive: true,
  deleteExtra: false,
  overwrite: false,
  dryRun: false,
  watchRecursive: true,
  watchDebounceMs: 500,
};

export function useActionFormState() {
  const [state, setState] = usePersistentState<FormState>(
    "fileflow.forms.v0.5.0",
    DEFAULT_FORM_STATE,
  );

  function update<K extends keyof FormState>(key: K, value: FormState[K]) {
    setState((prev) => ({ ...DEFAULT_FORM_STATE, ...prev, [key]: value }));
  }

  return {
    ...DEFAULT_FORM_STATE,
    ...state,

    setCopySrc: (value: string) => update("copySrc", value),
    setCopyDst: (value: string) => update("copyDst", value),
    setMoveSrc: (value: string) => update("moveSrc", value),
    setMoveDst: (value: string) => update("moveDst", value),
    setSyncSrc: (value: string) => update("syncSrc", value),
    setSyncDst: (value: string) => update("syncDst", value),
    setConfigPath: (value: string) => update("configPath", value),
    setWatchPath: (value: string) => update("watchPath", value),
    setRecursive: (value: boolean) => update("recursive", value),
    setDeleteExtra: (value: boolean) => update("deleteExtra", value),
    setOverwrite: (value: boolean) => update("overwrite", value),
    setDryRun: (value: boolean) => update("dryRun", value),
    setWatchRecursive: (value: boolean) => update("watchRecursive", value),
    setWatchDebounceMs: (value: number) => update("watchDebounceMs", value),
  };
}
