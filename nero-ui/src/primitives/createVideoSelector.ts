import { getSeriesVideos, type Episode } from "@nero/plugin-extensions";
import { invoke } from "@tauri-apps/api/core";
import { createResource, type Accessor } from "solid-js";

export function createVideoSelector(
  seriesId: Accessor<string>,
  episode: Accessor<Episode>,
) {
  const [videosResource] = createResource(
    () => ({ id: episode().id, num: episode().number }),
    ({ id, num }) => getSeriesVideos(seriesId(), id, num),
  );

  // TODO: Handle errors
  const selectVideo = async (url: string) => {
    await invoke("open_video_player", { url });
  };

  return { videosResource, selectVideo };
}
