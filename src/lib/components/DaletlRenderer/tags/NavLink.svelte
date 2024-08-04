<script lang="ts">
    import type { Tag } from "@txtdot/dalet";
    import BodyRenderer from "../BodyRenderer.svelte";

    export let tag: Tag;
</script>

<button class="navlink">
    <BodyRenderer body={tag.body} ifNull={tag.argument} />
</button>
