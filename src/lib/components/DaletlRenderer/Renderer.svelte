<script lang="ts">
    import TagRenderer from "./TagRenderer.svelte";
    import { state } from "$lib/stores";
</script>

{#each $state.current_data as tag, i}
    <TagRenderer {tag} />
{/each}
