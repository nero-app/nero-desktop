import { t } from "../../lib/i18n";
import { Typography } from "../ui/Typography";
import { type Metadata } from "@nero/plugin-extensions";
import { Show } from "solid-js";

// TODO: https://v2.tauri.app/plugin/opener/
function MetaItem(props: {
  label: string;
  value?: string | null;
  isLink?: boolean;
}) {
  return (
    <Show when={props.value}>
      <div class="flex flex-col gap-2">
        <Typography variant="subtitle" as="label">
          {props.label}
        </Typography>
        <Show
          when={props.isLink}
          fallback={<Typography class="truncate">{props.value}</Typography>}
        >
          <button>{props.value}</button>
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
  metadata: Metadata;
}) {
  return (
    <aside class="flex flex-col gap-4">
      <Typography as="code">{props.filePath}</Typography>

      <div class="flex flex-col gap-3">
        <Show when={props.metadata.description}>
          <Typography>{props.metadata.description}</Typography>
        </Show>
        <MetaItem
          label={t("settings.extensions.meta.version")}
          value={props.metadata.version}
        />
        <MetaItem
          label={t("settings.extensions.meta.revision")}
          value={props.metadata.revision}
        />
        <MetaItem
          label={t("settings.extensions.meta.authors")}
          value={props.metadata.authors}
        />
        <MetaItem
          label={t("settings.extensions.meta.licenses")}
          value={props.metadata.licenses}
        />
        <MetaItem
          label={t("settings.extensions.meta.homepage")}
          value={props.metadata.homepage}
          isLink
        />
        <MetaItem
          label={t("settings.extensions.meta.source")}
          value={props.metadata.source}
          isLink
        />
        <MetaItem
          label={t("settings.extensions.meta.range")}
          value={
            props.metadata.range
              ? `${props.metadata.range!.start} - ${props.metadata.range!.end}`
              : undefined
          }
        />
        <ProducersItem
          label={t("settings.extensions.meta.producers")}
          value={props.metadata.producers}
        />
        <Show when={props.metadata.dependencies?.length}>
          <MetaItem
            label={t("settings.extensions.meta.dependencies")}
            value={props.metadata.dependencies?.join(", ")}
          />
        </Show>
      </div>
    </aside>
  );
}
