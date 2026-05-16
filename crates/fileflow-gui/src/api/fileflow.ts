import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import type { JobPriority, ManagedJob, QueueState } from "../types";

export async function invokeFileFlow(
  command: string,
  args?: Record<string, unknown>,
  priority?: JobPriority,
): Promise<ManagedJob> {
  return await invoke<ManagedJob>(command, { ...(args ?? {}), priority });
}

export async function getQueueState(): Promise<QueueState> {
  return await invoke<QueueState>("get_queue_state");
}

export async function setConcurrencyLimit(limit: number): Promise<QueueState> {
  return await invoke<QueueState>("set_concurrency_limit", { limit });
}

export async function cancelJob(jobId: number): Promise<ManagedJob> {
  return await invoke<ManagedJob>("cancel_job", { jobId });
}

export async function updateJobPriority(
  jobId: number,
  priority: JobPriority,
): Promise<ManagedJob> {
  return await invoke<ManagedJob>("update_job_priority", { jobId, priority });
}

export async function savePipelineJson(path: string, content: string) {
  await invoke("save_pipeline_json", { path, content });
}

export async function readPipelineJson(path: string): Promise<string> {
  return await invoke<string>("read_pipeline_json", { path });
}

export async function pickFile(): Promise<string | null> {
  const selected = await open({
    multiple: false,
    directory: false,
  });

  return typeof selected === "string" ? selected : null;
}

export async function pickDirectory(): Promise<string | null> {
  const selected = await open({
    multiple: false,
    directory: true,
  });

  return typeof selected === "string" ? selected : null;
}

export async function pickJsonFile(): Promise<string | null> {
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

  return typeof selected === "string" ? selected : null;
}

export async function pickDestinationFile(): Promise<string | null> {
  const selected = await save({
    filters: [
      {
        name: "Todos los archivos",
        extensions: ["*"],
      },
    ],
  });

  return typeof selected === "string" ? selected : null;
}

export async function pickSaveJsonFile(): Promise<string | null> {
  const selected = await save({
    filters: [
      {
        name: "Pipeline JSON",
        extensions: ["json"],
      },
    ],
  });

  return typeof selected === "string" ? selected : null;
}
