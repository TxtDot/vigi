<script lang="ts">
    import type { Tag } from "@txtdot/dalet";
    import BodyRenderer from "../BodyRenderer.svelte";
    import { goToLink } from "$lib/utils";

    export let tag: Tag;

    function goTo() {
        goToLink(tag.argument as string);
    }
</script>

<button class="navbtn" on:click={goTo}>
    <BodyRenderer body={tag.body} ifNull={tag.argument} />
</button>
