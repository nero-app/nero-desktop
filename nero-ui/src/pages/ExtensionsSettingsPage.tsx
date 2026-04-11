import ExtensionsList from "../components/extensions/ExtensionsList";
import { Typography } from "../components/ui/Typography";
import { t } from "../lib/i18n";

export default function ExtensionsPage() {
  return (
    <div class="flex flex-col gap-4 px-8 py-4">
      <header>
        <Typography variant="h1">{t("settings.extensions.title")}</Typography>
        <Typography variant="subtitle">
          {t("settings.extensions.subtitle")}
        </Typography>
      </header>
      <ExtensionsList />
    </div>
  );
}
