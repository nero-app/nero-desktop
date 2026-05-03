import { createInfiniteResource } from "./createInfiniteResource";
import { createSentinel } from "./createSentinel";
import {
  getSeriesEpisodes,
  getSeriesInfo,
  type Episode,
} from "@nero/plugin-extensions";
import { createResource, type Accessor } from "solid-js";

export function createSeries(seriesId: Accessor<string>) {
  const [seriesQuery] = createResource(seriesId, getSeriesInfo);

  const [episodesQuery, { loadNext }] = createInfiniteResource<Episode>(
    async (page) => {
      const result = await getSeriesEpisodes(seriesId(), page);
      return { items: result.items, hasMore: result.hasNextPage };
    },
  );

  const sentinel = createSentinel(() => loadNext());

  return { seriesQuery, episodesQuery, sentinel };
}
