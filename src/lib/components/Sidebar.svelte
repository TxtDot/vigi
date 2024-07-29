<script lang="ts">
  import { slide } from "svelte/transition";

  import Block from "./Block.svelte";
  import WindowControls from "./WindowControls.svelte";
  import type { StateTab } from "$lib/types";
  import { state } from "$lib/stores";
  import Tab from "./Tab.svelte";
  import { addTab } from "$lib/utils";
  import Button from "./Button.svelte";
  import Add from "$lib/icons/Add.svelte";

  export let collapsed = true;

  let tabs: StateTab[] = [];
  let currentTabIndex = 0;

  state.subscribe(async (state) => {
    tabs = state.tabs.toReversed();
    currentTabIndex = state.current_tab_index;

    if (tabs.length === 0) {
      await addTab();
    }
  });
</script>

<Block className={`sidebar${collapsed ? "" : " collapsed"}`} draggable>
  {#if collapsed}
    <WindowControls />

    <div class="open-tabs">
      Open tabs
      <Button onClick={addTab}>
        <Add />
      </Button>
    </div>

    <div class="tabs">
      {#each tabs as tab, i (tab.id)}
        <Tab
          {tab}
          active={currentTabIndex === tabs.length - 1 - i}
          id={tabs.length - 1 - i}
        />
      {/each}
    </div>
  {/if}
</Block>
