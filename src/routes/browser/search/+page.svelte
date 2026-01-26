<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import { renderLink } from "$lib/utils";
  import type { Settings } from "$lib/types";

  onMount(async () => {
    const query = page.url.searchParams.get("q");
    if (!query) {
      goto("/browser/main");
      return;
    }

    let settings: Settings;
    try {
      settings = await invoke("get_settings");
    } catch {
      settings = {
        theme: "green",
        search_engine: "https://s.dc09.xyz/search?q=%s",
        home_page: "browser://main",
      };
    }

    const searchUrl = settings.search_engine.replace(
      "%s",
      encodeURIComponent(query),
    );

    goto(renderLink(searchUrl));
  });
</script>

<div class="search-redirect">
  <p>Searching...</p>
</div>
