import Toolbar from "../components/ui/Toolbar";
import type { ParentProps, ComponentProps, ValidComponent } from "solid-js";
import { splitProps } from "solid-js";
import { Dynamic } from "solid-js/web";
import { twMerge } from "tailwind-merge";

type MainProps = ParentProps & ComponentProps<"main">;

function Root(props: MainProps) {
  const [local, others] = splitProps(props, ["children", "class"]);

  return (
    <main
      class={twMerge("grid size-full grid-cols-[4fr_2fr]", local.class)}
      {...others}
    >
      {local.children}
    </main>
  );
}

type PolymorphicProps<T extends ValidComponent> = ParentProps &
  ComponentProps<T> & {
    as?: T;
    class?: string;
  };

function Main<T extends ValidComponent = "div">(props: PolymorphicProps<T>) {
  const [local, others] = splitProps(props, ["as", "children", "class"]);

  return (
    <Dynamic
      component={local.as ?? "div"}
      class={twMerge("overflow-y-auto", local.class)}
      {...others}
    >
      {local.children}
    </Dynamic>
  );
}

function Sidebar<T extends ValidComponent = "div">(props: PolymorphicProps<T>) {
  const [local, others] = splitProps(props, ["as", "children", "class"]);

  return (
    <Dynamic
      component={local.as ?? "div"}
      class={twMerge("flex flex-col overflow-y-auto px-4", local.class)}
      {...others}
    >
      <div class="sticky top-0 z-10 bg-white pt-4 pb-2">
        <Toolbar />
      </div>

      {local.children}
    </Dynamic>
  );
}

export const SidebarLayout = Object.assign(Root, {
  Main,
  Sidebar,
});
