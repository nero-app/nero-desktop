import SeriesCard from "../components/media/SeriesCard";
import { Input } from "../components/ui/Input";
import { SidebarLayout } from "../layouts/SidebarLayout";
import { t } from "../lib/i18n";
import { createFilters } from "../primitives/createFilters";
import { createSearch } from "../primitives/createSearch";
import { Accordion, Checkbox } from "@kobalte/core";
import type { CheckboxRootProps } from "@kobalte/core/checkbox";
import type { Filter, FilterCategory } from "@nero/plugin-extensions";
import { SearchIcon, CheckIcon } from "lucide-solid";
import { For, type JSX, Match, Show, Switch, splitProps } from "solid-js";

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
  const filters = createFilters();
  const { series, sentinel, query, setQuery, reset } = createSearch(
    filters.selected,
  );

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
                {(s) => (
                  <li>
                    <SeriesCard series={s} />
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
          onSubmit={(e) => {
            e.preventDefault();
            reset();
          }}
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
          <Match when={filters.categories.loading}>
            <p class="animate-pulse text-gray-500">{t("common.loading")}</p>
          </Match>
          <Match when={filters.categories.error}>
            <p class="text-red-500">{filters.categories.error.message}</p>
          </Match>
          <Match when={filters.categories()}>
            {(cats) => (
              <FilterCategoryList categories={cats()}>
                {(category, filter) => (
                  <FilterItem
                    filter={filter}
                    checked={filters.isSelected(category.id, filter.id)}
                    onChange={() => filters.toggle(category.id, filter.id)}
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
