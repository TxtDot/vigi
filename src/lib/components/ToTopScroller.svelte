<script lang="ts">
    import { onMount } from "svelte";

    onMount(() => {
        document.getElementsByClassName("browser-window")[0]?.scrollTo(0, 0);
    });
</script>
