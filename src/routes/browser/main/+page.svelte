<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { HistoryEntry, Bookmark, Settings } from "$lib/types";
  import { renderLink, gotoTBI } from "$lib/utils";
  import { goto } from "$app/navigation";

  let searchQuery = $state("");
  let frequentSites: HistoryEntry[] = $state([]);
  let recentHistory: HistoryEntry[] = $state([]);
  let bookmarks: Bookmark[] = $state([]);
  let settings: Settings = $state({
    theme: "green",
    search_engine: "https://s.dc09.xyz/search?q=%s",
    home_page: "browser://main",
  });

  onMount(async () => {
    await Promise.all([
      loadFrequentSites(),
      loadRecentHistory(),
      loadBookmarks(),
      loadSettings(),
    ]);
  });

  async function loadFrequentSites() {
    try {
      frequentSites = await invoke("get_frequent_sites", { limit: 8 });
    } catch {
      frequentSites = [];
    }
  }

  async function loadRecentHistory() {
    try {
      recentHistory = await invoke("get_history", { limit: 5 });
    } catch {
      recentHistory = [];
    }
  }

  async function loadBookmarks() {
    try {
      bookmarks = await invoke("get_bookmarks");
      bookmarks = bookmarks.slice(0, 5);
    } catch {
      bookmarks = [];
    }
  }

  async function loadSettings() {
    try {
      settings = await invoke("get_settings");
    } catch {
      // Use defaults
    }
  }

  function handleSearch(e: KeyboardEvent) {
    if (e.key === "Enter" && searchQuery.trim()) {
      const searchUrl = settings.search_engine.replace(
        "%s",
        encodeURIComponent(searchQuery),
      );
      gotoTBI(searchUrl);
    }
  }

  function navigateTo(uri: string) {
    goto(renderLink(uri));
  }

  function formatTime(timestamp: number): string {
    return new Date(timestamp * 1000).toLocaleTimeString("ru-RU", {
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  function getHostname(uri: string): string {
    try {
      return new URL(uri).hostname;
    } catch {
      return uri;
    }
  }
</script>

<div class="main-page">
  <div class="search-section">
    <input
      type="text"
      class="main-search"
      placeholder="Search..."
      bind:value={searchQuery}
      onkeydown={handleSearch}
    />
  </div>

  {#if frequentSites.length > 0}
    <section class="main-section">
      <h2 class="section-title">Frequent Sites</h2>
      <div class="frequent-sites">
        {#each frequentSites as site}
          <button class="frequent-site" onclick={() => navigateTo(site.uri)}>
            <span class="site-icon">
              {(site.title || getHostname(site.uri)).charAt(0).toUpperCase()}
            </span>
            <span class="site-name">
              {site.title || getHostname(site.uri)}
            </span>
          </button>
        {/each}
      </div>
    </section>
  {/if}

  {#if recentHistory.length > 0}
    <section class="main-section">
      <div class="section-header">
        <h2 class="section-title">Recent</h2>
        <button
          class="section-link"
          onclick={() => gotoTBI("browser://history")}>History →</button
        >
      </div>
      <div class="recent-list">
        {#each recentHistory as entry}
          <button class="recent-item" onclick={() => navigateTo(entry.uri)}>
            <span class="recent-title">{entry.title || entry.uri}</span>
            <span class="recent-uri">{getHostname(entry.uri)}</span>
            <span class="recent-time">{formatTime(entry.timestamp)}</span>
          </button>
        {/each}
      </div>
    </section>
  {/if}

  {#if bookmarks.length > 0}
    <section class="main-section">
      <div class="section-header">
        <h2 class="section-title">Bookmarks</h2>
        <button
          class="section-link"
          onclick={() => gotoTBI("browser://bookmarks")}>All bookmarks →</button
        >
      </div>
      <div class="bookmarks-list">
        {#each bookmarks as bookmark}
          <button
            class="bookmark-item"
            onclick={() => navigateTo(bookmark.uri)}
          >
            <span class="bookmark-title">{bookmark.title}</span>
            {#if bookmark.tags.length > 0}
              <div class="bookmark-tags">
                {#each bookmark.tags.slice(0, 3) as tag}
                  <span class="bookmark-tag">#{tag}</span>
                {/each}
              </div>
            {/if}
          </button>
        {/each}
      </div>
    </section>
  {/if}

  {#if frequentSites.length === 0 && recentHistory.length === 0 && bookmarks.length === 0}
    <div class="empty-state">
      <p>Welcome to Vigi!</p>
      <p class="empty-hint">
        Start browsing by entering an address in the search bar
      </p>
    </div>
  {/if}
</div>
