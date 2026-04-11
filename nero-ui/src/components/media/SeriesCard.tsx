import { Typography } from "../ui/Typography";
import type { Series } from "@nero/plugin-extensions";
import { A } from "@solidjs/router";
import { Show } from "solid-js";

export default function SeriesCard(props: { series: Series }) {
  return (
    <A
      class="flex flex-col gap-1 rounded-md p-1 duration-300 hover:bg-gray-200
        active:scale-95"
      href={`/series/${props.series.id}`}
    >
      <Show
        when={props.series.posterUrl}
        fallback={<div class="aspect-2/3 rounded-lg bg-gray-200" />}
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
