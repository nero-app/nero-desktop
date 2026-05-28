import { Typography } from "../ui/Typography";
import type { Series } from "@nero/plugin-extensions";
import { A } from "@solidjs/router";
import { ImageOffIcon } from "lucide-solid";
import { Show } from "solid-js";

export default function SeriesCard(props: {
  series: Series;
  extensionId: string;
}) {
  return (
    <A
      class="flex flex-col gap-1 rounded-md p-1 duration-300
        hover:bg-neutral-300 active:scale-95"
      href={`/series/${props.extensionId}/${props.series.id}`}
    >
      <Show
        when={props.series.posterUrl}
        fallback={
          <div
            class="flex aspect-2/3 items-center justify-center rounded-lg
              bg-neutral-200"
          >
            <ImageOffIcon class="text-neutral-300" size={28} />
          </div>
        }
      >
        <img
          class="aspect-2/3 rounded-lg object-cover"
          src={props.series.posterUrl}
          alt={props.series.title}
        />
      </Show>
      <Typography variant="subtitle" class="truncate text-center">
        {props.series.title}
      </Typography>
    </A>
  );
}
