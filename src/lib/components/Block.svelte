<script lang="ts">
  export let className = "";
</script>

<div class="block{className ? ` ${className}` : ''}">
  <slot />
</div>
