<script lang="ts">
  export let className = "";
  export let draggable = false;
</script>

{#if draggable}
  <div class="block{className ? ` ${className}` : ''}" data-tauri-drag-region>
    <slot />
  </div>
{:else}
  <div class="block{className ? ` ${className}` : ''}">
    <slot />
  </div>
{/if}
