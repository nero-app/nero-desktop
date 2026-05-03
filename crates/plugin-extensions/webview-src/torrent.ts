import { invoke } from "@tauri-apps/api/core";

export async function enableTorrentSupport(
  outputFolder: string,
): Promise<void> {
  return invoke("plugin:nero-extensions|enable_torrent_support", {
    outputFolder,
  });
}

export async function disableTorrentSupport(): Promise<void> {
  return invoke("plugin:nero-extensions|disable_torrent_support");
}
