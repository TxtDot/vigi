<script lang="ts">
    import TagRenderer from "./TagRenderer.svelte";
    import { state } from "$lib/stores";
    import ToTopScroller from "../ToTopScroller.svelte";
</script>

{#each $state.current_data as tag, i}
    {#if i === 0}
        <ToTopScroller />
    {/if}
    <TagRenderer {tag} />
{/each}
