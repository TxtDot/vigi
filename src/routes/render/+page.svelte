<script lang="ts">
  import DaletRenderer from "$lib/components/DaletRenderer.svelte";
  import Alert from "$lib/icons/Alert.svelte";

  let { data } = $props();
</script>

{#if data.error}
  <div
    class="flex max-lg:flex-col items-center lg:justify-center h-full gap-5 mx-auto"
  >
    <Alert width="128" height="128" class="opacity-50" />

    <div class="flex flex-col">
      <h1 class="text-4xl font-black max-lg:text-center">
        {data.error.message}
      </h1>
      <div class="text-xl font-semibold opacity-50 max-w-80">
        {#if data.error.body}
          {data.error.body}
        {/if}
      </div>
    </div>
  </div>
{:else if data.body}
  <DaletRenderer body={data.body} />
{/if}
