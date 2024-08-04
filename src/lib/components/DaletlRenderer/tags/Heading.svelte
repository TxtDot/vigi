<script lang="ts">
    import type { Argument, Body, Tag } from "@txtdot/dalet";

    export let tag: Tag;
</script>

{#if tag.argument === 2}
    <h2 class="h2">{tag.body}</h2>
{:else if tag.argument === 3}
    <h3 class="h3">{tag.body}</h3>
{:else if tag.argument === 4}
    <h4 class="h4">{tag.body}</h4>
{:else if tag.argument === 5}
    <h5 class="h5">{tag.body}</h5>
{:else if tag.argument === 6}
    <h6 class="h6">{tag.body}</h6>
{:else}
    <h1 class="h1">{tag.body}</h1>
{/if}
