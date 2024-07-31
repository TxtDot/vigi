<script lang="ts">
    import ArrowLeft from "$lib/icons/ArrowLeft.svelte";
    import ArrowRight from "$lib/icons/ArrowRight.svelte";
    import OpenTabs from "$lib/icons/OpenTabs.svelte";
    import Reload from "$lib/icons/Reload.svelte";
    import SidebarLeft from "$lib/icons/SidebarLeft.svelte";
    import SidebarRight from "$lib/icons/SidebarRight.svelte";
    import { state } from "$lib/stores";
    import { updateAndLoadInput } from "$lib/utils";
    import Block from "./Block.svelte";
    import Button from "./Button.svelte";

    export let onBack = () => {};
    export let onForward = () => {};

    export let sidebarOpen = true;

    let currentInput = "";
    let input = "";
    let tabsOpen = 0;

    state.subscribe((val) => {
        input = val.top_bar_input;
        currentInput = decodeURIComponent(input);
        tabsOpen = val.tabs.length;
    });

    let iEl: HTMLInputElement;
</script>

<div class="top-bar">
    <Block className="top-bar-buttons">
        <Button
            className="hide-sidebar-button"
            onClick={() => (sidebarOpen = !sidebarOpen)}
        >
            {#if sidebarOpen}
                <SidebarLeft />
            {:else}
                <SidebarRight />
            {/if}
        </Button>
        <Button className="back-button" onClick={onBack}><ArrowLeft /></Button>
        <Button className="forward-button" onClick={onForward}>
            <ArrowRight />
        </Button>
        <Button
            className="reload-button"
            onClick={() => updateAndLoadInput(input)}
        >
            <Reload />
        </Button>

        <Button className="open-tabs-page-button"><OpenTabs /></Button>
    </Block>

    <input
        type="text"
        placeholder="Search or enter URL"
        class="search-input"
        bind:value={currentInput}
        bind:this={iEl}
        on:keypress={(e) => {
            if (e.key === "Enter") {
                updateAndLoadInput(currentInput);
                iEl.blur();
            }
        }}
        on:focus={() => {
            currentInput = input;
            setTimeout(() => {
                iEl.select();
                iEl.scrollLeft = iEl.scrollWidth;
            }, 1);
        }}
        on:focusout={() => {
            currentInput = decodeURIComponent(input);
        }}
    />
</div>
