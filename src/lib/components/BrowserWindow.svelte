<script lang="ts">
    import Block from "./Block.svelte";
    import Renderer from "./DaletlRenderer/Renderer.svelte";
    import { isLoading, state } from "$lib/stores";
    import GooLoadSpin from "$lib/icons/GooLoadSpin.svelte";
    import { slide } from "svelte/transition";
</script>

<Block className="browser-window">
    {#if $isLoading}
        <div transition:slide>
            <GooLoadSpin />
        </div>
    {/if}

    <Renderer />
</Block>
