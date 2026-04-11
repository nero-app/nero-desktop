import { t } from "../../lib/i18n";
import { appState } from "../../store/appState";
import { Button } from "../ui/Button";
import { Dialog } from "../ui/Dialog";
import { Typography } from "../ui/Typography";
import type { Episode } from "@nero/plugin-extensions";
import { invoke } from "@tauri-apps/api/core";
import { LoaderCircleIcon } from "lucide-solid";
import {
  createResource,
  createSignal,
  For,
  Match,
  Switch,
  type ComponentProps,
} from "solid-js";

type VideoSelectorProps = ComponentProps<typeof Dialog> & {
  seriesId: string;
  episode: Episode;
  onOpenChange: (open: boolean) => void;
};

export default function VideoSelector(props: VideoSelectorProps) {
  const [videosResource] = createResource(
    () => {
      if (!appState.config.playerPath) return undefined;
      return {
        id: props.episode.id,
        num: props.episode.number,
        ext: appState.extension,
      };
    },
    async (source) => {
      if (!source.ext) throw new Error("No extension loaded");
      return source.ext.getSeriesVideos(props.seriesId, source.id, source.num);
    },
  );

  const [launching, setLaunching] = createSignal(false);

  const selectVideo = async (url: string) => {
    const playerPath = appState.config.playerPath;
    if (!playerPath) return;
    try {
      setLaunching(true);
      await invoke("open_video_player", {
        playerPath: appState.config.playerPath!,
        url,
      });
    } catch (error) {
      setLaunching(false);
      alert(`${error}`);
    }
  };

  return (
    <Dialog open={props.open} onOpenChange={props.onOpenChange}>
      <Dialog.Header
        title={`${props.episode.title || "Episode"} - ${props.episode.number}`}
      />
      <Dialog.Content class="grid grid-cols-2 py-2">
        <div class="flex flex-col gap-2 overflow-y-auto px-4">
          <img
            src={props.episode.thumbnailUrl}
            class="aspect-video w-full rounded object-cover"
            alt={`Episode ${props.episode.number}: ${props.episode.title}`}
          />
          <Typography>{props.episode.description}</Typography>
        </div>

        <Switch>
          <Match when={!appState.config.playerPath}>
            <div class="flex items-center justify-center px-4">
              <Typography>{t("media.player_not_configured")}</Typography>
            </div>
          </Match>
          <Match when={videosResource.loading}>
            <div class="flex items-center justify-center">
              <Typography>{t("common.loading")}</Typography>
            </div>
          </Match>
          <Match when={videosResource.error}>
            <Typography>{videosResource.error.message}</Typography>
          </Match>
          <Match when={launching()}>
            <div
              class="flex size-full flex-col items-center justify-center gap-3
                px-4 text-center"
            >
              <LoaderCircleIcon
                size={28}
                class="animate-spin text-neutral-500"
              />
              <div class="flex flex-col gap-2">
                <Typography variant="h4">
                  {t("media.player_launching")}
                </Typography>
                <Typography variant="caption">
                  {t("media.player_launching_hint")}
                </Typography>
              </div>
            </div>
          </Match>
          <Match when={videosResource()}>
            <ul class="flex flex-col gap-1.5 overflow-y-auto px-4">
              <For each={videosResource()}>
                {(video) => (
                  <li>
                    <Button
                      variant="outline"
                      onClick={() => selectVideo(video.url)}
                    >
                      <Typography variant="body" as="span">
                        {video.server}
                      </Typography>
                      <Typography variant="caption" as="span">
                        {video.resolution.join("x")}
                      </Typography>
                    </Button>
                  </li>
                )}
              </For>
            </ul>
          </Match>
        </Switch>
      </Dialog.Content>
    </Dialog>
  );
}
