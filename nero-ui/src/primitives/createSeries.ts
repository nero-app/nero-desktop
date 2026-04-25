import { t } from "../lib/i18n";
import { appState } from "../store/appState";
import { createInfiniteResource } from "./createInfiniteResource";
import { createSentinel } from "./createSentinel";
import type { Episode } from "@nero/plugin-extensions";
import { createResource } from "solid-js";

export function createSeries(seriesId: () => string) {
  const [seriesQuery] = createResource(async () => {
    const extension = appState.getters.extension();
    if (!extension) throw new Error(t("common.no_extension"));
    return extension.getSeriesInfo(seriesId());
  });

  const [episodesQuery, { loadNext }] = createInfiniteResource<Episode>(
    async (page) => {
      const extension = appState.getters.extension();
      if (!extension) throw new Error(t("common.no_extension"));
      const result = await extension.getSeriesEpisodes(seriesId(), page);
      return { items: result.items, hasMore: result.hasNextPage };
    },
  );

  const sentinel = createSentinel(() => loadNext());

  return { seriesQuery, episodesQuery, sentinel };
}
