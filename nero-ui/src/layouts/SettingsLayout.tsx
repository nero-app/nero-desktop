import { t } from "../lib/i18n";
import { SidebarLayout } from "./SidebarLayout";
import { A } from "@solidjs/router";
import { For, type ParentProps } from "solid-js";

const links = [
  { href: "/settings/app", label: () => t("nav.app") },
  { href: "/settings/extensions", label: () => t("nav.extensions") },
];

export default function SettingsLayout(props: ParentProps) {
  return (
    <SidebarLayout>
      <SidebarLayout.Main>{props.children}</SidebarLayout.Main>
      <SidebarLayout.Sidebar as="nav">
        <ul>
          <For each={links}>
            {(link) => (
              <li>
                <A href={link.href}>{link.label()}</A>
              </li>
            )}
          </For>
        </ul>
      </SidebarLayout.Sidebar>
    </SidebarLayout>
  );
}
