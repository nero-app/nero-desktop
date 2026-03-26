import {
  changeLocale,
  languages,
  locale,
  t,
  type Locale,
} from "../../lib/i18n";
import { SectionTable } from "../ui/SectionTable";
import { Select } from "@kobalte/core/select";
import { CheckIcon, ChevronsUpDownIcon } from "lucide-solid";

type Language = { locale: Locale; label: string };

export default function LanguageSettings() {
  const current = () => languages.find((l) => l.locale === locale())!;

  return (
    <SectionTable>
      <SectionTable.Header
        title={t("settings.app.language.title")}
        description={t("settings.app.language.description")}
      />
      <SectionTable.Content class="flex items-center justify-between gap-4">
        <p class="font-medium text-neutral-900">
          {t("settings.app.language.select_label")}
        </p>
        <Select<Language>
          options={languages}
          optionValue="locale"
          optionTextValue="label"
          value={current()}
          onChange={(lang) => lang && changeLocale(lang.locale)}
          itemComponent={(props) => (
            <Select.Item
              item={props.item}
              class="flex cursor-pointer items-center justify-between gap-6
                rounded-md px-3 py-2 text-sm text-neutral-700 outline-none
                hover:bg-neutral-100 data-highlighted:bg-neutral-100"
            >
              <Select.ItemLabel>{props.item.rawValue.label}</Select.ItemLabel>
              <Select.ItemIndicator>
                <CheckIcon size={14} class="text-orange-400" />
              </Select.ItemIndicator>
            </Select.Item>
          )}
        >
          <Select.Trigger
            class="flex items-center gap-2 rounded-md border border-neutral-300
              px-3 py-2 text-sm text-neutral-700 transition-colors
              hover:border-neutral-400 hover:bg-neutral-100"
          >
            <Select.Value<Language>>
              {(state) => state.selectedOption().label}
            </Select.Value>
            <ChevronsUpDownIcon size={14} class="text-neutral-400" />
          </Select.Trigger>
          <Select.Portal>
            <Select.Content
              class="rounded-md border border-neutral-200 bg-white p-1
                shadow-md"
            >
              <Select.Listbox />
            </Select.Content>
          </Select.Portal>
        </Select>
      </SectionTable.Content>
    </SectionTable>
  );
}
