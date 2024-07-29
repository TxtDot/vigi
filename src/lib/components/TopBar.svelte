<script lang="ts">
  import ArrowLeft from "$lib/icons/ArrowLeft.svelte";
  import ArrowRight from "$lib/icons/ArrowRight.svelte";
  import Reload from "$lib/icons/Reload.svelte";
  import SidebarLeft from "$lib/icons/SidebarLeft.svelte";
  import SidebarRight from "$lib/icons/SidebarRight.svelte";
  import Block from "./Block.svelte";
  import Button from "./Button.svelte";

  export let onBack = () => {};
  export let onForward = () => {};
  export let onInput = () => {};

  export let sidebarOpen = true;
  export let inputValue = "";

  let currentInputValue = "";

  let input: HTMLInputElement;
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
    <Button onClick={onInput}><Reload /></Button>
  </Block>

  <input
    type="text"
    placeholder="Search or enter URL"
    class="search-input"
    bind:value={currentInputValue}
    bind:this={input}
    on:keypress={(e) => {
      if (e.key === "Enter") {
        inputValue = currentInputValue;
        onInput();
      }
    }}
    on:focus={() => {
      currentInputValue = inputValue;
      setTimeout(() => input.select(), 1);
    }}
    on:focusout={() => {
      currentInputValue = decodeURIComponent(inputValue);
    }}
  />
</div>
