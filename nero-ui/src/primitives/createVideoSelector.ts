import { t } from "../lib/i18n";
import { appState } from "../store/appState";
import type { Episode } from "@nero/plugin-extensions";
import { invoke } from "@tauri-apps/api/core";
import { createResource, createSignal } from "solid-js";

export function createVideoSelector(
  seriesId: () => string,
  episode: () => Episode,
) {
  const [videosResource] = createResource(
    () => {
      if (!appState.config.playerPath) return undefined;
      return {
        id: episode().id,
        num: episode().number,
        ext: appState.extension,
      };
    },
    async (source) => {
      if (!source.ext) throw new Error(t("common.no_extension"));
      return source.ext.getSeriesVideos(seriesId(), source.id, source.num);
    },
  );

  const [launching, setLaunching] = createSignal(false);

  const selectVideo = async (url: string) => {
    const playerPath = appState.config.playerPath;
    if (!playerPath) return;
    try {
      setLaunching(true);
      await invoke("open_video_player", { playerPath, url });
    } catch (error) {
      setLaunching(false);
      alert(`${error}`);
    }
  };

  return { videosResource, launching, selectVideo };
}
