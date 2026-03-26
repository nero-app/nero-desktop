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
      class={twMerge("grid size-full grid-cols-[2fr_3fr]", local.class)}
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

function Media<T extends ValidComponent = "div">(props: PolymorphicProps<T>) {
  const [local, others] = splitProps(props, ["as", "children", "class"]);

  return (
    <Dynamic
      component={local.as ?? "div"}
      class={twMerge("size-full overflow-hidden", local.class)}
      {...others}
    >
      {local.children}
    </Dynamic>
  );
}

function Content<T extends ValidComponent = "div">(props: PolymorphicProps<T>) {
  const [local, others] = splitProps(props, ["as", "children", "class"]);

  return (
    <div class="flex size-full flex-col overflow-y-auto px-8">
      <div class="sticky top-0 z-10 bg-white pt-4 pb-2">
        <Toolbar />
      </div>
      <Dynamic
        component={local.as ?? "div"}
        class={twMerge("flex-1", local.class)}
        {...others}
      >
        {local.children}
      </Dynamic>
    </div>
  );
}

export const MediaLayout = Object.assign(Root, {
  Media,
  Content,
});
