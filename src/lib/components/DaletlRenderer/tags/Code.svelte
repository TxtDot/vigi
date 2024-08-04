<script lang="ts">
    import type { Tag } from "@txtdot/dalet";

    export let tag: Tag;
</script>

<pre class={`pre code${tag.argument ? ` lang-${tag.argument}` : ""}`}><code
        >{tag.body}</code
    ></pre>
