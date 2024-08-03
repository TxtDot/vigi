<script lang="ts">
    import Block from "./Block.svelte";
    import WindowControls from "./WindowControls.svelte";
    import type { StateTab } from "$lib/types";
    import { state } from "$lib/stores";
    import Tab from "./Tab.svelte";
    import { addTab } from "$lib/utils";
    import Button from "./Button.svelte";
    import Add from "$lib/icons/Add.svelte";
    import { fade } from "svelte/transition";

    export let collapsed = true;

    let tabs: StateTab[] = [];
    let currentTabIndex = 0;

    state.subscribe(async (state) => {
        tabs = state.tabs;
        currentTabIndex = state.current_tab_index;

        if (tabs.length === 0) {
            await addTab();
        }
    });
</script>

<Block className={`sidebar${collapsed ? "" : " collapsed"}`} draggable>
    <WindowControls />

    <div class="tabs-category">
        <div class="tabs-category-text">Open tabs</div>
        <Button onClick={addTab}>
            <Add />
        </Button>
    </div>

    <div class="tabs">
        {#each tabs as tab, i (tab.id)}
            <Tab {tab} active={currentTabIndex === i} index={i} />
        {/each}
    </div>
</Block>
