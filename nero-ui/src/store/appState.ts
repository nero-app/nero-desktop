import {
  disableTorrentSupport,
  enableTorrentSupport,
  Extension,
  type ExtensionOptions,
} from "@nero/plugin-extensions";
import { createFluxStore } from "@solid-primitives/flux-store";
import { appCacheDir } from "@tauri-apps/api/path";

export const appState = createFluxStore(
  {
    playerPath: null as string | null,
    torrentEnabled: false,
    torrentOutputFolder: null as string | null,
    extension: null as Extension | null,
  },
  {
    getters: (state) => ({
      playerPath: () => state.playerPath,
      torrentEnabled: () => state.torrentEnabled,
      torrentOutputFolder: () => state.torrentOutputFolder,
      extension: () => state.extension,
    }),
    actions: (setState, state) => ({
      setPlayerPath(path: string) {
        setState("playerPath", path);
      },

      async enableTorrent(outputFolder: string | null) {
        if (state.torrentEnabled) await disableTorrentSupport();
        const folder = outputFolder ?? (await appCacheDir());
        await enableTorrentSupport(folder);
        setState("torrentEnabled", true);
        setState("torrentOutputFolder", outputFolder);
      },

      async disableTorrent() {
        await disableTorrentSupport();
        setState("torrentEnabled", false);
      },

      async loadExtension(filePath: string, options: ExtensionOptions) {
        const ext = await Extension.load(filePath, options);
        setState("extension", ext);
        return ext;
      },
    }),
  },
);
