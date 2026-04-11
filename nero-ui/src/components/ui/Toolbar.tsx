import { t } from "../../lib/i18n";
import { Typography } from "./Typography";
import { A } from "@solidjs/router";
import { SearchIcon } from "lucide-solid";
import { For } from "solid-js";

const links = [
  { href: "/", label: () => t("nav.home") },
  { href: "/settings/extensions", label: () => t("nav.extensions") },
  { href: "/settings/app", label: () => t("nav.settings") },
];

export default function Toolbar() {
  return (
    <nav class="grid w-full grid-cols-[1fr_1fr]">
      <ul class="flex w-full justify-between gap-4">
        <For each={links}>
          {(link) => (
            <li>
              <A href={link.href}>
                <Typography as="span">{link.label()}</Typography>
              </A>
            </li>
          )}
        </For>
      </ul>

      <ul class="flex items-center justify-end">
        <li>
          <A href="/search">
            <SearchIcon size={22} />
          </A>
        </li>
      </ul>
    </nav>
  );
}
