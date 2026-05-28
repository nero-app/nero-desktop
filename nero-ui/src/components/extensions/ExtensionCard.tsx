import { t } from "../../lib/i18n";
import { Typography } from "../ui/Typography";
import type { LoadedExtension } from "@nero/plugin-extensions";
import { XIcon } from "lucide-solid";
import { Show } from "solid-js";

type ExtensionCardProps = {
  extension: LoadedExtension;
  onClick: () => void;
  onUnload: () => void;
};

export function ExtensionCard(props: ExtensionCardProps) {
  const subtitle = () => {
    const parts = [
      props.extension.metadata.version
        ? `v${props.extension.metadata.version}`
        : null,
      props.extension.metadata.authors ?? null,
      props.extension.metadata.licenses ?? null,
    ].filter(Boolean);
    return parts.length > 0 ? parts.join(` ${t("common.separator")} `) : null;
  };

  return (
    <div
      class="flex w-full items-center gap-4 rounded-md px-3 py-2.5
        transition-colors hover:bg-neutral-100"
    >
      <button
        class="flex min-w-0 flex-1 cursor-pointer items-center gap-4 text-left"
        onClick={props.onClick}
      >
        <span class="block min-w-0 flex-1">
          <Typography as="span" class="block truncate">
            {props.extension.metadata.name ?? props.extension.filePath}
          </Typography>
          <Show when={subtitle()}>
            <Typography variant="caption" class="block truncate">
              {subtitle()}
            </Typography>
          </Show>
        </span>
      </button>
      <button onClick={props.onUnload}>
        <XIcon size={16} />
      </button>
    </div>
  );
}
