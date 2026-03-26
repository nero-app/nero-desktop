import {
  type ParentProps,
  type ComponentProps,
  type ValidComponent,
  Show,
  splitProps,
} from "solid-js";
import { Dynamic } from "solid-js/web";
import { twMerge } from "tailwind-merge";

function Root(props: ParentProps & ComponentProps<"section">) {
  const [local, others] = splitProps(props, ["children", "class"]);

  return (
    <section
      class={twMerge("rounded-lg border border-neutral-200", local.class)}
      {...others}
    >
      {local.children}
    </section>
  );
}

type HeaderProps = ComponentProps<"header"> & {
  title: string;
  description?: string;
};

function Header(props: HeaderProps) {
  const [local, others] = splitProps(props, [
    "title",
    "description",
    "class",
    "children",
  ]);

  return (
    <header
      class={twMerge("border-b border-neutral-200 px-4 py-3", local.class)}
      {...others}
    >
      <div class="flex items-center justify-between gap-2">
        <div>
          <h2 class="font-medium text-neutral-900">{local.title}</h2>
          <Show when={local.description}>
            <p class="text-sm text-neutral-500">{local.description}</p>
          </Show>
        </div>

        <Show when={local.children}>
          <div class="flex shrink-0 items-center gap-2">{local.children}</div>
        </Show>
      </div>
    </header>
  );
}
type PolymorphicProps<T extends ValidComponent> = ParentProps &
  ComponentProps<T> & {
    as?: T;
    class?: string;
  };

function Content<T extends ValidComponent = "div">(props: PolymorphicProps<T>) {
  const [local, others] = splitProps(props, ["as", "children", "class"]);

  return (
    <Dynamic
      component={local.as ?? "div"}
      class={twMerge("p-4", local.class)}
      {...others}
    >
      {local.children}
    </Dynamic>
  );
}

function Footer(props: ParentProps & ComponentProps<"footer">) {
  const [local, others] = splitProps(props, ["children", "class"]);

  return (
    <footer
      class={twMerge("border-t border-neutral-200 px-4 py-3", local.class)}
      {...others}
    >
      {local.children}
    </footer>
  );
}

export const SectionTable = Object.assign(Root, {
  Header,
  Content,
  Footer,
});
