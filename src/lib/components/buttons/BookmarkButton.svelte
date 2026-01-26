<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Bookmarks from "$lib/icons/Bookmarks.svelte";
  import BookmarkFilled from "$lib/icons/BookmarkFilled.svelte";
  import Button from "../Button.svelte";
  import { currentLink } from "$lib/utils";
  import { showToast } from "$lib/state.svelte";
  import { page } from "$app/state";

  let isBookmarked = $state(false);
  let showPopup = $state(false);
  let tagsInput = $state("");

  async function checkBookmark() {
    const link = currentLink();
    if (link.ty !== "RENDER") {
      isBookmarked = false;
      return;
    }
    try {
      isBookmarked = await invoke("is_bookmarked", { uri: link.uri });
    } catch {
      isBookmarked = false;
    }
  }

  async function toggleBookmark() {
    const link = currentLink();
    if (link.ty !== "RENDER") return;

    if (isBookmarked) {
      showPopup = true;
    } else {
      try {
        await invoke("add_bookmark", {
          uri: link.uri,
          title: link.title || link.uri,
          tags: [],
        });
        isBookmarked = true;
        showToast("Bookmark added", "success");
      } catch {
        showToast("Error adding bookmark", "error");
      }
    }
  }

  async function saveBookmark() {
    const link = currentLink();
    const tags = tagsInput
      .split(",")
      .map((t) => t.trim())
      .filter((t) => t);

    try {
      await invoke("add_bookmark", {
        uri: link.uri,
        title: link.title || link.uri,
        tags,
      });
      showPopup = false;
      showToast("Bookmark updated", "success");
    } catch {
      showToast("Error updating bookmark", "error");
    }
  }

  async function removeBookmark() {
    const link = currentLink();
    try {
      await invoke("remove_bookmark", { uri: link.uri });
      isBookmarked = false;
      showPopup = false;
      tagsInput = "";
      showToast("Bookmark removed", "info");
    } catch {
      showToast("Error removing bookmark", "error");
    }
  }

  // Check bookmark status on page change
  $effect(() => {
    page.url;
    checkBookmark();
  });

  onMount(() => {
    checkBookmark();
  });
</script>

<div class="bookmark-wrapper">
  <Button className="top-bar-input-button" onclick={toggleBookmark}>
    {#if isBookmarked}
      <BookmarkFilled />
    {:else}
      <Bookmarks />
    {/if}
  </Button>

  {#if showPopup}
    <div class="bookmark-popup">
      <div class="popup-header">Edit bookmark</div>
      <input
        type="text"
        class="tags-input"
        placeholder="tags separated by commas"
        bind:value={tagsInput}
      />
      <div class="popup-actions">
        <button class="popup-btn save" onclick={saveBookmark}>Save</button>
        <button class="popup-btn delete" onclick={removeBookmark}>
          Remove
        </button>
        <button class="popup-btn cancel" onclick={() => (showPopup = false)}>
          Cancel
        </button>
      </div>
    </div>
  {/if}
</div>
