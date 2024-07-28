<script>
  import Close from "$lib/icons/Close.svelte";
  import Maximize from "$lib/icons/Maximize.svelte";
  import Minimize from "$lib/icons/Minimize.svelte";
  import Button from "./Button.svelte";

  import { appWindow } from "@tauri-apps/api/window";
</script>

<div class="flex justify-start" data-tauri-drag-region>
  <Button onClick={() => appWindow.close()}>
    <Close />
  </Button>
  <Button onClick={() => appWindow.toggleMaximize()}>
    <Maximize />
  </Button>
  <Button onClick={() => appWindow.minimize()}>
    <Minimize />
  </Button>
</div>
