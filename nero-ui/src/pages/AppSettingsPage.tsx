import LanguageSettings from "../components/settings/LanguageSettings";
import PlayerSettings from "../components/settings/PlayerSettings";
import { Typography } from "../components/ui/Typography";
import { t } from "../lib/i18n";

export default function AppSettingsPage() {
  return (
    <div class="flex flex-col gap-4 p-8">
      <header>
        <Typography variant="h1">{t("settings.app.title")}</Typography>
        <Typography variant="subtitle">{t("settings.app.subtitle")}</Typography>
      </header>
      <LanguageSettings />
      <PlayerSettings />
    </div>
  );
}
