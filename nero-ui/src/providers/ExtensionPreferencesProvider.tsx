import { Typography } from "../components/ui/Typography";
import { t } from "../lib/i18n";
import { getPreferences, type PreferencesData } from "@nero/plugin-extensions";
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

const ExtensionPreferencesContext = createContext<Resource<PreferencesData>>();

export function ExtensionPreferencesProvider(props: ParentProps) {
  const [preferences, { mutate }] = createResource(async () => {
    return await getPreferences();
  });

  listen<PreferencesData>("nero-extensions://preferences-changed", (event) => {
    mutate(event.payload);
  });

  return (
    <ExtensionPreferencesContext.Provider value={preferences}>
      <Switch>
        <Match when={preferences.loading}>
          <Typography>{t("common.loading")}</Typography>
        </Match>
        <Match when={preferences.error}>
          {(err) => <Typography>{err().message}</Typography>}
        </Match>
        <Match when={preferences()}>{props.children}</Match>
      </Switch>
    </ExtensionPreferencesContext.Provider>
  );
}

export function useExtensionPreferences() {
  const ctx = useContext(ExtensionPreferencesContext);
  if (!ctx)
    throw new Error(
      "useExtensionPreferences must be used within PreferencesProvider",
    );
  return () => ctx() as PreferencesData;
}
