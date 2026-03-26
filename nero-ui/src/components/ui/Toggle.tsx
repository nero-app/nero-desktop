import { Switch } from "@kobalte/core";
import type { SwitchRootProps } from "@kobalte/core/switch";

export function Toggle(props: SwitchRootProps) {
  return (
    <Switch.Root {...props}>
      <Switch.Input />
      <Switch.Control
        class="inline-flex h-6 w-11 rounded-full border-2 border-transparent
          bg-neutral-300 transition-colors data-checked:bg-orange-200"
      >
        <Switch.Thumb
          class="size-5 rounded-full bg-white shadow transition-transform
            data-checked:translate-x-5"
        />
      </Switch.Control>
    </Switch.Root>
  );
}
