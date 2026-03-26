import SeriesCard from "../components/media/SeriesCard";
import { Input } from "../components/ui/Input";
import { SidebarLayout } from "../layouts/SidebarLayout";
import { t } from "../lib/i18n";
import { createInfiniteResource } from "../primitives/createInfiniteResource";
import { createSentinel } from "../primitives/createSentinel";
import { appState } from "../store/appState";
import { Accordion, Checkbox } from "@kobalte/core";
import type { CheckboxRootProps } from "@kobalte/core/checkbox";
import type {
  Filter,
  FilterCategory,
  SearchFilter,
  Series,
} from "@nero/plugin-extensions";
import { SearchIcon, CheckIcon } from "lucide-solid";
import {
  For,
  type JSX,
  Match,
  Show,
  Switch,
  createResource,
  createSignal,
  splitProps,
} from "solid-js";

type FilterItemProps = CheckboxRootProps & {
  filter: Filter;
};

// TODO: styles
function FilterItem(props: FilterItemProps) {
  const [local, checkboxProps] = splitProps(props, ["filter"]);

  return (
    <Checkbox.Root
      class="flex cursor-pointer items-center gap-2"
      {...checkboxProps}
    >
      <Checkbox.Input />
      <Checkbox.Control
        class="inline-flex size-5 items-center justify-center rounded border
          transition-colors data-checked:border-blue-500
          data-checked:bg-blue-500"
      >
        <Checkbox.Indicator>
          <CheckIcon class="size-3 text-white" />
        </Checkbox.Indicator>
      </Checkbox.Control>
      <Checkbox.Label class="cursor-pointer text-sm">
        {local.filter.displayName}
      </Checkbox.Label>
    </Checkbox.Root>
  );
}

function FilterCategoryItem(props: {
  category: FilterCategory;
  children: (filter: Filter) => JSX.Element;
}) {
  return (
    <Accordion.Item value={props.category.id}>
      <Accordion.Header>
        <Accordion.Trigger class="flex w-full font-medium hover:bg-gray-100">
          {props.category.displayName}
        </Accordion.Trigger>
      </Accordion.Header>
      <Accordion.Content as="ul">
        <For each={props.category.filters}>
          {(f) => <li>{props.children(f)}</li>}
        </For>
      </Accordion.Content>
    </Accordion.Item>
  );
}

function FilterCategoryList(props: {
  categories: FilterCategory[];
  children: (category: FilterCategory, filter: Filter) => JSX.Element;
}) {
  return (
    <Accordion.Root multiple defaultValue={props.categories.map((c) => c.id)}>
      <For each={props.categories}>
        {(category) => (
          <FilterCategoryItem category={category}>
            {(filter) => props.children(category, filter)}
          </FilterCategoryItem>
        )}
      </For>
    </Accordion.Root>
  );
}

export default function SearchPage() {
  const [filterCategories] = createResource(() => {
    const extension = appState.extension;
    if (!extension) throw new Error("No extension loaded");
    return extension.getFilters();
  });

  const [searchFilters, setSearchFilters] = createSignal<SearchFilter[]>([]);
  const [query, setQuery] = createSignal("");

  const [series, { loadNext, reset }] = createInfiniteResource<Series>(
    async (page) => {
      const extension = appState.extension;
      if (!extension) throw new Error("No extension loaded");
      const result = await extension.search(query(), page, searchFilters());
      return {
        items: result.items,
        hasMore: result.hasNextPage,
      };
    },
  );

  const sentinel = createSentinel(() => loadNext());

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

      if (!existing) {
        return [...prev, { id: categoryId, values: [filterId] }];
      }

      const hasFilter = existing.values.includes(filterId);

      if (hasFilter) {
        const newValues = existing.values.filter((v) => v !== filterId);
        if (newValues.length === 0) {
          return prev.filter((sf) => sf.id !== categoryId);
        }
        return prev.map((sf) =>
          sf.id === categoryId ? { ...sf, values: newValues } : sf,
        );
      } else {
        return prev.map((sf) =>
          sf.id === categoryId
            ? { ...sf, values: [...sf.values, filterId] }
            : sf,
        );
      }
    });
  }

  function handleSearchSubmit(e: Event) {
    e.preventDefault();
    reset();
  }

  return (
    <SidebarLayout>
      <SidebarLayout.Main as="section">
        <Switch>
          <Match when={series.loading && series().length === 0}>
            <p class="animate-pulse text-gray-500">{t("common.loading")}</p>
          </Match>

          <Match when={series.error}>
            <p class="text-red-500">{series.error.message}</p>
          </Match>
          <Match when={series().length > 0}>
            <ul class="grid grid-cols-4">
              <For each={series()}>
                {(series) => (
                  <li>
                    <SeriesCard series={series} />
                  </li>
                )}
              </For>

              <div ref={sentinel} />

              <Show when={series.loading}>
                <p class="animate-pulse text-gray-500">{t("common.loading")}</p>
              </Show>
            </ul>
          </Match>

          <Match when={!series.loading && series().length === 0}>
            <p class="text-gray-400">{t("media.no_results")}</p>
          </Match>
        </Switch>
      </SidebarLayout.Main>

      <SidebarLayout.Sidebar as="aside">
        <form
          class="flex items-center gap-2 rounded-lg bg-slate-100 px-3 py-2"
          onSubmit={handleSearchSubmit}
        >
          <SearchIcon />
          <Input
            variant="filled"
            class="text-slate-600 outline-none placeholder:text-slate-400"
            type="search"
            placeholder={t("media.search_placeholder")}
            value={query()}
            onInput={(e) => setQuery(e.currentTarget.value)}
            ref={(el) => requestAnimationFrame(() => el.focus())}
          />
        </form>

        <Switch fallback={<p class="text-gray-400">{t("filters.empty")}</p>}>
          <Match when={filterCategories.loading}>
            <p class="animate-pulse text-gray-500">{t("common.loading")}</p>
          </Match>
          <Match when={filterCategories.error}>
            <p class="text-red-500">{filterCategories.error.message}</p>
          </Match>
          <Match when={filterCategories()}>
            {(categories) => (
              <FilterCategoryList categories={categories()}>
                {(category, filter) => (
                  <FilterItem
                    filter={filter}
                    checked={isFilterSelected(category.id, filter.id)}
                    onChange={() => toggleFilter(category.id, filter.id)}
                  />
                )}
              </FilterCategoryList>
            )}
          </Match>
        </Switch>
      </SidebarLayout.Sidebar>
    </SidebarLayout>
  );
}
