import { t } from "../../lib/i18n";
import { appState } from "../../store/appState";
import { Button } from "../ui/Button";
import { SectionTable } from "../ui/SectionTable";
import { Typography } from "../ui/Typography";
import { ExtensionCard } from "./ExtensionCard";
import { ExtensionInfoDialog } from "./ExtensionInfoDialog";
import { ExtensionLoadDialog } from "./ExtensionLoadDialog";
import { open } from "@tauri-apps/plugin-dialog";
import { BlocksIcon } from "lucide-solid";
import { Match, Show, Switch, createSignal } from "solid-js";

export default function ExtensionsList() {
  const [selectedFile, setSelectedFile] = createSignal<string | null>(null);
  const [showAddDialog, setShowAddDialog] = createSignal(false);
  const [showInfoDialog, setShowInfoDialog] = createSignal(false);
  const currentExtension = () => appState.extension;

  async function selectExtension() {
    const file = await open({
      filters: [{ name: "Extension", extensions: ["wasm"] }],
    });
    if (file) {
      setSelectedFile(file);
      setShowAddDialog(true);
    }
  }

  function closeDialog() {
    setShowAddDialog(false);
    setSelectedFile(null);
  }

  return (
    <SectionTable>
      <SectionTable.Header title={t("settings.extensions.loaded_label")}>
        <Show when={currentExtension()}>
          <Button variant="outline" size="sm" onClick={selectExtension}>
            <Typography as="span">{t("settings.extensions.load")}</Typography>
          </Button>
        </Show>
      </SectionTable.Header>

      <SectionTable.Content>
        <Show
          when={currentExtension()}
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
          {(ext) => (
            <ExtensionCard
              extension={ext()}
              onClick={() => setShowInfoDialog(true)}
            />
          )}
        </Show>
      </SectionTable.Content>

      <SectionTable.Footer>
        <Typography variant="caption">
          {t("settings.extensions.single_notice")}
        </Typography>
      </SectionTable.Footer>

      <Switch>
        <Match when={showAddDialog() && selectedFile()}>
          <ExtensionLoadDialog
            filePath={selectedFile()!}
            open={showAddDialog()}
            onOpenChange={(open) => {
              if (!open) closeDialog();
            }}
          />
        </Match>
        <Match when={showInfoDialog()}>
          <ExtensionInfoDialog
            extension={currentExtension()!}
            open={showInfoDialog()}
            onOpenChange={(open) => {
              if (!open) setShowInfoDialog(false);
            }}
          />
        </Match>
      </Switch>
    </SectionTable>
  );
}
