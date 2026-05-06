import type { PluginStatus } from "./types";
import { invoke } from "@tauri-apps/api/core";

export * from "./types";
export * from "./extensions";

export async function getStatus(): Promise<PluginStatus> {
  return invoke("plugin:nero-extensions|get_status");
}
