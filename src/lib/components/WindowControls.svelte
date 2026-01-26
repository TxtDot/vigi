<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import Minimize from "$lib/icons/Minimize.svelte";
  import Maximize from "$lib/icons/Maximize.svelte";
  import Button from "./Button.svelte";
  import X from "$lib/icons/X.svelte";

  const appWindow = getCurrentWindow();

  async function minimize() {
    await appWindow.minimize();
  }

  async function toggleMaximize() {
    await appWindow.toggleMaximize();
  }

  async function close() {
    await appWindow.close();
  }
</script>

<Button className="" onclick={minimize}>
  <Minimize />
</Button>
<Button onclick={toggleMaximize}>
  <Maximize />
</Button>
<Button className="window-control-close" onclick={close}>
  <X />
</Button>
