<script lang="ts">
  import type { Root } from "@txtdot/dalet";
  import TagRenderer from "./TagRenderer.svelte";

  export let data: Root = [];
</script>

{#each data as tag}
  <TagRenderer {tag} />
{/each}
