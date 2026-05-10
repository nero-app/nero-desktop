import { Typography } from "../components/ui/Typography";
import { t } from "../lib/i18n";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  createContext,
  createResource,
  Match,
  Switch,
  useContext,
  type ParentProps,
  type Resource,
} from "solid-js";

export interface AppPreferences {
  playerPath: string | null;
}

const AppContext = createContext<Resource<AppPreferences>>();

export function AppPreferencesProvider(props: ParentProps) {
  const [status, { mutate }] = createResource(() =>
    invoke<AppPreferences>("get_preferences"),
  );

  listen<AppPreferences>("app://preferences-changed", (event) => {
    mutate(event.payload);
  });

  return (
    <AppContext.Provider value={status}>
      <Switch>
        <Match when={status.loading}>
          <Typography>{t("common.loading")}</Typography>
        </Match>
        <Match when={status.error}>
          {(err) => <Typography>{err().message}</Typography>}
        </Match>
        <Match when={status()}>{props.children}</Match>
      </Switch>
    </AppContext.Provider>
  );
}

export function useAppPreferences() {
  const ctx = useContext(AppContext);
  if (!ctx)
    throw new Error(
      "useAppPreferences must be used within AppPreferencesProvider",
    );
  return () => ctx() as AppPreferences;
}
