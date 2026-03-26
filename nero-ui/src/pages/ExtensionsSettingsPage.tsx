import ExtensionsList from "../components/extensions/ExtensionsList";
import { t } from "../lib/i18n";

export default function ExtensionsPage() {
  return (
    <div class="flex flex-col gap-4 px-8 py-4">
      <header>
        <h1 class="text-xl font-semibold text-neutral-900">
          {t("settings.extensions.title")}
        </h1>
        <p class="text-neutral-600">{t("settings.extensions.subtitle")}</p>
      </header>
      <ExtensionsList />
    </div>
  );
}
