import { onCleanup } from "solid-js";

export function createSentinel(
  onIntersect: () => void,
  options: IntersectionObserverInit = {},
) {
  return (el: HTMLElement) => {
    if (!el) return;

    const observer = new IntersectionObserver((entries) => {
      if (entries[0]?.isIntersecting) onIntersect();
    }, options);

    observer.observe(el);
    onCleanup(() => observer.disconnect());
  };
}
