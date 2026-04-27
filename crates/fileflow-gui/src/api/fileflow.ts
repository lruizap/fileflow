import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import type { GuiRunResult } from "../types";

export async function invokeFileFlow(
  command: string,
  args?: Record<string, unknown>,
): Promise<GuiRunResult> {
  return await invoke<GuiRunResult>(command, args ?? {});
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
