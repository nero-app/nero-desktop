import { createInfiniteResource } from "./createInfiniteResource";
import { createSentinel } from "./createSentinel";
import {
  getSeriesEpisodes,
  getSeriesInfo,
  type Episode,
} from "@nero/plugin-extensions";
import { createResource, type Accessor } from "solid-js";

export function createSeries(
  extensionId: Accessor<string>,
  seriesId: Accessor<string>,
) {
  const [seriesQuery] = createResource(
    () => ({ extensionId: extensionId(), seriesId: seriesId() }),
    ({ extensionId, seriesId }) => getSeriesInfo(extensionId, seriesId),
  );

  const [episodesQuery, { loadNext }] = createInfiniteResource<Episode>(
    async (page) => {
      const result = await getSeriesEpisodes(extensionId(), seriesId(), page);
      return { items: result.items, hasMore: result.hasNextPage };
    },
  );

  const sentinel = createSentinel(() => loadNext());

  return { seriesQuery, episodesQuery, sentinel };
}
