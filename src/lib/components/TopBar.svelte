<script lang="ts">
  import ArrowLeft from "$lib/icons/ArrowLeft.svelte";
  import ArrowRight from "$lib/icons/ArrowRight.svelte";
  import Reload from "$lib/icons/Reload.svelte";
  import SidebarLeft from "$lib/icons/SidebarLeft.svelte";
  import SidebarRight from "$lib/icons/SidebarRight.svelte";
  import { state } from "$lib/stores";
  import { updateInput } from "$lib/utils";
  import Block from "./Block.svelte";
  import Button from "./Button.svelte";

  export let onBack = () => {};
  export let onForward = () => {};

  export let sidebarOpen = true;

  let currentInput = "";
  let input = "";

  state.subscribe((val) => {
    input = val.top_bar_input;
    currentInput = decodeURIComponent(input);
  });

  let iEl: HTMLInputElement;
</script>

<div class="top-bar">
  <Block className="flex">
    <Button onClick={() => (sidebarOpen = !sidebarOpen)}>
      {#if sidebarOpen}
        <SidebarLeft />
      {:else}
        <SidebarRight />
      {/if}
    </Button>
    <Button onClick={onBack}><ArrowLeft /></Button>
    <Button onClick={onForward}><ArrowRight /></Button>
    <Button onClick={() => updateInput(input)}><Reload /></Button>
  </Block>

  <input
    type="text"
    placeholder="Search or enter URL"
    class="search-input"
    bind:value={currentInput}
    bind:this={iEl}
    on:keypress={(e) => {
      if (e.key === "Enter") {
        updateInput(currentInput);
      }
    }}
    on:focus={() => {
      currentInput = input;
      setTimeout(() => {
        iEl.select();
        iEl.scrollLeft = iEl.scrollWidth;
      }, 1);
    }}
    on:focusout={() => {
      currentInput = decodeURIComponent(input);
    }}
  />
</div>
