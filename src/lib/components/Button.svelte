<script>
  export let onClick = () => {};
</script>

<button
  class="hover:cursor-pointer hover:bg-gray-800 active:bg-gray-900 p-1 rounded-lg"
  on:click={onClick}
>
  <slot />
</button>
