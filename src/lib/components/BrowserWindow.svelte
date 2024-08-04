<script lang="ts">
    import type { Root } from "@txtdot/dalet";
    import Block from "./Block.svelte";
    import Renderer from "./DaletlRenderer/Renderer.svelte";
    import { isLoading, state } from "$lib/stores";
    import type { VigiState } from "$lib/types";
    import GooLoadSpin from "$lib/icons/GooLoadSpin.svelte";
    import { slide } from "svelte/transition";

    let loading = false;

    let data: Root;
    let tabId = 0;

    state.subscribe((st) => {
        data = (st as VigiState).current_data;
        console.log("ada");
        if (!loading) {
            tabId = (st as VigiState).current_tab_index;
        }
    });

    isLoading.subscribe((val) => {
        loading = val;

        if (loading) {
            tabId = -1;
        }
    });
</script>

<Block className="browser-window">
    {#if loading}
        <div transition:slide>
            <GooLoadSpin />
        </div>
    {/if}

    <Renderer {data} />
</Block>
