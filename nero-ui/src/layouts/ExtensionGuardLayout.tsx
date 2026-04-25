import { appState } from "../store/appState";
import { useNavigate } from "@solidjs/router";
import { createEffect, Show, type ParentProps } from "solid-js";

export default function ExtensionGuardLayout(props: ParentProps) {
  const navigate = useNavigate();

  createEffect(() => {
    if (!appState.getters.extension()) {
      navigate("/settings/extensions", { replace: true });
    }
  });

  return <Show when={appState.getters.extension()}>{props.children}</Show>;
}
