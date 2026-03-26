import LanguageSettings from "../components/settings/LanguageSettings";
import PlayerSettings from "../components/settings/PlayerSettings";
import TorrentSettings from "../components/settings/TorrentSettings";
import { t } from "../lib/i18n";

export default function AppSettingsPage() {
  return (
    <div class="flex flex-col gap-4 px-8 py-4">
      <header>
        <h1 class="text-xl font-semibold text-neutral-900">
          {t("settings.app.title")}
        </h1>
        <p class="text-neutral-600">{t("settings.app.subtitle")}</p>
      </header>
      <LanguageSettings />
      <PlayerSettings />
      <TorrentSettings />
    </div>
  );
}
