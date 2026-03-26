import { t } from "../../lib/i18n";
import { createMutation } from "../../primitives/createMutation";
import { setAppState } from "../../store/appState";
import { Button } from "../ui/Button";
import { Dialog } from "../ui/Dialog";
import { Extension, type Metadata } from "@nero/plugin-extensions";
import {
  Switch,
  Match,
  createResource,
  type ComponentProps,
  splitProps,
  Show,
} from "solid-js";

type ExtensionDialogProps = ComponentProps<typeof Dialog> & {
  filePath: string;
  readOnly?: boolean;
};

function MetaItem(props: {
  label: string;
  value?: string | null;
  isLink?: boolean;
}) {
  return (
    <Show when={props.value}>
      <div class="flex flex-col gap-2">
        <span class="font-semibold">{props.label}</span>
        <Show
          when={props.isLink}
          fallback={<span class="truncate">{props.value}</span>}
        >
          <a
            href={props.value!}
            target="_blank"
            rel="noopener noreferrer"
            class="truncate text-blue-500 underline"
          >
            {props.value}
          </a>
        </Show>
      </div>
    </Show>
  );
}

function ProducersItem(props: {
  label: string;
  value?: [string, Record<string, string>][];
}) {
  const formatted = () =>
    props.value
      ?.map(([tool, fields]) => {
        const detail = Object.entries(fields)
          .map(([k, v]) => `${k}: ${v}`)
          .join(", ");
        return `${tool} (${detail})`;
      })
      .join(` ${t("common.separator")} `);

  return <MetaItem label={props.label} value={formatted()} />;
}

export default function ExtensionDialog(props: ExtensionDialogProps) {
  const [local, dialogProps] = splitProps(props, ["filePath", "readOnly"]);

  const [metadata] = createResource(
    () => local.filePath,
    (file) => Extension.getMetadata(file) as Promise<Metadata>,
  );

  const [loadMutation, mutate] = createMutation(async (filePath: string) => {
    const loadedExtension = await Extension.load(filePath);
    setAppState("extension", loadedExtension);
    dialogProps.onOpenChange?.(false);
    return loadedExtension;
  });

  const dialogTitle = () => {
    if (metadata.loading) return t("common.loading");
    const m = metadata();
    return m?.name ?? t("settings.extensions.meta.fallback_title");
  };

  return (
    <Dialog {...dialogProps}>
      <Dialog.Header title={dialogTitle()} />

      <Dialog.Content class="p-4">
        <span class="text-gray-500">{local.filePath}</span>

        <Switch>
          <Match when={metadata.loading}>
            <div class="flex items-center justify-center py-8">
              <span class="animate-pulse text-gray-500">
                {t("common.loading")}
              </span>
            </div>
          </Match>

          <Match when={metadata.error}>
            <p class="text-red-500">
              {metadata.error?.message ?? t("common.error_hint")}
            </p>
          </Match>

          <Match when={metadata()}>
            {(m) => (
              <div class="flex flex-col gap-4">
                <Show when={m().description}>
                  <p class="text-gray-600">{m().description}</p>
                </Show>

                <div class="grid grid-cols-2 gap-3 pt-2 text-sm">
                  <MetaItem
                    label={t("settings.extensions.meta.version")}
                    value={m().version}
                  />
                  <MetaItem
                    label={t("settings.extensions.meta.revision")}
                    value={m().revision}
                  />
                  <MetaItem
                    label={t("settings.extensions.meta.authors")}
                    value={m().authors}
                  />
                  <MetaItem
                    label={t("settings.extensions.meta.licenses")}
                    value={m().licenses}
                  />
                  <MetaItem
                    label={t("settings.extensions.meta.homepage")}
                    value={m().homepage}
                    isLink
                  />
                  <MetaItem
                    label={t("settings.extensions.meta.source")}
                    value={m().source}
                    isLink
                  />
                  <MetaItem
                    label={t("settings.extensions.meta.range")}
                    value={
                      m().range
                        ? `${m().range!.start} - ${m().range!.end}`
                        : undefined
                    }
                  />
                  <ProducersItem
                    label={t("settings.extensions.meta.producers")}
                    value={m().producers}
                  />
                  <Show when={m().dependencies?.length}>
                    <MetaItem
                      label={t("settings.extensions.meta.dependencies")}
                      value={m().dependencies?.join(", ")}
                    />
                  </Show>
                </div>
              </div>
            )}
          </Match>
        </Switch>

        <Show when={loadMutation.error}>
          <p class="text-red-500">
            {loadMutation.error?.message ?? t("common.error_hint")}
          </p>
        </Show>
      </Dialog.Content>

      <Show when={!local.readOnly}>
        <Dialog.Footer>
          <Button
            class="w-full"
            onClick={() => mutate(local.filePath)}
            disabled={metadata.loading || loadMutation.loading}
          >
            {loadMutation.loading
              ? t("common.loading")
              : t("settings.extensions.load")}
          </Button>
        </Dialog.Footer>
      </Show>
    </Dialog>
  );
}
