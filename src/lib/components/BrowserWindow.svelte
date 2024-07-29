<script lang="ts">
  import type { Root } from "@txtdot/dalet";
  import Block from "./Block.svelte";
  import Renderer from "./DaletlRenderer/Renderer.svelte";

  export let data: Root;
  export let isLoading = false;
</script>

<Block className="browser-window">
  {#if isLoading}
    <div>Loading...</div>
  {:else}
    <Renderer {data} />
  {/if}
</Block>
