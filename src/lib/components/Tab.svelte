<script lang="ts">
  import type { StateTab } from "$lib/types";
  import { removeTab, selectTab } from "$lib/utils";
  import { slide } from "svelte/transition";
  import Close from "$lib/icons/Close.svelte";

  export let active = false;
  export let tab: StateTab;

  export let id: number;

  let tabElement: HTMLButtonElement;

  let hovered = false;
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
      on:click={() => removeTab(id)}><Close /></button
    >
  {/if}

  <button
    class="tab"
    class:active
    transition:slide={{ duration: 100 }}
    bind:this={tabElement}
    on:click={() => {
      selectTab(id);
    }}
  >
    {tab.title}
  </button>
</div>
