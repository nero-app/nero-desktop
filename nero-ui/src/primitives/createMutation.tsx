import { createSignal } from "solid-js";

export type MutationResource<TData> = {
  (): TData | undefined;
  loading: boolean;
  error: any;
  state: "unresolved" | "pending" | "errored" | "ready";
};

export function createMutation<TData, TVariables>(
  mutationFn: (variables: TVariables) => Promise<TData>,
): [
  MutationResource<TData>,
  (variables: TVariables) => Promise<TData | undefined>,
] {
  const [data, setData] = createSignal<TData | undefined>(undefined);
  const [error, setError] = createSignal<any>(null);
  const [state, setState] =
    createSignal<MutationResource<TData>["state"]>("unresolved");

  async function mutate(variables: TVariables): Promise<TData | undefined> {
    setState("pending");
    setError(null);

    try {
      const result = await mutationFn(variables);
      setData(() => result);
      setState("ready");
      return result;
    } catch (err) {
      setError(err);
      setState("errored");
      return undefined;
    }
  }

  const resource = () => data();

  Object.defineProperties(resource, {
    loading: { get: () => state() === "pending" },
    error: { get: () => error() },
    state: { get: () => state() },
  });

  return [resource as MutationResource<TData>, mutate];
}
