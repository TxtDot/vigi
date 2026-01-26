<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { Settings } from "$lib/types";
  import { showToast } from "$lib/state.svelte";

  let settings: Settings = $state({
    theme: "green",
    custom_css: undefined,
    search_engine: "https://s.dc09.xyz/search?q=%s",
    home_page: "browser://main",
  });

  let showCustomCss = $state(false);

  const themes = [
    { id: "green", name: "Green" },
    { id: "blue", name: "Blue" },
    { id: "dark", name: "Dark" },
    { id: "light", name: "Light" },
  ];

  const searchEngines = [
    { id: "https://s.dc09.xyz/search?q=%s", name: "SearXNG" },
    { id: "https://duckduckgo.com/?q=%s", name: "DuckDuckGo" },
    { id: "https://www.google.com/search?q=%s", name: "Google" },
  ];

  onMount(async () => {
    try {
      settings = await invoke("get_settings");
    } catch {
      // Use defaults
    }
  });

  async function saveSettings() {
    try {
      await invoke("save_settings", { settings });
      applyTheme(settings.theme);
      showToast("Settings saved", "success");
    } catch {
      showToast("Error saving settings", "error");
    }
  }

  function applyTheme(theme: string) {
    const html = document.documentElement;
    html.className = `${theme}-theme`;
  }

  async function clearHistory() {
    try {
      await invoke("clear_history");
      showToast("History cleared", "success");
    } catch {
      showToast("Error clearing history", "error");
    }
  }
</script>

<div class="settings-page">
  <h1 class="settings-title">Settings</h1>

  <section class="settings-section">
    <h2 class="settings-section-title">Appearance</h2>

    <div class="settings-row">
      <label class="settings-label">Theme</label>
      <div class="theme-options">
        {#each themes as theme}
          <button
            class="theme-option"
            class:selected={settings.theme === theme.id}
            onclick={() => {
              settings.theme = theme.id;
              saveSettings();
            }}
          >
            <span class="theme-preview {theme.id}-theme-preview"></span>
            <span>{theme.name}</span>
          </button>
        {/each}
      </div>
    </div>

    <div class="settings-row">
      <button
        class="settings-link"
        onclick={() => (showCustomCss = !showCustomCss)}
      >
        {showCustomCss ? "Hide" : "Advanced CSS settings..."}
      </button>
    </div>

    {#if showCustomCss}
      <div class="settings-row">
        <textarea
          class="custom-css-input"
          placeholder="/* Custom CSS */"
          bind:value={settings.custom_css}
          onblur={saveSettings}
        ></textarea>
      </div>
    {/if}
  </section>

  <section class="settings-section">
    <h2 class="settings-section-title">Search</h2>

    <div class="settings-row">
      <label class="settings-label">Search Engine</label>
      <select
        class="settings-select"
        bind:value={settings.search_engine}
        onchange={saveSettings}
      >
        {#each searchEngines as engine}
          <option value={engine.id}>{engine.name}</option>
        {/each}
      </select>
    </div>

    <div class="settings-row">
      <label class="settings-label">Search Engine URL</label>
      <input
        type="text"
        class="settings-input"
        bind:value={settings.search_engine}
        onblur={saveSettings}
        placeholder="https://s.dc09.xyz/search?q=%s"
      />
    </div>
  </section>

  <section class="settings-section">
    <h2 class="settings-section-title">Data</h2>

    <div class="settings-row">
      <label class="settings-label">Home Page</label>
      <input
        type="text"
        class="settings-input"
        bind:value={settings.home_page}
        onblur={saveSettings}
        placeholder="browser://main"
      />
    </div>

    <div class="settings-row">
      <button class="settings-button danger" onclick={clearHistory}>
        Clear History
      </button>
    </div>
  </section>
</div>
