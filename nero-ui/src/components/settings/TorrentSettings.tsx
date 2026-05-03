import { t } from "../../lib/i18n";
import { useExtensionStatus } from "../../providers/ExtensionProvider";
import { Button } from "../ui/Button";
import { SectionTable } from "../ui/SectionTable";
import { Toggle } from "../ui/Toggle";
import { Typography } from "../ui/Typography";
import {
  disableTorrentSupport,
  enableTorrentSupport,
} from "@nero/plugin-extensions";
import { appCacheDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";
import { Show, createResource } from "solid-js";

export default function TorrentSettings() {
  const status = useExtensionStatus();
  const [defaultFolder] = createResource(appCacheDir);

  // TODO: Handle errors
  async function handleToggle(value: boolean) {
    if (value) {
      await enableTorrentSupport(
        status().torrentOutputFolder ?? (await appCacheDir()),
      );
    } else await disableTorrentSupport();
  }

  // TODO: Handle errors
  async function selectOutputFolder() {
    const dir = await open({
      title: t("settings.app.torrent.output_folder_title"),
      directory: true,
    });
    if (dir) await enableTorrentSupport(dir);
  }

  // TODO: Handle errors
  async function resetOutputFolder() {
    await enableTorrentSupport(await appCacheDir());
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
          <Toggle checked={status().torrentEnabled} onChange={handleToggle} />
        </div>
        <Show when={status().torrentEnabled}>
          <div class="flex items-center justify-between gap-4">
            <div class="min-w-0">
              <Typography variant="h4">
                {t("settings.app.torrent.output_folder_label")}
              </Typography>
              <Typography variant="subtitle" class="truncate">
                {status().torrentOutputFolder || defaultFolder()}
              </Typography>
            </div>
            <div class="flex shrink-0 gap-2">
              <Button variant="outline" size="sm" onClick={selectOutputFolder}>
                <Typography as="span">{t("common.change")}</Typography>
              </Button>

              <Show when={status().torrentOutputFolder}>
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
