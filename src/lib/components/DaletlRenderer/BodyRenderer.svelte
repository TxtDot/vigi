<script lang="ts">
  import type { Body } from "@txtdot/dalet";
  import TagRenderer from "./TagRenderer.svelte";
  import Element from "./tags/Element.svelte";

  export let body: Body;
</script>

{#if typeof body === "string"}
  <Element {body} />
{:else if body !== null}
  {#each body as tag}
    <TagRenderer {tag} />
  {/each}
{/if}
