<script lang="ts">
    import type { Tag } from "@txtdot/dalet";
    import BodyRenderer from "../BodyRenderer.svelte";

    export let tag: Tag;
</script>

<section class="el">
    <BodyRenderer body={tag.body} />
</section>
