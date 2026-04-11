import { t } from "../../lib/i18n";
import { Dialog } from "../ui/Dialog";
import { Typography } from "../ui/Typography";
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

        <hr class="h-full border border-neutral-200" />

        <section class="flex flex-col gap-4">
          <Typography variant="h3">
            {t("settings.extensions.options.title")}
          </Typography>

          <div class="flex flex-col gap-2">
            <Typography variant="subtitle">
              {t("settings.extensions.options.cache_dir")}
            </Typography>
            <Typography as="code">{options().cacheDir}</Typography>
          </div>

          <div class="flex flex-col gap-2">
            <Typography variant="subtitle">
              {t("settings.extensions.options.max_cache_size")}
            </Typography>
            <Typography>
              {options().maxCacheSize
                ? `${options().maxCacheSize! / 1024 / 1024} MB`
                : "0 MB"}
            </Typography>
          </div>
        </section>
      </Dialog.Content>
    </Dialog>
  );
}
