<script lang="ts">
    import type { Tag } from "@txtdot/dalet";
    import BodyRenderer from "../BodyRenderer.svelte";

    export let tag: Tag;
</script>

<div class="bq">
    <BodyRenderer body={tag.body} />
</div>
