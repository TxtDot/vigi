<script lang="ts">
    import type { Tag } from "@txtdot/dalet";
    import BodyRenderer from "../BodyRenderer.svelte";
    import TagRenderer from "../TagRenderer.svelte";

    export let tag: Tag;

    let body = tag.body as Tag[];
</script>

<ul class="ul">
    {#each body as tag}
        <li class="ul-li"><TagRenderer {tag} /></li>
    {/each}
</ul>
