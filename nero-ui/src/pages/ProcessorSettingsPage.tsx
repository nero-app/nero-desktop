import TorrentSettings from "../components/settings/TorrentSettings";
import { Typography } from "../components/ui/Typography";
import { t } from "../lib/i18n";

export default function ProcessorSettingsPage() {
  return (
    <div class="flex flex-col gap-4 px-8 py-4">
      <header>
        <Typography variant="h1">{t("settings.processor.title")}</Typography>
        <Typography variant="subtitle">
          {t("settings.processor.subtitle")}
        </Typography>
      </header>
      <TorrentSettings />
    </div>
  );
}
