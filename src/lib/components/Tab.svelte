<script lang="ts">
  import type { StateTab } from "$lib/types";
  import { removeTab, selectTab } from "$lib/utils";
  import { fade, slide } from "svelte/transition";
  import Close from "$lib/icons/Close.svelte";
  import { isLoading } from "$lib/stores";
  import GooLoad from "$lib/icons/GooLoad.svelte";

  export let active = false;
  export let tab: StateTab;

  export let id: number;

  let tabElement: HTMLButtonElement;

  let hovered = false;
  let loading = false;

  isLoading.subscribe((val) => {
    loading = val;
  });
</script>

<div
  class="flex gap-1 items-center shrink"
  on:mouseenter={() => (hovered = true)}
  on:mouseleave={() => (hovered = false)}
  role="tab"
  tabindex={id}
>
  {#if hovered}
    <button
      class="close-button"
      transition:slide={{ duration: 100, axis: "x" }}
      on:click={() => removeTab(id)}
    >
      <Close />
    </button>
  {/if}

  <button
    class="tab"
    class:active
    transition:slide={{ duration: 100 }}
    bind:this={tabElement}
    on:click={() => {
      if (!active) {
        selectTab(id);
      }
    }}
  >
    <div>
      {#if loading && active}
        <GooLoad />
      {/if}
    </div>

    <div>
      {tab.title}
    </div>
  </button>
</div>
