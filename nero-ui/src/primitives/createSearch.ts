import { t } from "../lib/i18n";
import { appState } from "../store/appState";
import { createInfiniteResource } from "./createInfiniteResource";
import type { SearchFilter } from "@nero/plugin-extensions";
import type { Series } from "@nero/plugin-extensions";
import { createSignal } from "solid-js";

export function createSearch() {
  const [query, setQuery] = createSignal("");
  const [searchFilters, setSearchFilters] = createSignal<SearchFilter[]>([]);

  const [series, { loadNext, reset }] = createInfiniteResource<Series>(
    async (page) => {
      const extension = appState.extension;
      if (!extension) throw new Error(t("common.no_extension"));
      const result = await extension.search(query(), page, searchFilters());
      return { items: result.items, hasMore: result.hasNextPage };
    },
  );

  function isFilterSelected(categoryId: string, filterId: string) {
    return (
      searchFilters()
        .find((sf) => sf.id === categoryId)
        ?.values.includes(filterId) ?? false
    );
  }

  function toggleFilter(categoryId: string, filterId: string) {
    setSearchFilters((prev) => {
      const existing = prev.find((sf) => sf.id === categoryId);
      if (!existing) return [...prev, { id: categoryId, values: [filterId] }];

      const hasFilter = existing.values.includes(filterId);
      if (hasFilter) {
        const newValues = existing.values.filter((v) => v !== filterId);
        return newValues.length === 0
          ? prev.filter((sf) => sf.id !== categoryId)
          : prev.map((sf) =>
              sf.id === categoryId ? { ...sf, values: newValues } : sf,
            );
      }

      return prev.map((sf) =>
        sf.id === categoryId ? { ...sf, values: [...sf.values, filterId] } : sf,
      );
    });
  }

  return {
    query,
    setQuery,
    series,
    loadNext,
    reset,
    isFilterSelected,
    toggleFilter,
  };
}
