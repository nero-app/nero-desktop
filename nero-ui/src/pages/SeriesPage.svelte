<script lang="ts">
  import EpisodeCard from "../components/EpisodeCard.svelte";
  import ErrorMessage from "../components/ErrorMessage.svelte";
  import VideoSelectionModal from "../components/VideoSelectionModal.svelte";
  // import { appState } from "../lib/appState.svelte";
  import { createInfiniteQuery } from "../lib/createInfiniteQuery.svelte";
  import { createQuery } from "../lib/createQuery.svelte";
  import { sampleSeries } from "../lib/dummy";
  import { createInfiniteScroll } from "../lib/infiniteScroll.svelte";
  import { PlayIcon, Share2Icon, ThumbsUpIcon } from "@lucide/svelte";
  import type { Episode } from "@nero/plugin-extensions";
  import { Tabs } from "bits-ui";

  let { params }: { params: { seriesId: string } } = $props();

  let seriesQuery = createQuery(() => {
    // const extension = appState.extension;
    // if (!extension) throw new Error("No extension loaded");
    // return extension.getSeriesInfo(params.seriesId);
    return sampleSeries;
  });

  let episodesQuery = createInfiniteQuery(async (page) => {
    // const extension = appState.extension;
    // if (!extension) throw new Error("No extension loaded");
    // const result = await extension.getSeriesEpisodes(params.seriesId, page);
    // return {
    // data: result.items,
    // hasNextPage: result.hasNextPage,
    // };
    return {
      data: [],
      hasNextPage: false,
    };
  });

  let infiniteScroll = createInfiniteScroll(() =>
    episodesQuery.fetchNextPage(),
  );

  let selectedEpisode = $state<Episode | null>(null);
  let firstEpisode = $derived(episodesQuery.data[0]);

  function openFirstEpisode() {
    if (firstEpisode) selectedEpisode = firstEpisode;
  }

  function closeVideoModal() {
    selectedEpisode = null;
  }
</script>

<div class="grid h-full grid-cols-[2fr_3fr]">
  <figure class="overflow-hidden">
    {#if seriesQuery.isLoading}
      <p>Loading...</p>
    {:else if seriesQuery.isError}
      <div class="size-full rounded-xl bg-gray-700"></div>
    {:else if seriesQuery.data}
      <img
        class="size-full object-cover"
        src={seriesQuery.data.posterUrl}
        alt="{seriesQuery.data.title} poster"
      />
    {/if}
  </figure>

  <article class="flex flex-col justify-center gap-4 overflow-y-auto p-8">
    {#if seriesQuery.isLoading}
      <p>Loading...</p>
    {:else if seriesQuery.error}
      <ErrorMessage
        message="Apparently an error has occurred"
        error={seriesQuery.error}
      />
    {:else if seriesQuery.data}
      <header class="flex flex-col gap-4 pt-12">
        <h1 class="truncate text-4xl font-bold text-neutral-100">
          {seriesQuery.data.title}
        </h1>
        <div class="flex items-center gap-1.5 text-sm text-neutral-300">
          <span>{seriesQuery.data.type}</span>
          <span>•</span>
          <!-- TODO: -->
          <span>some-extension@v0.1.0-draft</span>
        </div>
        <p class="line-clamp-3 text-sm leading-relaxed text-neutral-300">
          {seriesQuery.data.synopsis}
        </p>
        <div class="flex w-full items-center gap-6 py-4">
          <button
            class="flex flex-1 cursor-pointer items-center justify-center gap-2.5
              rounded-lg bg-neutral-100 px-12 py-2 text-sm font-semibold
              text-neutral-900 transition-colors hover:bg-black/5"
            onclick={openFirstEpisode}
          >
            <PlayIcon size={24} fill="currentColor" strokeWidth={0} />
            Start Watching
          </button>
          <button
            class="flex size-10 shrink-0 cursor-pointer items-center justify-center
              rounded-full border-[1.5px] border-neutral-500 text-neutral-100
              transition-colors hover:border-neutral-600 hover:bg-black/5"
            title="Share"
          >
            <Share2Icon size={15} />
          </button>
          <button
            class="flex size-10 shrink-0 cursor-pointer items-center justify-center
              rounded-full border-[1.5px] border-neutral-500 text-neutral-100
              transition-colors hover:border-neutral-600 hover:bg-black/5"
            title="Like"
          >
            <ThumbsUpIcon size={15} strokeWidth={2.5} />
          </button>
        </div>
      </header>
    {/if}

    <Tabs.Root value="episodes">
      <Tabs.List
        class="flex w-full justify-between border-t border-neutral-600"
      >
        <Tabs.Trigger
          value="episodes"
          class="tab-trigger flex-1 cursor-pointer py-2.5 text-center text-sm
            text-neutral-400 transition-colors hover:text-neutral-300
            data-[state=active]:border-t-2
            data-[state=active]:border-neutral-100
            data-[state=active]:font-semibold
            data-[state=active]:text-neutral-100"
        >
          Episodes
        </Tabs.Trigger>
        <Tabs.Trigger
          value="more"
          class="tab-trigger flex-1 cursor-pointer py-2.5 text-center text-sm
            text-neutral-400 transition-colors hover:text-neutral-300
            data-[state=active]:border-t-2
            data-[state=active]:border-neutral-100
            data-[state=active]:font-semibold
            data-[state=active]:text-neutral-100"
        >
          More like this
        </Tabs.Trigger>
      </Tabs.List>

      <Tabs.Content value="episodes" class="pt-2">
        {#if episodesQuery.isLoading}
          <p class="text-center text-sm text-neutral-400">Loading...</p>
        {:else if episodesQuery.error}
          <ErrorMessage
            message="Apparently an error has occurred"
            error={episodesQuery.error}
          />
        {:else if episodesQuery.isSuccess}
          <div class="grid grid-cols-3 gap-2">
            {#each episodesQuery.data as episode (episode.id)}
              <EpisodeCard
                seriesId={params.seriesId}
                {episode}
                onclick={() => (selectedEpisode = episode)}
              />
            {/each}
          </div>
          <div bind:this={infiniteScroll.element}></div>
          {#if episodesQuery.isFetchingNextPage}
            <p class="p-2 text-center text-sm text-neutral-400">
              Loading more...
            </p>
          {/if}
        {/if}
      </Tabs.Content>

      <Tabs.Content value="more" class="text-neutral-400">
        Here goes the more like this
      </Tabs.Content>
    </Tabs.Root>
  </article>
</div>

{#if selectedEpisode}
  <VideoSelectionModal
    seriesId={params.seriesId}
    episode={selectedEpisode}
    onClose={closeVideoModal}
  />
{/if}

<style>
  :global(.tab-trigger) {
    position: relative;
  }

  :global(.tab-trigger[data-state="active"])::after {
    content: "";
    position: absolute;
    top: 0px;
    left: 50%;
    transform: translateX(-50%);
    border-left: 6px solid transparent;
    border-right: 6px solid transparent;
    border-top: 6px solid white;
  }
</style>
