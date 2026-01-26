<script lang="ts">
  import TopBarDesktop from "$lib/components/TopBarDesktop.svelte";
  import TopBarMobile from "$lib/components/TopBarMobile.svelte";
  import Toast from "$lib/components/Toast.svelte";
  import { temporal, vigi } from "$lib/state.svelte";
  import { onMount } from "svelte";
  import "../app.css";
  import { afterNavigate, beforeNavigate } from "$app/navigation";
  import { page } from "$app/state";
  import BotBar from "$lib/components/BotBar.svelte";
  import SideBar from "$lib/components/SideBar.svelte";

  const { children } = $props();

  let browserEl: HTMLDivElement;

  //todo: make this mess better
  let pathname = "";
  onMount(() => {
    pathname = page.url.pathname;
  });
  beforeNavigate(() => {
    pathname = page.url.pathname;
  });
  afterNavigate(() => {
    if (pathname !== page.url.pathname)
      browserEl.scrollTo({ top: 0, behavior: "auto" });
  });

  document.addEventListener("keypress", (e: KeyboardEvent) => {
    if (
      ["INPUT", "TEXTAREA", "SELECT", "OPTION"].includes(
        (e.target as Element).tagName,
      )
    ) {
      return;
    }
    if (e.code === "KeyQ") temporal.sidebar_open = !temporal.sidebar_open;
  });

  let width: number = $state(0);

  let is_desktop = $derived(width >= 1024);

  let sidebar_open = $derived(
    is_desktop ? temporal.sidebar_open : !temporal.sidebar_open,
  );
</script>

<svelte:window bind:innerWidth={width} />

<div class="flex w-full" class:p-2={is_desktop}>
  {#if sidebar_open}
    <SideBar {is_desktop} />
  {/if}
  <div class="main-window">
    {#if is_desktop}
      <TopBarDesktop />
    {:else}
      <TopBarMobile />
    {/if}
    <div
      class="browser"
      class:loading-pulse={vigi.tabs[vigi.current_tab].links[
        vigi.tabs[vigi.current_tab].current_link
      ].loading}
      bind:this={browserEl}
    >
      {@render children()}
    </div>
    {#if !is_desktop}
      <BotBar />
    {/if}
  </div>
</div>

<Toast />
