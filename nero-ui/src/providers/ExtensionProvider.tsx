import { Typography } from "../components/ui/Typography";
import { t } from "../lib/i18n";
import { getStatus, type PluginStatus } from "@nero/plugin-extensions";
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

const ExtensionContext = createContext<Resource<PluginStatus>>();

export function ExtensionProvider(props: ParentProps) {
  const [status, { mutate }] = createResource(async () => {
    return await getStatus();
  });

  listen<PluginStatus>("nero-extensions://status-changed", (event) => {
    mutate(event.payload);
  });

  return (
    <ExtensionContext.Provider value={status}>
      <Switch>
        <Match when={status.loading}>
          <Typography>{t("common.loading")}</Typography>
        </Match>
        <Match when={status.error}>
          {(err) => <Typography>{err().message}</Typography>}
        </Match>
        <Match when={status()}>{props.children}</Match>
      </Switch>
    </ExtensionContext.Provider>
  );
}

export function useExtensionStatus() {
  const ctx = useContext(ExtensionContext);
  if (!ctx)
    throw new Error("useExtensionStatus must be used within ExtensionProvider");
  return () => ctx() as PluginStatus;
}
