import { Typography } from "../ui/Typography";
import type { Episode } from "@nero/plugin-extensions";

type EpisodeCardProps = {
  seriesId: string;
  episode: Episode;
  onClick?: (episode: Episode) => void;
};

export default function EpisodeCard(props: EpisodeCardProps) {
  return (
    <button
      class="group flex w-full cursor-pointer flex-col gap-1 overflow-hidden
        rounded"
      onClick={() => props.onClick?.(props.episode)}
    >
      <div class="relative aspect-video w-full overflow-hidden rounded-lg">
        <img
          class="size-full object-cover transition-transform duration-300
            group-hover:scale-105"
          src={props.episode.thumbnailUrl}
          alt={`Episode ${props.episode.number}: ${props.episode.title}`}
        />

        <div
          class="absolute inset-0 flex items-center justify-center bg-black/20
            opacity-0 transition-opacity group-hover:opacity-100"
        >
          <Typography variant="subtitle" as="span">
            Ep. {props.episode.number}
          </Typography>
        </div>
      </div>

      <Typography
        variant="subtitle"
        as="span"
        class="w-full truncate text-center"
      >
        {props.episode.title}
      </Typography>
    </button>
  );
}
