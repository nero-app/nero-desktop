import { t } from "../../lib/i18n";
import { appState } from "../../store/appState";
import { Button } from "../ui/Button";
import { SectionTable } from "../ui/SectionTable";
import { Typography } from "../ui/Typography";
import { open } from "@tauri-apps/plugin-dialog";
import { Show } from "solid-js";

const players = [
  { label: "VLC", href: "https://www.videolan.org/vlc/" },
  { label: "MPV", href: "https://mpv.io/" },
  { label: "IINA", href: "https://iina.io/" },
];

// TODO: https://v2.tauri.app/plugin/opener/
function PlayerLink(props: { label: string; href: string }) {
  return (
    <button class="cursor-pointer text-blue-500 underline underline-offset-2">
      <Typography variant="caption" as="span">
        {props.label}
      </Typography>
    </button>
  );
}

export default function PlayerSettings() {
  const playerPath = () => appState.getters.playerPath();

  async function selectPlayer() {
    const file = await open({
      title: t("settings.app.player.select_title"),
    });
    if (file) {
      appState.actions.setPlayerPath(file);
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
          <Typography variant="h4" class="truncate">
            {t("settings.app.player.placeholder")}
          </Typography>
          <Show
            when={playerPath()}
            fallback={
              <Typography variant="subtitle" class="truncate">
                {t("settings.app.player.browse_hint")}
              </Typography>
            }
          >
            <Typography variant="subtitle" class="truncate">
              {playerPath()}
            </Typography>
          </Show>
        </div>
        <Button variant="outline" size="sm" onClick={selectPlayer}>
          <Typography as="span">{t("common.change")}</Typography>
        </Button>
      </SectionTable.Content>
      <SectionTable.Footer>
        <Typography variant="caption">
          <strong>{t("common.tip")}:</strong>{" "}
          {t("settings.app.player.popular_options")}{" "}
          {players.map((player, i) => (
            <>
              <PlayerLink label={player.label} href={player.href} />
              {i < players.length - 1 && ", "}
            </>
          ))}
        </Typography>
      </SectionTable.Footer>
    </SectionTable>
  );
}
