import { cva, type VariantProps } from "class-variance-authority";
import { splitProps, type JSX } from "solid-js";
import { Dynamic } from "solid-js/web";
import { twMerge } from "tailwind-merge";

const typographyVariants = cva("", {
  variants: {
    variant: {
      h1: "text-4xl font-bold",
      h2: "text-2xl font-semibold",
      h3: "text-xl font-semibold",
      h4: "text-lg font-medium",
      subtitle: "text-sm",
      body: "",
      caption: "text-xs",
    },
  },
  defaultVariants: {
    variant: "body",
  },
});

const variantTag: Record<
  NonNullable<VariantProps<typeof typographyVariants>["variant"]>,
  string
> = {
  h1: "h1",
  h2: "h2",
  h3: "h3",
  h4: "h4",
  subtitle: "p",
  body: "p",
  caption: "span",
};

type TypographyProps = JSX.HTMLAttributes<HTMLElement> &
  VariantProps<typeof typographyVariants> & {
    as?: string;
  };

export function Typography(props: TypographyProps) {
  const [local, rest] = splitProps(props, ["variant", "as", "class"]);

  const tag = local.as ?? variantTag[local.variant ?? "body"];

  return (
    <Dynamic
      component={tag}
      class={twMerge(
        typographyVariants({ variant: local.variant }),

        local.class,
      )}
      {...rest}
    />
  );
}
