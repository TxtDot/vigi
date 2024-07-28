<script lang="ts">
  import type { Body, Tag } from "@txtdot/dalet";
  import BodyRenderer from "../BodyRenderer.svelte";

  export let body: Body;
</script>

<section class="el">
  {#if typeof body === "string"}
    {#each body.split("\n") as line}
      {line} <br />
    {/each}
  {:else}
    <BodyRenderer {body} />
  {/if}
</section>
