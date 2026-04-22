import { t } from "../../lib/i18n";
import { createExtensionLoader } from "../../primitives/createExtensionLoader";
import { Button } from "../ui/Button";
import { Dialog } from "../ui/Dialog";
import { Input } from "../ui/Input";
import { Typography } from "../ui/Typography";
import { ExtensionMetaPanel } from "./ExtensionMetaPanel";
import {
  Extension,
  MAX_CACHE_SIZE_MB,
  type Metadata,
} from "@nero/plugin-extensions";
import { open } from "@tauri-apps/plugin-dialog";
import {
  createResource,
  Show,
  type ComponentProps,
  splitProps,
  ErrorBoundary,
} from "solid-js";

type ExtensionLoadDialogProps = ComponentProps<typeof Dialog> & {
  filePath: string;
};

export function ExtensionLoadDialog(props: ExtensionLoadDialogProps) {
  const [local, dialogProps] = splitProps(props, ["filePath"]);

  const [metadata] = createResource(
    () => local.filePath,
    (file) => Extension.getMetadata(file) as Promise<Metadata>,
  );

  const { cache, load } = createExtensionLoader(() => local.filePath);

  const pickCacheDir = async () => {
    const selected = await open({ directory: true, multiple: false });
    if (selected) cache.setDir(selected);
  };

  const title = () => {
    if (metadata.loading) return t("common.loading");
    return metadata()?.name ?? t("settings.extensions.meta.fallback_title");
  };

  return (
    <Dialog {...dialogProps}>
      <Dialog.Header title={title()} />

      <Dialog.Content class="grid grid-cols-[1fr_auto_1fr] gap-4 p-4">
        <ErrorBoundary
          fallback={(err) => <Typography>{err.message}</Typography>}
        >
          <Show when={metadata()}>
            {(metadata) => (
              <ExtensionMetaPanel
                filePath={local.filePath}
                metadata={metadata()}
              />
            )}
          </Show>
        </ErrorBoundary>

        <hr class="h-full border border-neutral-200" />

        <section class="flex flex-col gap-4">
          <Typography variant="h3">
            {t("settings.extensions.options.title")}
          </Typography>

          <div class="flex flex-col gap-2">
            <Typography variant="subtitle" as="label">
              {t("settings.extensions.options.cache_dir")}
            </Typography>
            <div class="flex gap-2">
              <Input
                class="px-3 py-2"
                readOnly
                placeholder={t(
                  "settings.extensions.options.cache_dir_placeholder",
                )}
                value={cache.dir() ?? ""}
              />
              <Button variant="outline" onClick={pickCacheDir}>
                <Typography as="span">{t("common.browse")}</Typography>
              </Button>
            </div>
          </div>

          <div class="flex flex-col gap-2">
            <Typography variant="subtitle" as="label">
              {t("settings.extensions.options.max_cache_size")}
            </Typography>
            <Input
              class="px-3 py-2"
              type="number"
              min={0}
              max={MAX_CACHE_SIZE_MB}
              value={cache.maxSize()}
              onInput={(e) => cache.setMaxSize(Number(e.currentTarget.value))}
            />
            <Show
              when={cache.isExceeded()}
              fallback={
                <Typography variant="caption">
                  {t("settings.extensions.options.max_cache_size_hint")} (max{" "}
                  {MAX_CACHE_SIZE_MB} MB)
                </Typography>
              }
            >
              <Typography variant="caption">
                {t("settings.extensions.options.max_cache_size_exceeded")}
              </Typography>
            </Show>
          </div>

          <Show when={load.mutation.error}>
            <Typography class="text-red-500">
              {load.mutation.error?.message ?? t("common.error_hint")}
            </Typography>
          </Show>

          <Button
            class="mt-auto w-full"
            onClick={() =>
              load.trigger().then(() => dialogProps.onOpenChange?.(false))
            }
            disabled={
              cache.dir.loading ||
              !cache.dir() ||
              cache.isExceeded() ||
              metadata.loading ||
              load.mutation.loading
            }
          >
            <Typography as="span">
              {load.mutation.loading
                ? t("common.loading")
                : t("settings.extensions.load")}
            </Typography>
          </Button>
        </section>
      </Dialog.Content>
    </Dialog>
  );
}
