<script lang="ts">
  import { link } from "./Router.svelte";
  import { SearchIcon } from "@lucide/svelte";
</script>

<nav class="grid w-full grid-cols-[1fr_1fr] text-neutral-900">
  <ul class="flex w-full justify-between gap-4">
    <li>
      <a href="/" use:link>Home</a>
    </li>
    <li>
      <a href="/settings/extensions" use:link>Extensions</a>
    </li>
    <li>
      <a href="/settings" use:link>Settings</a>
    </li>
  </ul>

  <ul class="flex items-center justify-end gap-4">
    <li>
      <a href="/search" use:link>
        <SearchIcon size={22} />
      </a>
    </li>
  </ul>
</nav>
