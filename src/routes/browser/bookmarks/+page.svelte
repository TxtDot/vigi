<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { Bookmark } from "$lib/types";
  import { renderLink } from "$lib/utils";
  import { goto } from "$app/navigation";
  import { showToast } from "$lib/state.svelte";
  import X from "$lib/icons/X.svelte";

  let bookmarks: Bookmark[] = $state([]);
  let allTags: string[] = $state([]);
  let searchQuery = $state("");
  let selectedTag: string | null = $state(null);
  let editingBookmark: Bookmark | null = $state(null);
  let editTags = $state("");

  onMount(async () => {
    await loadBookmarks();
    await loadTags();
  });

  async function loadBookmarks() {
    try {
      bookmarks = await invoke("get_bookmarks");
    } catch {
      bookmarks = [];
    }
  }

  async function loadTags() {
    try {
      allTags = await invoke("get_all_tags");
    } catch {
      allTags = [];
    }
  }

  async function removeBookmark(uri: string) {
    try {
      await invoke("remove_bookmark", { uri });
      await loadBookmarks();
      await loadTags();
      showToast("Bookmark removed", "info");
    } catch {
      showToast("Error removing bookmark", "error");
    }
  }

  async function updateBookmarkTags() {
    if (!editingBookmark) return;

    const tags = editTags
      .split(",")
      .map((t) => t.trim())
      .filter((t) => t);

    try {
      await invoke("add_bookmark", {
        uri: editingBookmark.uri,
        title: editingBookmark.title,
        tags,
      });
      await loadBookmarks();
      await loadTags();
      editingBookmark = null;
      showToast("Bookmark updated", "success");
    } catch {
      showToast("Error updating bookmark", "error");
    }
  }

  function startEdit(bookmark: Bookmark) {
    editingBookmark = bookmark;
    editTags = bookmark.tags.join(", ");
  }

  function cancelEdit() {
    editingBookmark = null;
    editTags = "";
  }

  function navigateTo(uri: string) {
    goto(renderLink(uri));
  }

  let filteredBookmarks = $derived(() => {
    let result = bookmarks;

    if (selectedTag) {
      result = result.filter((b) => b.tags.includes(selectedTag!));
    }

    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      result = result.filter(
        (b) =>
          b.title.toLowerCase().includes(query) ||
          b.uri.toLowerCase().includes(query) ||
          b.tags.some((t) => t.toLowerCase().includes(query)),
      );
    }

    return result;
  });
</script>

<div class="page-container">
  <div class="page-header">
    <h1 class="page-title">Bookmarks</h1>
    <input
      type="text"
      class="page-search"
      placeholder="Search..."
      bind:value={searchQuery}
    />
  </div>

  {#if allTags.length > 0}
    <div class="tags-filter">
      <button
        class="tag-filter-btn"
        class:selected={selectedTag === null}
        onclick={() => (selectedTag = null)}
      >
        all
      </button>
      {#each allTags as tag}
        <button
          class="tag-filter-btn"
          class:selected={selectedTag === tag}
          onclick={() => (selectedTag = selectedTag === tag ? null : tag)}
        >
          #{tag}
        </button>
      {/each}
    </div>
  {/if}

  <div class="bookmarks-list">
    {#each filteredBookmarks() as bookmark}
      <div class="bookmark-card">
        {#if editingBookmark?.uri === bookmark.uri}
          <div class="bookmark-edit">
            <div class="bookmark-info">
              <span class="bookmark-title">{bookmark.title}</span>
              <span class="bookmark-uri">{bookmark.uri}</span>
            </div>
            <div class="edit-tags-row">
              <input
                type="text"
                class="edit-tags-input"
                placeholder="tags separated by commas"
                bind:value={editTags}
              />
              <button class="edit-btn save" onclick={updateBookmarkTags}>
                ✓
              </button>
              <button class="edit-btn cancel" onclick={cancelEdit}>✕</button>
            </div>
          </div>
        {:else}
          <button
            class="bookmark-content"
            onclick={() => navigateTo(bookmark.uri)}
          >
            <div class="bookmark-info">
              <span class="bookmark-title">{bookmark.title}</span>
              <span class="bookmark-uri">{bookmark.uri}</span>
              {#if bookmark.tags.length > 0}
                <div class="bookmark-tags">
                  {#each bookmark.tags as tag}
                    <span class="bookmark-tag">#{tag}</span>
                  {/each}
                </div>
              {/if}
            </div>
          </button>
          <div class="bookmark-actions">
            <button class="action-btn" onclick={() => startEdit(bookmark)}>
              ✎
            </button>
            <button
              class="action-btn danger"
              onclick={() => removeBookmark(bookmark.uri)}
            >
              <X width="14" height="14" />
            </button>
          </div>
        {/if}
      </div>
    {/each}

    {#if filteredBookmarks().length === 0}
      <div class="empty-state">
        {#if searchQuery || selectedTag}
          Nothing found
        {:else}
          No bookmarks yet
        {/if}
      </div>
    {/if}
  </div>
</div>
