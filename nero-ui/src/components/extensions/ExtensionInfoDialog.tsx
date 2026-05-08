import { t } from "../../lib/i18n";
import { Dialog } from "../ui/Dialog";
import { Typography } from "../ui/Typography";
import { ExtensionMetaPanel } from "./ExtensionMetaPanel";
import {
  type ExtensionPreferences,
  type Metadata,
} from "@nero/plugin-extensions";
import { type ComponentProps, splitProps, ErrorBoundary } from "solid-js";

type ExtensionInfoDialogProps = ComponentProps<typeof Dialog> & {
  preferences: ExtensionPreferences;
  metadata: Metadata;
};

export function ExtensionInfoDialog(props: ExtensionInfoDialogProps) {
  const [local, dialogProps] = splitProps(props, ["preferences", "metadata"]);

  return (
    <Dialog {...dialogProps}>
      <Dialog.Header
        title={
          local.metadata.name ?? t("settings.extensions.meta.fallback_title")
        }
      />

      <Dialog.Content class="grid grid-cols-[1fr_auto_1fr] gap-4 p-4">
        <ErrorBoundary
          fallback={(err) => <Typography>{err.message}</Typography>}
        >
          <ExtensionMetaPanel
            filePath={local.preferences.filePath}
            metadata={local.metadata}
          />
        </ErrorBoundary>

        <hr class="h-full border border-neutral-200" />

        <section class="flex flex-col gap-4">
          <Typography variant="h3">
            {t("settings.extensions.options.title")}
          </Typography>

          <div class="flex flex-col gap-2">
            <Typography variant="subtitle">
              {t("settings.extensions.options.cache_dir")}
            </Typography>
            <Typography as="code">{local.preferences.cacheDir}</Typography>
          </div>

          <div class="flex flex-col gap-2">
            <Typography variant="subtitle">
              {t("settings.extensions.options.max_cache_size")}
            </Typography>
            <Typography>
              {local.preferences.maxCacheSize
                ? `${local.preferences.maxCacheSize! / 1024 / 1024} MB`
                : "0 MB"}
            </Typography>
          </div>
        </section>
      </Dialog.Content>
    </Dialog>
  );
}
