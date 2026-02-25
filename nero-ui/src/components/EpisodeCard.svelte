<script lang="ts">
  import type { Episode } from "@nero/plugin-extensions";

  interface EpisodeCardProps {
    seriesId: string;
    episode: Episode;
    onclick?: () => void;
  }
  let { episode, onclick }: EpisodeCardProps = $props();
</script>

<button
  class="group flex flex-col items-center gap-1.5 overflow-hidden rounded-lg"
  {onclick}
>
  <div class="relative aspect-video w-full overflow-hidden rounded-lg">
    <img
      class="size-full object-cover transition-transform duration-300
        group-hover:scale-105"
      src={episode.thumbnailUrl}
      alt="Episode {episode.number}"
    />
    <div
      class="absolute inset-0 flex items-center justify-center bg-black/20
        opacity-0 transition-opacity group-hover:opacity-100"
    >
      <span class="text-sm font-semibold text-white">Ep. {episode.number}</span>
    </div>
  </div>
  <span class="w-full truncate text-center text-xs text-neutral-600">
    {episode.title}
  </span>
</button>
