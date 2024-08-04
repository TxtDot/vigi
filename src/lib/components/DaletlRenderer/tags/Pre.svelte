<script lang="ts">
    import type { Tag } from "@txtdot/dalet";
    import BodyRenderer from "../BodyRenderer.svelte";

    export let tag: Tag;
    export let body = tag.body as string;
</script>

<pre class="pre">{body}</pre>
