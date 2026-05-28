import { t } from "../../lib/i18n";
import { Button } from "../ui/Button";
import { SectionTable } from "../ui/SectionTable";
import { Toggle } from "../ui/Toggle";
import { Typography } from "../ui/Typography";
import {
  getMediaProxyPreferences,
  setMediaProxyPreferences,
} from "@nero/plugin-extensions";
import { appCacheDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";
import { Show, createResource } from "solid-js";

export default function TorrentSettings() {
  const [preferences, { mutate }] = createResource(() =>
    getMediaProxyPreferences(),
  );
  const [defaultFolder] = createResource(appCacheDir, { initialValue: "" });

  async function handleToggle(value: boolean) {
    const updated = {
      torrentEnabled: value,
      torrentOutputFolder:
        preferences()?.torrentOutputFolder ?? defaultFolder(),
    };
    await setMediaProxyPreferences(updated);
    mutate(updated);
  }

  async function selectOutputFolder() {
    const dir = await open({
      title: t("settings.streaming.torrent.output_folder_title"),
      directory: true,
    });
    if (dir) {
      const updated = {
        torrentEnabled: preferences()?.torrentEnabled ?? false,
        torrentOutputFolder: dir,
      };
      await setMediaProxyPreferences(updated);
      mutate(updated);
    }
  }

  async function resetOutputFolder() {
    const updated = {
      torrentEnabled: preferences()?.torrentEnabled ?? false,
      torrentOutputFolder: defaultFolder(),
    };
    await setMediaProxyPreferences(updated);
    mutate(updated);
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
            checked={preferences()?.torrentEnabled}
            onChange={handleToggle}
          />
        </div>

        <Show when={preferences()?.torrentEnabled}>
          <div class="flex items-center justify-between gap-4">
            <div class="min-w-0">
              <Typography variant="h4">
                {t("settings.streaming.torrent.output_folder_label")}
              </Typography>
              <Typography variant="subtitle" class="truncate">
                {preferences()?.torrentOutputFolder || defaultFolder()}
              </Typography>
            </div>
            <div class="flex shrink-0 gap-2">
              <Button variant="outline" size="sm" onClick={selectOutputFolder}>
                <Typography as="span">{t("common.change")}</Typography>
              </Button>
              <Show when={preferences()?.torrentOutputFolder}>
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
