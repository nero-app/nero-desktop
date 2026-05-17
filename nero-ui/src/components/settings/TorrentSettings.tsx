import { t } from "../../lib/i18n";
import { useExtensionPreferences } from "../../providers/ExtensionPreferencesProvider";
import { Button } from "../ui/Button";
import { SectionTable } from "../ui/SectionTable";
import { Toggle } from "../ui/Toggle";
import { Typography } from "../ui/Typography";
import { setMediaProxyPreferences } from "@nero/plugin-extensions";
import { appCacheDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";
import { Show, createResource } from "solid-js";

export default function TorrentSettings() {
  const preferences = useExtensionPreferences();
  const [defaultFolder] = createResource(appCacheDir, { initialValue: "" });

  async function handleToggle(value: boolean) {
    await setMediaProxyPreferences({
      torrentEnabled: value,
      torrentOutputFolder:
        preferences().mediaProxy?.torrentOutputFolder ?? defaultFolder(),
    });
  }

  async function selectOutputFolder() {
    const dir = await open({
      title: t("settings.streaming.torrent.output_folder_title"),
      directory: true,
    });
    if (dir) {
      await setMediaProxyPreferences({
        torrentEnabled: preferences().mediaProxy?.torrentEnabled ?? false,
        torrentOutputFolder: dir,
      });
    }
  }

  async function resetOutputFolder() {
    await setMediaProxyPreferences({
      torrentEnabled: preferences().mediaProxy?.torrentEnabled ?? false,
      torrentOutputFolder: defaultFolder(),
    });
  }

  return (
    <SectionTable>
      <SectionTable.Header
        title={t("settings.streaming.torrent.title")}
        description={t("settings.streaming.torrent.description")}
      />
      <SectionTable.Content class="flex flex-col gap-4">
        <div class="flex items-center justify-between">
          <div class="min-w-0">
            <Typography variant="h4">
              {t("settings.streaming.torrent.enable_label")}
            </Typography>
            <Typography variant="subtitle" class="truncate">
              {t("settings.streaming.torrent.enable_description")}
            </Typography>
          </div>
          <Toggle
            checked={preferences().mediaProxy?.torrentEnabled ?? false}
            onChange={handleToggle}
          />
        </div>

        <Show when={preferences().mediaProxy?.torrentEnabled}>
          <div class="flex items-center justify-between gap-4">
            <div class="min-w-0">
              <Typography variant="h4">
                {t("settings.streaming.torrent.output_folder_label")}
              </Typography>
              <Typography variant="subtitle" class="truncate">
                {preferences().mediaProxy?.torrentOutputFolder ||
                  defaultFolder()}
              </Typography>
            </div>
            <div class="flex shrink-0 gap-2">
              <Button variant="outline" size="sm" onClick={selectOutputFolder}>
                <Typography as="span">{t("common.change")}</Typography>
              </Button>
              <Show when={preferences().mediaProxy?.torrentOutputFolder}>
                <Button variant="outline" size="sm" onClick={resetOutputFolder}>
                  <Typography as="span">{t("common.reset")}</Typography>
                </Button>
              </Show>
            </div>
          </div>
        </Show>
      </SectionTable.Content>
    </SectionTable>
  );
}
