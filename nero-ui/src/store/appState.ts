import type { Extension } from "@nero/plugin-extensions";
import { createStore } from "solid-js/store";

export interface AppConfig {
  playerPath: string | null;
  torrentEnabled: boolean;
  torrentOutputFolder: string | null;
}

interface AppState {
  config: AppConfig;
  extension: Extension | null;
}

const [appState, setAppState] = createStore<AppState>({
  config: {
    playerPath: null,
    torrentEnabled: false,
    torrentOutputFolder: null,
  },
  extension: null,
});

export { appState, setAppState };
