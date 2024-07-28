<script>
  import Block from "$lib/components/Block.svelte";
  import WindowControls from "$lib/components/WindowControls.svelte";
  import "../app.css";
</script>

<div class="common-window" data-tauri-drag-region>
  <Block className="sidebar">
    <WindowControls />
  </Block>
  <div class="main-window">
    <Block className="top-bar">Top bar</Block>

    <Block className="browser-window">Browser window</Block>
  </div>
</div>
