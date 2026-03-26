import type en from "../locales/en.json";
import { flatten, resolveTemplate, translator } from "@solid-primitives/i18n";
import { createRoot, createSignal } from "solid-js";

export type RawDictionary = typeof en;
export type Locale = "en";

export const languages: { locale: Locale; label: string }[] = [
  { locale: "en", label: "English" },
];

const locales: Record<Locale, () => Promise<RawDictionary>> = {
  en: () => import("../locales/en.json"),
};

export function createI18nContext() {
  const [locale, setLocale] = createSignal<Locale>("en");
  const [dict, setDict] = createSignal(flatten({} as RawDictionary));

  locales[locale()]().then((mod) => setDict(flatten(mod)));

  async function changeLocale(next: Locale) {
    const mod = await locales[next]();
    setDict(flatten(mod));
    setLocale(next);
  }

  const t = translator(dict, resolveTemplate);

  return { t, locale, changeLocale };
}

export const { t, locale, changeLocale } = createRoot(() =>
  createI18nContext(),
);
