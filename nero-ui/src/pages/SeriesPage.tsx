import EpisodeCard from "../components/media/EpisodeCard";
import VideoSelector from "../components/media/VideoSelector";
import { Button } from "../components/ui/Button";
import { MediaLayout } from "../layouts/MediaLayout";
import { t } from "../lib/i18n";
import { createInfiniteResource } from "../primitives/createInfiniteResource";
import { createSentinel } from "../primitives/createSentinel";
import { appState } from "../store/appState";
import { Tabs } from "@kobalte/core/tabs";
import type { Episode, Extension } from "@nero/plugin-extensions";
import { A, useParams } from "@solidjs/router";
import { PlayIcon, Share2Icon, ThumbsUpIcon } from "lucide-solid";
import {
  createResource,
  Switch,
  Match,
  createSignal,
  Show,
  For,
} from "solid-js";

function extensionLabel(extension: Extension) {
  const { name, version } = extension.metadata;
  const label = name ?? extension.filePath;
  return version ? `${label}@v${version}` : label;
}

export default function SeriesPage() {
  const params = useParams<{ seriesId: string }>();

  const [isOpen, setIsOpen] = createSignal(false);
  const [selectedEpisode, setSelectedEpisode] = createSignal<Episode | null>(
    null,
  );

  const [seriesQuery] = createResource(async () => {
    const extension = appState.extension;
    if (!extension) throw new Error("No extension loaded");
    return extension.getSeriesInfo(params.seriesId);
  });

  const [episodesQuery, { loadNext }] = createInfiniteResource<Episode>(
    async (page) => {
      const extension = appState.extension;
      if (!extension) throw new Error("No extension loaded");
      const result = await extension.getSeriesEpisodes(params.seriesId, page);
      return { items: result.items, hasMore: result.hasNextPage };
    },
  );

  const firstEpisode = () => episodesQuery()?.[0] ?? null;
  const sentinel = createSentinel(() => loadNext());

  const handleEpisodeClick = (episode: Episode) => {
    setSelectedEpisode(episode);
    setIsOpen(true);
  };

  return (
    <>
      <MediaLayout>
        <MediaLayout.Media>
          <Switch>
            <Match when={seriesQuery.loading}>
              <p class="animate-pulse text-gray-500">{t("common.loading")}</p>
            </Match>
            <Match when={seriesQuery.error}>
              <div class="size-full bg-gray-200" />
            </Match>
            <Match when={seriesQuery()}>
              <img
                class="size-full object-cover"
                src={seriesQuery()?.posterUrl}
                alt={seriesQuery()?.title}
              />
            </Match>
          </Switch>
        </MediaLayout.Media>

        <MediaLayout.Content as="article" class="flex-col gap-4">
          <Switch>
            <Match when={seriesQuery.loading}>
              <p class="animate-pulse text-gray-500">{t("common.loading")}</p>
            </Match>
            <Match when={seriesQuery.error}>
              <p class="text-red-500">{seriesQuery.error.message}</p>
            </Match>
            <Match when={seriesQuery()}>
              <header class="flex flex-col gap-4">
                <h1 class="truncate text-4xl font-bold text-neutral-900">
                  {seriesQuery()?.title}
                </h1>
                <div class="flex items-center gap-1.5 text-neutral-600">
                  <span>{seriesQuery()?.type}</span>
                  <span>{t("common.separator")}</span>
                  <A class="truncate underline" href="/settings/extensions">
                    {extensionLabel(appState.extension!)}
                  </A>
                </div>
                <p class="line-clamp-4 leading-relaxed text-neutral-900">
                  {seriesQuery()?.synopsis}
                </p>
                <div class="flex w-full items-center gap-6 py-2">
                  <Button
                    class="w-full"
                    disabled={!firstEpisode()}
                    onClick={() => handleEpisodeClick(firstEpisode()!)}
                  >
                    <PlayIcon size={24} />
                    {t("media.start_watching")}
                  </Button>
                  {/* TODO: onClick */}
                  <Button variant="outline" size="icon" disabled>
                    <Share2Icon size={20} />
                  </Button>
                  {/* TODO: onClick */}
                  <Button variant="outline" size="icon" disabled>
                    <ThumbsUpIcon size={20} stroke-width={2} />
                  </Button>
                </div>
              </header>
            </Match>
          </Switch>

          <Tabs defaultValue="episodes">
            <Tabs.List class="flex w-full justify-between border-t">
              <Tabs.Trigger value="episodes" class="tab-trigger">
                {t("media.episodes")}
              </Tabs.Trigger>
            </Tabs.List>

            <Tabs.Content
              value="episodes"
              as="ul"
              class="grid grid-cols-3 gap-2 pt-2"
            >
              <For each={episodesQuery()}>
                {(episode) => (
                  <li>
                    <EpisodeCard
                      seriesId={params.seriesId}
                      episode={episode}
                      onClick={handleEpisodeClick}
                    />
                  </li>
                )}
              </For>
              <div ref={sentinel} />
            </Tabs.Content>
          </Tabs>
        </MediaLayout.Content>
      </MediaLayout>

      <Show when={selectedEpisode()}>
        {(episode) => (
          <VideoSelector
            seriesId={params.seriesId}
            episode={episode()}
            open={isOpen()}
            onOpenChange={setIsOpen}
          />
        )}
      </Show>
    </>
  );
}
