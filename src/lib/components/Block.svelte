<script lang="ts">
  export let className = "";
  export let draggable = false;
</script>

<div class="block{className ? ` ${className}` : ''}">
  <slot />
</div>
