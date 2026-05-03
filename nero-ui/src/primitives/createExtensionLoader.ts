import { createMutation } from "./createMutation";
import {
  loadExtension,
  MAX_CACHE_SIZE_MB,
  type ExtensionInfo,
} from "@nero/plugin-extensions";
import { appCacheDir } from "@tauri-apps/api/path";
import { createResource, createSignal } from "solid-js";

export function createExtensionLoader(filePath: () => string) {
  const [cacheDir, { mutate: setCacheDir }] = createResource(appCacheDir);
  const [maxCacheSize, setMaxCacheSize] = createSignal<number>(0);

  const isMaxCacheExceeded = () => maxCacheSize() > MAX_CACHE_SIZE_MB;
  const maxCacheSizeBytes = () => {
    const mb = maxCacheSize();
    return mb > 0 ? mb * 1024 * 1024 : undefined;
  };

  const [loadMutation, mutate] = createMutation<ExtensionInfo, void>(
    async () => {
      return loadExtension(filePath(), {
        cacheDir: cacheDir()!,
        maxCacheSize: maxCacheSizeBytes(),
      });
    },
  );

  return {
    cache: {
      dir: cacheDir,
      setDir: setCacheDir,
      maxSize: maxCacheSize,
      setMaxSize: setMaxCacheSize,
      isExceeded: isMaxCacheExceeded,
    },
    load: {
      trigger: () => mutate() as Promise<ExtensionInfo>,
      mutation: loadMutation,
    },
  };
}
