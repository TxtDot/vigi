<script lang="ts">
    import type { Argument, Body, Tag } from "@txtdot/dalet";
    import BodyRenderer from "../BodyRenderer.svelte";

    export let tag: Tag;

    let body = tag.body as string;
    let argument = tag.argument as number | null;
</script>

{#if argument === 2}
    <h2 class="h2">{body}</h2>
{:else if argument === 3}
    <h3 class="h3">{body}</h3>
{:else if argument === 4}
    <h4 class="h4">{body}</h4>
{:else if argument === 5}
    <h5 class="h5">{body}</h5>
{:else if argument === 6}
    <h6 class="h6">{body}</h6>
{:else}
    <h1 class="h1">{body}</h1>
{/if}
