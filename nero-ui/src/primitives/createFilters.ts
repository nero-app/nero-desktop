import { t } from "../lib/i18n";
import { appState } from "../store/appState";
import { createResource } from "solid-js";

export function createFilters() {
  const [filterCategories] = createResource(() => {
    const extension = appState.extension;
    if (!extension) throw new Error(t("common.no_extension"));
    return extension.getFilters();
  });

  return { filterCategories };
}
