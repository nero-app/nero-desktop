import SeriesCard from "../components/media/SeriesCard";
import { Input } from "../components/ui/Input";
import { Typography } from "../components/ui/Typography";
import { SidebarLayout } from "../layouts/SidebarLayout";
import { t } from "../lib/i18n";
import { createExtensionsFilters } from "../primitives/createFilters";
import { createExtensionsSearch } from "../primitives/createSearch";
import { Accordion, Checkbox } from "@kobalte/core";
import type { CheckboxRootProps } from "@kobalte/core/checkbox";
import {
  getLoadedExtensions,
  type Filter,
  type FilterCategory,
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
  const [extensions] = createResource(() => getLoadedExtensions(), {
    initialValue: [],
  });
  const [query, setQuery] = createSignal("");

  const { items: filterItems, filtersFor } =
    createExtensionsFilters(extensions);
  const { allSeries, isLoading, sentinel, reset } = createExtensionsSearch(
    extensions,
    query,
    filtersFor,
  );

  return (
    <SidebarLayout>
      <SidebarLayout.Main as="section">
        <Switch>
          <Match when={isLoading() && allSeries().length === 0}>
            <Typography>{t("common.loading")}</Typography>
          </Match>
          <Match when={allSeries().length > 0}>
            <ul class="grid grid-cols-4">
              <For each={allSeries()}>
                {({ series, ext }) => (
                  <li>
                    <SeriesCard series={series} extensionId={ext.id} />
                  </li>
                )}
              </For>
              <div ref={sentinel} />
              <Show when={isLoading()}>
                <Typography>{t("common.loading")}</Typography>
              </Show>
            </ul>
          </Match>
          <Match when={!isLoading() && allSeries().length === 0}>
            <Typography>{t("media.no_results")}</Typography>
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

        <For each={filterItems()}>
          {({ ext, categories, isSelected, toggle }) => (
            <div class="flex flex-col gap-2">
              <Show when={filterItems().length > 1}>
                <Typography>{ext.metadata.name ?? ext.filePath}</Typography>
              </Show>
              <Switch fallback={<Typography>{t("filters.empty")}</Typography>}>
                <Match when={categories.loading}>
                  <Typography>{t("common.loading")}</Typography>
                </Match>
                <Match when={categories.error}>
                  <Typography>{categories.error.message}</Typography>
                </Match>
                <Match when={categories()}>
                  {(cats) => (
                    <FilterCategoryList categories={cats()}>
                      {(category, filter) => (
                        <FilterItem
                          filter={filter}
                          checked={isSelected(category.id, filter.id)}
                          onChange={() => toggle(category.id, filter.id)}
                        />
                      )}
                    </FilterCategoryList>
                  )}
                </Match>
              </Switch>
            </div>
          )}
        </For>
      </SidebarLayout.Sidebar>
    </SidebarLayout>
  );
}
