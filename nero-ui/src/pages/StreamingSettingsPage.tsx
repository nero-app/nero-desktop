import TorrentSettings from "../components/settings/TorrentSettings";
import { Typography } from "../components/ui/Typography";
import { t } from "../lib/i18n";

export default function StreamingSettingsPage() {
  return (
    <div class="flex flex-col gap-4 p-8">
      <header>
        <Typography variant="h1">{t("settings.streaming.title")}</Typography>
        <Typography variant="subtitle">
          {t("settings.streaming.subtitle")}
        </Typography>
      </header>
      <TorrentSettings />
    </div>
  );
}
