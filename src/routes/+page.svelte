<script lang="ts">
  import "../app.css";

  import TopBar from "$lib/components/TopBar.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import BrowserWindow from "$lib/components/BrowserWindow.svelte";
  import type { Root } from "@txtdot/dalet";

  import { invoke } from "@tauri-apps/api/tauri";

  let sidebarOpen = true;
  let inputValue = "";

  let data: Root = [];

  document.addEventListener("keypress", (e: KeyboardEvent) => {
    const formElements = ["INPUT", "TEXTAREA", "SELECT", "OPTION"];
    if (formElements.includes((e.target as Element).tagName)) {
      return;
    }
    if (e.key === "q") sidebarOpen = !sidebarOpen;
  });

  function processInput() {
    invoke("process_input", { input: inputValue })
      .then((res) => {
        data = res as Root;
      })
      .catch((err) => {
        data = [{ id: 0, body: "Error: " + err, argument: null }];
      });
  }
</script>

<div class="common-window" data-tauri-drag-region>
  <Sidebar bind:sidebarOpen />

  <div class="main-window">
    <TopBar bind:sidebarOpen bind:inputValue onInput={processInput} />
    <BrowserWindow {data} />
  </div>
</div>
