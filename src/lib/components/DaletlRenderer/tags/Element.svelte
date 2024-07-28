<script lang="ts">
  import type { Body, Tag } from "@txtdot/dalet";
  import BodyRenderer from "../BodyRenderer.svelte";

  export let body: Body;
</script>

<section class="el"><BodyRenderer {body} /></section>
