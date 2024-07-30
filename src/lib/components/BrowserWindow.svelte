<script lang="ts">
  import type { Root } from "@txtdot/dalet";
  import Block from "./Block.svelte";
  import Renderer from "./DaletlRenderer/Renderer.svelte";
  import { isLoading, state } from "$lib/stores";
  import type { VigiState } from "$lib/types";

  let loading = false;

  let data: Root;

  state.subscribe((st) => {
    data = (st as VigiState).current_data;
  });

  isLoading.subscribe((val) => {
    loading = val;
  });
</script>

<Block className="browser-window">
  {#if loading}
    <div>Loading...</div>
  {:else}
    <Renderer {data} />
  {/if}
</Block>
