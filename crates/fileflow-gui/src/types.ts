export type GuiRunResult = {
  status: string;
  logs: string[];
};

export type RunCommand = (
  command: string,
  args?: Record<string, unknown>,
) => Promise<void>;
