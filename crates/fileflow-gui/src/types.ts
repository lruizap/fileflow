export type GuiRunResult = {
  status: string;
  logs: string[];
};

export type RunCommand = (
  command: string,
  args?: Record<string, unknown>,
  label?: string,
) => Promise<void>;

export type ToastState = {
  type: "success" | "error" | "info";
  message: string;
} | null;

export type HistoryItem = {
  id: string;
  label: string;
  command: string;
  status: string;
  createdAt: string;
};

export type ProgressPayload = {
  action: string;
  file: string;
  current: number;
  total: number;
  percent: number;
  elapsedSeconds: number;
  etaSeconds: number | null;
  done: boolean;
};

export type PipelineActionType = "copy" | "move" | "sync" | "echo";

export type PipelineStepDraft = {
  id: string;
  action: PipelineActionType;
  src: string;
  dst: string;
  recursive: boolean;
  deleteExtra: boolean;
  overwrite: boolean;
  dryRun: boolean;
};

export type PipelineConfigStep = {
  action: string;
  args: string[];
};

export type PipelineConfig = {
  name: string;
  steps: PipelineConfigStep[];
};
