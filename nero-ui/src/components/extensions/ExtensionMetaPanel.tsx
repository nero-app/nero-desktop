import { t } from "../../lib/i18n";
import { type Metadata } from "@nero/plugin-extensions";
import { Switch, Match, Show } from "solid-js";
import type { Resource } from "solid-js";

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
          fallback={<span class="truncate text-sm">{props.value}</span>}
        >
          <a
            href={props.value!}
            target="_blank"
            rel="noopener noreferrer"
            class="truncate text-sm text-blue-500 underline"
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

export function ExtensionMetaPanel(props: {
  filePath: string;
  metadata: Resource<Metadata>;
}) {
  return (
    <div class="flex flex-col gap-4">
      <span>{props.filePath}</span>

      <Switch>
        <Match when={props.metadata.loading}>
          <div class="flex items-center justify-center py-8">
            <span class="animate-pulse text-gray-500">
              {t("common.loading")}
            </span>
          </div>
        </Match>

        <Match when={props.metadata.error}>
          <p class="text-red-500">
            {props.metadata.error?.message ?? t("common.error_hint")}
          </p>
        </Match>

        <Match when={props.metadata()}>
          {(m) => (
            <div class="flex flex-col gap-3">
              <Show when={m().description}>
                <p class="text-gray-600">{m().description}</p>
              </Show>
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
          )}
        </Match>
      </Switch>
    </div>
  );
}
