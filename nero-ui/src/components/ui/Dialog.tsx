import { Button } from "./Button";
import {
  Dialog as KobalteDialog,
  type DialogRootProps,
} from "@kobalte/core/dialog";
import { XIcon } from "lucide-solid";
import { splitProps, type ParentProps } from "solid-js";
import { twMerge } from "tailwind-merge";

type RootProps = ParentProps & DialogRootProps;

function Root(props: RootProps) {
  const [local, others] = splitProps(props, ["children"]);

  return (
    <KobalteDialog {...others}>
      <KobalteDialog.Portal>
        <KobalteDialog.Overlay
          class="fixed inset-0 bg-black/50 backdrop-blur-xs"
        />
        <div class="fixed inset-0 flex items-center justify-center p-4">
          <KobalteDialog.Content
            class="flex aspect-video w-full max-w-[85vw] flex-col
              overflow-hidden rounded bg-white shadow-xl"
          >
            {local.children}
          </KobalteDialog.Content>
        </div>
      </KobalteDialog.Portal>
    </KobalteDialog>
  );
}

type SectionProps = ParentProps & {
  class?: string;
};

function Header(props: SectionProps & { title: string }) {
  const [local, others] = splitProps(props, ["children", "class", "title"]);

  return (
    <header
      class={twMerge(
        `flex items-center justify-between gap-4 border-b border-neutral-200
        px-4 py-2`,
        local.class,
      )}
      {...others}
    >
      <KobalteDialog.Title class="text-lg font-semibold text-neutral-900">
        {local.title}
      </KobalteDialog.Title>

      <KobalteDialog.CloseButton as={Button} variant="outline" size="icon">
        <XIcon />
      </KobalteDialog.CloseButton>
    </header>
  );
}

function Content(props: SectionProps) {
  const [local, others] = splitProps(props, ["children", "class"]);

  return (
    <div class={twMerge("flex-1 overflow-y-auto", local.class)} {...others}>
      {local.children}
    </div>
  );
}

function Footer(props: SectionProps) {
  const [local, others] = splitProps(props, ["children", "class"]);

  return (
    <footer
      class={twMerge(
        "flex gap-4 border-t border-neutral-200 px-6 py-4",
        local.class,
      )}
      {...others}
    >
      {local.children}
    </footer>
  );
}

export const Dialog = Object.assign(Root, {
  Header,
  Content,
  Footer,
});
