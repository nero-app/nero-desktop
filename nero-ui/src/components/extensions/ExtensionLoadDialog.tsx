import { t } from "../../lib/i18n";
import { createMutation } from "../../primitives/createMutation";
import { setAppState } from "../../store/appState";
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
  createSignal,
  Show,
  type ComponentProps,
  splitProps,
} from "solid-js";

type ExtensionLoadDialogProps = ComponentProps<typeof Dialog> & {
  filePath: string;
};

export function ExtensionLoadDialog(props: ExtensionLoadDialogProps) {
  const [local, dialogProps] = splitProps(props, ["filePath"]);

  const [cacheDir, setCacheDir] = createSignal<string | null>(null);
  const [maxCacheSize, setMaxCacheSize] = createSignal<string>("0");

  const [metadata] = createResource(
    () => local.filePath,
    (file) => Extension.getMetadata(file) as Promise<Metadata>,
  );

  const pickCacheDir = async () => {
    const selected = await open({ directory: true, multiple: false });
    if (selected) setCacheDir(selected);
  };

  const isMaxCacheExceeded = () => {
    const raw = maxCacheSize().trim();
    return raw ? Number(raw) > MAX_CACHE_SIZE_MB : false;
  };

  const maxCacheSizeBytes = () => {
    const raw = maxCacheSize().trim();
    const mb = Number(raw);
    return raw && mb > 0 ? mb * 1024 * 1024 : undefined;
  };

  const [loadMutation, mutate] = createMutation(async () => {
    const loadedExtension = await Extension.load(local.filePath, {
      cacheDir: cacheDir()!,
      maxCacheSize: maxCacheSizeBytes(),
    });
    setAppState("extension", loadedExtension);
    dialogProps.onOpenChange?.(false);
    return loadedExtension;
  });

  const title = () => {
    if (metadata.loading) return t("common.loading");
    return metadata()?.name ?? t("settings.extensions.meta.fallback_title");
  };

  return (
    <Dialog {...dialogProps}>
      <Dialog.Header title={title()} />

      <Dialog.Content class="grid grid-cols-[1fr_auto_1fr] gap-4 p-4">
        <ExtensionMetaPanel filePath={local.filePath} metadata={metadata} />

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
                value={cacheDir() ?? ""}
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
              value={maxCacheSize()}
              onInput={(e) => setMaxCacheSize(e.currentTarget.value)}
            />
            <Show
              when={isMaxCacheExceeded()}
              fallback={
                <Typography variant="caption">
                  {t("settings.extensions.options.max_cache_size_hint")} (max{" "}
                  {MAX_CACHE_SIZE_MB} MB)
                </Typography>
              }
            >
              {/* TODO: error variant */}
              <Typography variant="caption">
                {t("settings.extensions.options.max_cache_size_exceeded")}
              </Typography>
            </Show>
          </div>

          <Show when={loadMutation.error}>
            <Typography class="text-red-500">
              {loadMutation.error?.message ?? t("common.error_hint")}
            </Typography>
          </Show>

          <Button
            class="mt-auto w-full"
            onClick={() => mutate(local.filePath)}
            disabled={
              !cacheDir() ||
              isMaxCacheExceeded() ||
              metadata.loading ||
              loadMutation.loading
            }
          >
            <Typography as="span">
              {loadMutation.loading
                ? t("common.loading")
                : t("settings.extensions.load")}
            </Typography>
          </Button>
        </section>
      </Dialog.Content>
    </Dialog>
  );
}
