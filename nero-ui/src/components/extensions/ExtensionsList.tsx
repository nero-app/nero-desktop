import { t } from "../../lib/i18n";
import { useExtensionPreferences } from "../../providers/ExtensionPreferencesProvider";
import { Button } from "../ui/Button";
import { dialogManager } from "../ui/DialogManager";
import { SectionTable } from "../ui/SectionTable";
import { Typography } from "../ui/Typography";
import { ExtensionCard } from "./ExtensionCard";
import { ExtensionInfoDialog } from "./ExtensionInfoDialog";
import { ExtensionLoadDialog } from "./ExtensionLoadDialog";
import {
  getExtensionMetadata,
  type ExtensionPreferences,
  type Metadata,
} from "@nero/plugin-extensions";
import { open } from "@tauri-apps/plugin-dialog";
import { BlocksIcon } from "lucide-solid";
import { Show, createResource } from "solid-js";

export default function ExtensionsList() {
  const extensionPreferences = useExtensionPreferences();

  const [metadata] = createResource(
    () => extensionPreferences().extension?.filePath,
    (filePath) => getExtensionMetadata(filePath) as Promise<Metadata>,
  );

  const extensionReady = () => {
    const extension = extensionPreferences().extension;
    const meta = metadata();
    if (!extension || !meta) return undefined;
    return { extension, metadata: meta };
  };

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

  function showExtensionInfo(
    preferences: ExtensionPreferences,
    metadata: Metadata,
  ) {
    dialogManager.show((props) => (
      <ExtensionInfoDialog
        preferences={preferences}
        metadata={metadata}
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
          when={extensionReady()}
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
          {(ready) => (
            <ExtensionCard
              preferences={ready().extension}
              metadata={ready().metadata}
              onClick={() =>
                showExtensionInfo(ready().extension, ready().metadata)
              }
            />
          )}
        </Show>
      </SectionTable.Content>

      <SectionTable.Footer>
        <Typography variant="caption">
          {t("settings.extensions.single_notice")}
        </Typography>
      </SectionTable.Footer>
    </SectionTable>
  );
}
