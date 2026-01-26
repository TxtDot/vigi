<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { HistoryEntry } from "$lib/types";
  import { renderLink } from "$lib/utils";
  import { goto } from "$app/navigation";

  let history: HistoryEntry[] = $state([]);
  let frequentSites: HistoryEntry[] = $state([]);
  let searchQuery = $state("");
  let expandedDays: Set<string> = $state(new Set());

  onMount(async () => {
    await loadHistory();
    await loadFrequentSites();
  });

  async function loadHistory() {
    try {
      history = await invoke("get_history", { limit: 500 });
      // Auto-expand today and yesterday
      const grouped = groupByDay(history);
      if (grouped.has("Today")) expandedDays.add("Today");
      if (grouped.has("Yesterday")) expandedDays.add("Yesterday");
      expandedDays = new Set(expandedDays);
    } catch {
      history = [];
    }
  }

  async function loadFrequentSites() {
    try {
      frequentSites = await invoke("get_frequent_sites", { limit: 8 });
    } catch {
      frequentSites = [];
    }
  }

  async function search() {
    if (searchQuery.trim()) {
      try {
        history = await invoke("search_history", { query: searchQuery });
      } catch {
        history = [];
      }
    } else {
      await loadHistory();
    }
  }

  function groupByDay(entries: HistoryEntry[]): Map<string, HistoryEntry[]> {
    const groups = new Map<string, HistoryEntry[]>();
    const today = new Date();
    const yesterday = new Date(today);
    yesterday.setDate(yesterday.getDate() - 1);

    for (const entry of entries) {
      const date = new Date(entry.timestamp * 1000);
      let dayKey: string;

      if (date.toDateString() === today.toDateString()) {
        dayKey = "Today";
      } else if (date.toDateString() === yesterday.toDateString()) {
        dayKey = "Yesterday";
      } else {
        dayKey = date.toLocaleDateString("ru-RU", {
          day: "numeric",
          month: "long",
        });
      }

      if (!groups.has(dayKey)) {
        groups.set(dayKey, []);
      }
      groups.get(dayKey)!.push(entry);
    }

    return groups;
  }

  function formatTime(timestamp: number): string {
    return new Date(timestamp * 1000).toLocaleTimeString("ru-RU", {
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  function toggleDay(day: string) {
    if (expandedDays.has(day)) {
      expandedDays.delete(day);
    } else {
      expandedDays.add(day);
    }
    expandedDays = new Set(expandedDays);
  }

  function navigateTo(uri: string) {
    goto(renderLink(uri));
  }

  let groupedHistory = $derived(groupByDay(history));
</script>

<div class="page-container">
  <div class="page-header">
    <h1 class="page-title">History</h1>
    <input
      type="text"
      class="page-search"
      placeholder="Search..."
      bind:value={searchQuery}
      oninput={search}
    />
  </div>

  {#if frequentSites.length > 0 && !searchQuery}
    <section class="page-section">
      <h2 class="section-title">Frequently Visited</h2>
      <div class="frequent-sites">
        {#each frequentSites as site}
          <button class="frequent-site" onclick={() => navigateTo(site.uri)}>
            <span class="visit-count">{site.visit_count}</span>
            <span class="site-name"
              >{site.title || new URL(site.uri).hostname}</span
            >
          </button>
        {/each}
      </div>
    </section>
  {/if}

  <section class="page-section">
    {#each [...groupedHistory.entries()] as [day, entries]}
      <div class="day-group">
        <button class="day-header" onclick={() => toggleDay(day)}>
          <span class="day-arrow">{expandedDays.has(day) ? "▼" : "▶"}</span>
          <span class="day-name">{day}</span>
          <span class="day-count">({entries.length})</span>
        </button>

        {#if expandedDays.has(day)}
          <div class="day-entries">
            {#each entries as entry}
              <button
                class="history-entry"
                onclick={() => navigateTo(entry.uri)}
              >
                <div class="entry-info">
                  <span class="entry-title">{entry.title || entry.uri}</span>
                  <span class="entry-uri">{entry.uri}</span>
                </div>
                <span class="entry-time">{formatTime(entry.timestamp)}</span>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    {/each}

    {#if history.length === 0}
      <div class="empty-state">
        {#if searchQuery}
          Nothing found
        {:else}
          History is empty
        {/if}
      </div>
    {/if}
  </section>
</div>
