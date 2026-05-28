import { createInfiniteResource } from "./createInfiniteResource";
import { createSentinel } from "./createSentinel";
import {
  search,
  type LoadedExtension,
  type SearchFilter,
} from "@nero/plugin-extensions";
import { createMemo, type Accessor } from "solid-js";

export function createSearch(
  extensionId: Accessor<string>,
  query: Accessor<string>,
  filters: Accessor<SearchFilter[]>,
) {
  const [series, { loadNext, reset }] = createInfiniteResource(async (page) => {
    const result = await search(extensionId(), query(), page, filters());
    return { items: result.items, hasMore: result.hasNextPage };
  });

  return { series, reset, loadNext };
}

export function createExtensionsSearch(
  extensions: Accessor<LoadedExtension[]>,
  query: Accessor<string>,
  filtersFor: (extensionId: string) => Accessor<SearchFilter[]>,
) {
  const states = createMemo(() =>
    extensions().map((ext) => {
      const { series, loadNext, reset } = createSearch(
        () => ext.id,
        query,
        filtersFor(ext.id),
      );
      return { ext, series, loadNext, reset };
    }),
  );

  const sentinel = createSentinel(() =>
    states().forEach(({ loadNext }) => loadNext()),
  );

  const allSeries = () =>
    states().flatMap(({ ext, series }) =>
      series().map((s) => ({ series: s, ext })),
    );

  const isLoading = () => states().some(({ series }) => series.loading);
  const reset = () => states().forEach(({ reset }) => reset());

  return { allSeries, isLoading, sentinel, reset };
}
