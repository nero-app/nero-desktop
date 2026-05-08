import { t } from "../../lib/i18n";
import { Typography } from "../ui/Typography";
import type { ExtensionPreferences, Metadata } from "@nero/plugin-extensions";
import { BlocksIcon } from "lucide-solid";
import { Show } from "solid-js";

type ExtensionCardProps = {
  preferences: ExtensionPreferences;
  metadata: Metadata;
  onClick: () => void;
};

export function ExtensionCard(props: ExtensionCardProps) {
  const subtitle = () => {
    const parts = [
      props.metadata.version ? `v${props.metadata.version}` : null,
      props.metadata.authors ?? null,
      props.metadata.licenses ?? null,
    ].filter(Boolean);
    return parts.length > 0 ? parts.join(` ${t("common.separator")} `) : null;
  };

  return (
    <button
      class="flex w-full cursor-pointer items-center gap-4 rounded-md px-3
        py-2.5 text-left transition-colors hover:bg-neutral-100
        active:bg-neutral-200"
      onClick={props.onClick}
    >
      <BlocksIcon class="shrink-0 text-neutral-400" />
      <span class="block min-w-0 flex-1">
        <Typography as="span" class="block truncate">
          {props.metadata.name ?? props.preferences.filePath}
        </Typography>
        <Show when={subtitle()}>
          <Typography variant="caption" class="block truncate">
            {subtitle()}
          </Typography>
        </Show>
      </span>
    </button>
  );
}
