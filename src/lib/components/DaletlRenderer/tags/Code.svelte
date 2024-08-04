<script lang="ts">
    import type { Tag } from "@txtdot/dalet";
    import BodyRenderer from "../BodyRenderer.svelte";

    export let tag: Tag;
    export let body = tag.body as string;
    export let argument = tag.argument as string | undefined;
</script>

<pre class={`pre code${argument ? ` lang-${argument}` : ""}`}><code>{body}</code
    ></pre>
