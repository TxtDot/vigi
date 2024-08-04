<script lang="ts">
    import type { Tag } from "@txtdot/dalet";
    import BodyRenderer from "../BodyRenderer.svelte";

    export let tag: Tag;
</script>

<p class="p">
    <BodyRenderer body={tag.body} />
</p>
