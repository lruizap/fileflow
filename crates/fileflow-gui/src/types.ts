export type JobPriority = "low" | "normal" | "high" | "critical";

export type JobStatus =
  | "QUEUED"
  | "RUNNING"
  | "SUCCESS"
  | "FAILED"
  | "CANCELLED";

export type RunCommand = (
  command: string,
  args?: Record<string, unknown>,
  label?: string,
  priority?: JobPriority,
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

export type SavedPipeline = {
  id: string;
  name: string;
  path: string;
  updatedAt: string;
};

export type ProgressPayload = {
  jobId: number;
  action: string;
  file: string;
  current: number;
  total: number;
  percent: number;
  elapsedSeconds: number;
  etaSeconds: number | null;
  done: boolean;
};

export type ManagedJob = {
  id: number;
  command: string;
  label: string;
  status: JobStatus;
  priority: JobPriority;
  progress: ProgressPayload | null;
  logs: string[];
  error: string | null;
  createdAt: number;
  startedAt: number | null;
  finishedAt: number | null;
};

export type QueueState = {
  jobs: ManagedJob[];
  concurrencyLimit: number;
  runningCount: number;
  queuedCount: number;
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
