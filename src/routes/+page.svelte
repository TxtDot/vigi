<script>
  import Block from "$lib/components/Block.svelte";
  import WindowControls from "$lib/components/WindowControls.svelte";
  import "../app.css";
</script>

<div class="window" data-tauri-drag-region>
  <Block className="col-span-2">
    <WindowControls />
  </Block>
  <Block className="col-span-6">Browser window</Block>
</div>
