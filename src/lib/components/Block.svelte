<script lang="ts">
  export let className = "";
</script>

<div class="p-3 rounded-xl bg-gray-700{className ? ` ${className}` : ''}">
  <slot />
</div>
