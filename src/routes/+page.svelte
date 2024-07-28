<script lang="ts">
  import "../app.css";

  import TopBar from "$lib/components/TopBar.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import BrowserWindow from "$lib/components/BrowserWindow.svelte";
  import type { Root } from "@txtdot/dalet";

  let sidebarOpen = true;
  let data: Root = [];

  document.addEventListener("keypress", (e: KeyboardEvent) => {
    const formElements = ["INPUT", "TEXTAREA", "SELECT", "OPTION"];
    if (formElements.includes((e.target as Element).tagName)) {
      return;
    }
    if (e.key === "q") sidebarOpen = !sidebarOpen;
  });
</script>

<div class="common-window" data-tauri-drag-region>
  <Sidebar bind:sidebarOpen />

  <div class="main-window">
    <TopBar bind:sidebarOpen />
    <BrowserWindow {data} />
  </div>
</div>
