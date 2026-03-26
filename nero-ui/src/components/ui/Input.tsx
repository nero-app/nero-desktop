import { cva, type VariantProps } from "class-variance-authority";
import { splitProps, type JSX } from "solid-js";
import { twMerge } from "tailwind-merge";

const inputVariants = cva(
  `flex w-full rounded-md transition-colors placeholder:text-neutral-500
  disabled:cursor-not-allowed disabled:opacity-50`,
  {
    variants: {
      variant: {
        outline: "border border-neutral-300",
        filled: "bg-slate-100",
      },
    },
    defaultVariants: {
      variant: "filled",
    },
  },
);

type InputProps = JSX.InputHTMLAttributes<HTMLInputElement> &
  VariantProps<typeof inputVariants>;

export function Input(props: InputProps) {
  const [local, rest] = splitProps(props, ["class", "variant"]);

  return (
    <input
      class={twMerge(inputVariants({ variant: local.variant }), local.class)}
      {...rest}
    />
  );
}
