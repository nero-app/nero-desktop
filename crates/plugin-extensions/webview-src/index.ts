import { invoke } from "@tauri-apps/api/core";

export * from "./types";
export * from "./extensions";

export interface ProcessorPreferences {
  torrentEnabled: boolean;
  torrentOutputFolder: string;
}

export interface ExtensionPreferences {
  filePath: string;
  cacheDir: string;
  maxCacheSize?: number;
}

export interface PreferencesData {
  extension?: ExtensionPreferences;
  processor?: ProcessorPreferences;
}

export async function getPreferences(): Promise<PreferencesData> {
  return invoke("plugin:nero-extensions|get_preferences");
}

export async function setProcessorPreferences(
  processor: ProcessorPreferences,
): Promise<void> {
  return invoke("plugin:nero-extensions|set_processor_preferences", {
    processor,
  });
}
