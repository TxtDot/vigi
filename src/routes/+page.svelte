<script lang="ts">
  import "../app.css";

  import Block from "$lib/components/Block.svelte";
  import TopBar from "$lib/components/TopBar.svelte";
  import WindowControls from "$lib/components/WindowControls.svelte";
  import { slide } from "svelte/transition";

  let sidebarOpen = true;

  document.addEventListener("keypress", (e: KeyboardEvent) => {
    const formElements = ["INPUT", "TEXTAREA", "SELECT", "OPTION"];
    if (formElements.includes((e.target as Element).tagName)) {
      return;
    }
    if (e.key === "q") sidebarOpen = !sidebarOpen;
  });
</script>

<div class="common-window" data-tauri-drag-region>
  <Block className={`sidebar${sidebarOpen ? "" : " collapsed"}`}>
    {#if sidebarOpen}
      <div transition:slide={{ axis: "x", duration: 100 }}>
        <WindowControls />
      </div>
    {/if}
  </Block>

  <div class="main-window">
    <TopBar
      bind:sidebarOpen
      onSidebarToggle={() => (sidebarOpen = !sidebarOpen)}
    />
    <Block className="browser-window">Browser window</Block>
  </div>
</div>
