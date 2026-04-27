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
