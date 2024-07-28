<script lang="ts">
  import type { Tag } from "@txtdot/dalet";
  import Element from "./tags/Element.svelte";

  export let tag: Tag;
</script>

{#if tag.id === 0}
  <Element body={tag.body} />
{/if}
