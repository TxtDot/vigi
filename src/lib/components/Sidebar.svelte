<script>
  import { slide } from "svelte/transition";

  import Block from "./Block.svelte";
  import WindowControls from "./WindowControls.svelte";

  export let sidebarOpen = true;
</script>

<Block className={`sidebar${sidebarOpen ? "" : " collapsed"}`} draggable>
  {#if sidebarOpen}
    <div transition:slide={{ axis: "x", duration: 100 }}>
      <WindowControls />
    </div>
  {/if}
</Block>
