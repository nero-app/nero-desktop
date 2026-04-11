import { t } from "../../lib/i18n";
import { appState, setAppState } from "../../store/appState";
import { Button } from "../ui/Button";
import { SectionTable } from "../ui/SectionTable";
import { Toggle } from "../ui/Toggle";
import { Typography } from "../ui/Typography";
import {
  enableTorrentSupport,
  disableTorrentSupport,
} from "@nero/plugin-extensions";
import { appCacheDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";
import { Show, createResource } from "solid-js";

export default function TorrentSettings() {
  const enabled = () => appState.config.torrentEnabled;
  const outputFolder = () => appState.config.torrentOutputFolder;
  const [defaultFolder] = createResource(appCacheDir);

  async function applyFolder(path: string) {
    await disableTorrentSupport();
    await enableTorrentSupport(path);
  }

  async function handleToggle(value: boolean) {
    setAppState("config", "torrentEnabled", value);
    if (value) {
      await enableTorrentSupport(outputFolder() || (await appCacheDir()));
    } else {
      await disableTorrentSupport();
    }
  }

  async function selectOutputFolder() {
    const dir = await open({
      title: t("settings.app.torrent.output_folder_title"),
      directory: true,
    });
    if (dir) {
      setAppState("config", "torrentOutputFolder", dir);
      if (enabled()) await applyFolder(dir);
    }
  }

  async function resetOutputFolder() {
    setAppState("config", "torrentOutputFolder", null);
    if (enabled()) await applyFolder(await appCacheDir());
  }

  return (
    <SectionTable>
      <SectionTable.Header
        title={t("settings.app.torrent.title")}
        description={t("settings.app.torrent.description")}
      />
      <SectionTable.Content class="flex flex-col gap-4">
        <div class="flex items-center justify-between">
          <div class="min-w-0">
            <Typography variant="h4">
              {t("settings.app.torrent.enable_label")}
            </Typography>
            <Typography variant="subtitle" class="truncate">
              {t("settings.app.torrent.enable_description")}
            </Typography>
          </div>
          <Toggle checked={enabled()} onChange={handleToggle} />
        </div>
        <Show when={enabled()}>
          <div class="flex items-center justify-between gap-4">
            <div class="min-w-0">
              <Typography variant="h4">
                {t("settings.app.torrent.output_folder_label")}
              </Typography>
              <Typography variant="subtitle" class="truncate">
                {outputFolder() || defaultFolder()}
              </Typography>
            </div>
            <div class="flex shrink-0 gap-2">
              <Button variant="outline" size="sm" onClick={selectOutputFolder}>
                <Typography as="span">{t("common.change")}</Typography>
              </Button>

              <Show when={outputFolder()}>
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
