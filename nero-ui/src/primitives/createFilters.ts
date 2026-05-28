import {
  getFilters,
  type LoadedExtension,
  type SearchFilter,
} from "@nero/plugin-extensions";
import {
  createMemo,
  createResource,
  createSignal,
  type Accessor,
} from "solid-js";

export function createFilters(extensionId: Accessor<string>) {
  const [categories] = createResource(extensionId, (id) => getFilters(id));

  const [selected, setSelected] = createSignal<SearchFilter[]>([]);

  function isSelected(categoryId: string, filterId: string) {
    return (
      selected()
        .find((sf) => sf.id === categoryId)
        ?.values.includes(filterId) ?? false
    );
  }

  function toggle(categoryId: string, filterId: string) {
    setSelected((prev) => {
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
    categories,
    selected,
    isSelected,
    toggle,
  };
}

export function createExtensionsFilters(
  extensions: Accessor<LoadedExtension[]>,
) {
  const items = createMemo(() =>
    extensions().map((ext) => ({ ext, ...createFilters(() => ext.id) })),
  );

  const filtersFor =
    (extensionId: string): Accessor<SearchFilter[]> =>
    () =>
      items()
        .find((f) => f.ext.id === extensionId)
        ?.selected() ?? [];

  return { items, filtersFor };
}
