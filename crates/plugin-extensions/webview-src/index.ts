import { invoke } from "@tauri-apps/api/core";

export * from "./types";
export * from "./extensions";

export interface MediaProxyPreferences {
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
  mediaProxy?: MediaProxyPreferences;
}

export async function getPreferences(): Promise<PreferencesData> {
  return invoke("plugin:nero-extensions|get_preferences");
}

export async function setMediaProxyPreferences(
  mediaProxy: MediaProxyPreferences,
): Promise<void> {
  return invoke("plugin:nero-extensions|set_media_proxy_preferences", {
    mediaProxy,
  });
}
