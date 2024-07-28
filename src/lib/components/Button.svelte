<script>
  export let onClick = () => {};
</script>

<button class="button" on:click={onClick}>
  <slot />
</button>
