import { createSignal } from "solid-js";

export type InfiniteResource<T> = {
  (): T[];
  loading: boolean;
  error: any;
  hasMore: boolean;
  state: "unresolved" | "pending" | "errored" | "ready";
};

export function createInfiniteResource<T>(
  fetcher: (page: number) => Promise<{ items: T[]; hasMore: boolean }>,
) {
  const [items, setItems] = createSignal<T[]>([]);
  const [page, setPage] = createSignal(1);
  const [hasMore, setHasMore] = createSignal(true);
  const [error, setError] = createSignal<any>(null);
  const [state, setState] =
    createSignal<InfiniteResource<T>["state"]>("unresolved");

  const loadPage = async (p: number) => {
    if (!hasMore() && p !== 1) return;
    setState("pending");
    setError(null);
    try {
      const response = await fetcher(p);
      setItems((prev) =>
        p === 1 ? response.items : [...prev, ...response.items],
      );
      setHasMore(response.hasMore);
      setState("ready");
    } catch (err) {
      setError(err);
      setState("errored");
    }
  };

  const resource = () => items();
  Object.defineProperties(resource, {
    loading: { get: () => state() === "pending" },
    error: { get: () => error() },
    state: { get: () => state() },
    hasMore: { get: () => hasMore() },
  });

  const loadNext = () => {
    if (state() !== "pending" && hasMore()) {
      const next = page() + 1;
      setPage(next);
      loadPage(next);
    }
  };

  const reset = () => {
    setPage(1);
    setHasMore(true);
    loadPage(1);
  };

  return [resource as InfiniteResource<T>, { loadNext, reset }] as const;
}
