<script lang="ts">
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();
</script>

<div class="fixed flex h-screen w-full flex-col gap-4">
  <main class="h-full overflow-auto">
    {@render children?.()}
  </main>
</div>
