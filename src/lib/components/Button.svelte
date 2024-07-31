<script>
    export let onClick = () => {};
    export let className = "";
</script>

<button class={`button${className ? " " + className : ""}`} on:click={onClick}>
    <slot />
</button>
