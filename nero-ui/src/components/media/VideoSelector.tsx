import { t } from "../../lib/i18n";
import { createVideoSelector } from "../../primitives/createVideoSelector";
import { appState } from "../../store/appState";
import { Button } from "../ui/Button";
import { Dialog } from "../ui/Dialog";
import { Typography } from "../ui/Typography";
import type { Episode, Video } from "@nero/plugin-extensions";
import { For, Match, Switch, type ComponentProps } from "solid-js";

function VideoCard(props: { video: Video; onClick?: (video: Video) => void }) {
  return (
    <Button
      class="w-full"
      variant="outline"
      onClick={() => props.onClick?.(props.video)}
    >
      <Typography variant="body" as="span">
        {props.video.server}
      </Typography>
      <Typography variant="caption" as="span">
        {props.video.resolution.join("x")}
      </Typography>
    </Button>
  );
}

type VideoSelectorProps = ComponentProps<typeof Dialog> & {
  seriesId: string;
  episode: Episode;
  onOpenChange: (open: boolean) => void;
};

export default function VideoSelector(props: VideoSelectorProps) {
  const { videosResource, selectVideo } = createVideoSelector(
    () => props.seriesId,
    () => props.episode,
  );

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
          <Match when={videosResource()}>
            <ul class="flex flex-col gap-1.5 overflow-y-auto px-4">
              <For each={videosResource()}>
                {(video) => (
                  <li>
                    <VideoCard
                      video={video}
                      onClick={() => selectVideo(video.url)}
                    />
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
