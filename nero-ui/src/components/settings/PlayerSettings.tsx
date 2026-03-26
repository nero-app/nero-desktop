import { t } from "../../lib/i18n";
import { appState, setAppState } from "../../store/appState";
import { Button } from "../ui/Button";
import { SectionTable } from "../ui/SectionTable";
import { open } from "@tauri-apps/plugin-dialog";
import { Show } from "solid-js";

const players = [
  { label: "VLC", href: "https://www.videolan.org/vlc/" },
  { label: "MPV", href: "https://mpv.io/" },
  { label: "IINA", href: "https://iina.io/" },
];

function PlayerLink(props: { label: string; href: string }) {
  return (
    <a
      href={props.href}
      target="_blank"
      rel="noopener noreferrer"
      class="cursor-pointer text-blue-500 underline underline-offset-2"
    >
      {props.label}
    </a>
  );
}

export default function PlayerSettings() {
  const playerPath = () => appState.config.playerPath;

  async function selectPlayer() {
    const file = await open({
      title: t("settings.app.player.select_title"),
    });
    if (file) {
      setAppState("config", "playerPath", file);
    }
  }

  return (
    <SectionTable>
      <SectionTable.Header
        title={t("settings.app.player.title")}
        description={t("settings.app.player.description")}
      />
      <SectionTable.Content class="flex items-center justify-between gap-4">
        <div class="min-w-0">
          <p class="truncate font-medium text-neutral-900">
            {t("settings.app.player.placeholder")}
          </p>
          <Show
            when={playerPath()}
            fallback={
              <p class="truncate text-sm text-neutral-500">
                {t("settings.app.player.browse_hint")}
              </p>
            }
          >
            <p class="truncate text-sm text-neutral-500">{playerPath()}</p>
          </Show>
        </div>
        <Button variant="outline" size="sm" onClick={selectPlayer}>
          {t("common.change")}
        </Button>
      </SectionTable.Content>
      <SectionTable.Footer>
        <p class="text-xs text-neutral-600">
          <strong>{t("common.tip")}:</strong>{" "}
          {t("settings.app.player.popular_options")}{" "}
          {players.map((player, i) => (
            <>
              <PlayerLink label={player.label} href={player.href} />
              {i < players.length - 1 && ", "}
            </>
          ))}
        </p>
      </SectionTable.Footer>
    </SectionTable>
  );
}
