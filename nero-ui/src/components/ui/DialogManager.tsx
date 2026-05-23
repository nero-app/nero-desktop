import { createSignal, type JSX } from "solid-js";

export interface DialogOverlayProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

type DialogEntry = {
  renderFn: (props: DialogOverlayProps) => JSX.Element;
};

const [current, setCurrent] = createSignal<DialogEntry | null>(null);
const [isOpen, setIsOpen] = createSignal(false);

const dismiss = () => {
  setIsOpen(false);
  setCurrent(null);
};

export const dialogManager = {
  show: (renderFn: (props: DialogOverlayProps) => JSX.Element) => {
    setCurrent({ renderFn });
    setIsOpen(true);
  },
  close: dismiss,
};

export function DialogOverlay() {
  const props = {
    get open() {
      return isOpen();
    },
    onOpenChange(open: boolean) {
      if (!open) dismiss();
    },
  };

  return <>{current()?.renderFn(props)}</>;
}
