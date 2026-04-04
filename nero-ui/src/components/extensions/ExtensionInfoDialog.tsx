import { t } from "../../lib/i18n";
import { Dialog } from "../ui/Dialog";
import { ExtensionMetaPanel } from "./ExtensionMetaPanel";
import { Extension, type Metadata } from "@nero/plugin-extensions";
import { createResource, type ComponentProps, splitProps } from "solid-js";

type ExtensionInfoDialogProps = ComponentProps<typeof Dialog> & {
  extension: Extension;
};

export function ExtensionInfoDialog(props: ExtensionInfoDialogProps) {
  const [local, dialogProps] = splitProps(props, ["extension"]);

  const [metadata] = createResource(
    () => local.extension.filePath,
    (file) => Extension.getMetadata(file) as Promise<Metadata>,
  );

  const options = () => local.extension.options;

  return (
    <Dialog {...dialogProps}>
      <Dialog.Header
        title={
          local.extension.metadata.name ??
          t("settings.extensions.meta.fallback_title")
        }
      />

      <Dialog.Content class="grid grid-cols-[1fr_auto_1fr] gap-4 p-4">
        <ExtensionMetaPanel
          filePath={local.extension.filePath}
          metadata={metadata}
        />

        <div class="border border-neutral-200" />

        <div class="flex flex-col gap-4">
          <p class="font-semibold">{t("settings.extensions.options.title")}</p>

          <div class="flex flex-col gap-2">
            <span class="text-sm font-medium">
              {t("settings.extensions.options.cache_dir")}
            </span>
            <span class="truncate text-sm">{options().cacheDir}</span>
          </div>

          <div class="flex flex-col gap-2">
            <span class="text-sm font-medium">
              {t("settings.extensions.options.max_cache_size")}
            </span>
            <span class="text-sm">
              {options().maxCacheSize
                ? `${options().maxCacheSize! / 1024 / 1024} MB`
                : "0 MB"}
            </span>
          </div>
        </div>
      </Dialog.Content>
    </Dialog>
  );
}
