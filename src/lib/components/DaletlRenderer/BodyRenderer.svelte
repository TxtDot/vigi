<script lang="ts">
    import type { Body } from "@txtdot/dalet";
    import TagRenderer from "./TagRenderer.svelte";

    export let body: Body;
    export let ifNull: any = undefined;
</script>

{#if typeof body === "string"}
    {#each body.split("\n") as line}
        {line} <br />
    {/each}
{:else if body !== null}
    {#each body as tag}
        <TagRenderer {tag} />
    {/each}
{/if}

{#if body === null && ifNull}
    {ifNull}
{/if}
