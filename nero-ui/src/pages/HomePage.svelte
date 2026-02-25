<script lang="ts">
  import shockedCat from "../assets/images/shocked_cat.svg";
  import Toolbar from "../components/Toolbar.svelte";
  import { sampleSeries } from "../lib/dummy";
  import { SearchIcon } from "@lucide/svelte";

  function focusSearchInput() {
    document.getElementById("search-input")!.focus();
  }
</script>

{#snippet emptyFeedback()}
  <article
    class="flex size-full flex-col items-center justify-center overflow-auto pb-8"
  >
    <img class="w-64" src={shockedCat} alt="Shocked cat" />
    <p class="pb-2 text-center">
      Whoops...
      <br />
      Apparently there's nothing around here.
    </p>
    <button
      class="cursor-pointer rounded-lg bg-orange-200 px-3 py-1.5 duration-300 active:scale-95"
      onclick={focusSearchInput}
    >
      <div class="flex items-center gap-2">
        <SearchIcon />
        <span>Search for series</span>
      </div>
    </button>
  </article>
{/snippet}

<div class="grid h-full grid-cols-[2fr_3fr]">
  <figure class="overflow-hidden">
    <img
      class="size-full object-cover"
      src={sampleSeries.posterUrl}
      alt={sampleSeries.title}
    />
  </figure>
  <div class="flex size-full flex-col p-4">
    <Toolbar />
    {@render emptyFeedback()}
  </div>
</div>
