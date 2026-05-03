import { createInfiniteResource } from "./createInfiniteResource";
import { createSentinel } from "./createSentinel";
import { search, type SearchFilter } from "@nero/plugin-extensions";
import { createSignal, type Accessor } from "solid-js";

export function createSearch(filters: Accessor<SearchFilter[]>) {
  const [query, setQuery] = createSignal("");

  const [series, { loadNext, reset }] = createInfiniteResource(async (page) => {
    const result = await search(query(), page, filters());
    return { items: result.items, hasMore: result.hasNextPage };
  });

  const sentinel = createSentinel(() => loadNext());

  return { query, setQuery, series, reset, sentinel };
}
