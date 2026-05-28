import { t } from "../../lib/i18n";
import { Button } from "../ui/Button";
import { dialogManager } from "../ui/DialogManager";
import { SectionTable } from "../ui/SectionTable";
import { Typography } from "../ui/Typography";
import { ExtensionCard } from "./ExtensionCard";
import { ExtensionInfoDialog } from "./ExtensionInfoDialog";
import { ExtensionLoadDialog } from "./ExtensionLoadDialog";
import {
  getLoadedExtensions,
  unloadExtension,
  type LoadedExtension,
} from "@nero/plugin-extensions";
import { open } from "@tauri-apps/plugin-dialog";
import { BlocksIcon } from "lucide-solid";
import { For } from "solid-js";
import { Show, createResource } from "solid-js";

export default function ExtensionsList() {
  const [loadedExtensions, { refetch }] = createResource(() =>
    getLoadedExtensions(),
  );

  async function handleUnload(filePath: string) {
    await unloadExtension(filePath);
    refetch();
  }

  async function selectExtension() {
    const file = await open({
      filters: [{ name: "Extension", extensions: ["wasm"] }],
    });

    if (file) {
      dialogManager.show((props) => (
        <ExtensionLoadDialog
          filePath={file}
          open={props.open}
          onOpenChange={props.onOpenChange}
        />
      ));
    }
  }

  function showExtensionInfo(extension: LoadedExtension) {
    dialogManager.show((props) => (
      <ExtensionInfoDialog
        extension={extension}
        open={props.open}
        onOpenChange={props.onOpenChange}
      />
    ));
  }

  return (
    <SectionTable>
      <SectionTable.Header title={t("settings.extensions.loaded_label")}>
        <Button variant="outline" size="sm" onClick={selectExtension}>
          <Typography as="span">{t("settings.extensions.load")}</Typography>
        </Button>
      </SectionTable.Header>

      <SectionTable.Content>
        <Show
          when={loadedExtensions()?.length}
          fallback={
            <div class="flex flex-col items-center gap-2 text-center">
              <BlocksIcon class="size-10 text-neutral-300" />
              <Typography>{t("settings.extensions.status_idle")}</Typography>
              <Button variant="outline" size="sm" onClick={selectExtension}>
                <Typography as="span">
                  {t("settings.extensions.load")}
                </Typography>
              </Button>
            </div>
          }
        >
          <For each={loadedExtensions()}>
            {(extension) => (
              <ExtensionCard
                extension={extension}
                onClick={() => showExtensionInfo(extension)}
                onUnload={() => handleUnload(extension.id)}
              />
            )}
          </For>
        </Show>
      </SectionTable.Content>
    </SectionTable>
  );
}
