import { cva, type VariantProps } from "class-variance-authority";
import { splitProps, type JSX } from "solid-js";
import { twMerge } from "tailwind-merge";

const buttonVariants = cva(
  `inline-flex cursor-pointer items-center justify-center gap-2 text-center
  duration-300 not-disabled:active:scale-95 disabled:cursor-not-allowed
  disabled:opacity-50`,
  {
    variants: {
      variant: {
        primary:
          "rounded-md bg-orange-200 text-black not-disabled:hover:bg-orange-300",
        outline: `rounded-md border border-neutral-300 text-neutral-700
        not-disabled:hover:border-neutral-400 not-disabled:hover:bg-neutral-100`,
      },
      size: {
        sm: "px-3 py-1.5 text-sm",
        md: "px-4 py-2",
        icon: "size-10 shrink-0",
      },
    },
    compoundVariants: [
      {
        variant: "outline",
        size: "icon",
        class: "rounded-full",
      },
    ],
    defaultVariants: {
      variant: "primary",
      size: "md",
    },
  },
);

type ButtonProps = JSX.ButtonHTMLAttributes<HTMLButtonElement> &
  VariantProps<typeof buttonVariants>;

export function Button(props: ButtonProps) {
  const [local, rest] = splitProps(props, ["class", "variant", "size"]);

  return (
    <button
      class={twMerge(
        buttonVariants({ variant: local.variant, size: local.size }),
        local.class,
      )}
      {...rest}
    />
  );
}
