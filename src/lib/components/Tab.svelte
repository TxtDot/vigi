<script lang="ts">
    import type { StateTab } from "$lib/types";
    import { removeTab, selectTab } from "$lib/utils";
    import { slide } from "svelte/transition";
    import Close from "$lib/icons/Close.svelte";
    import { isLoading } from "$lib/stores";
    import GooLoad from "$lib/icons/GooLoad.svelte";

    export let active = false;
    export let tab: StateTab;

    export let index: number;

    let tabElement: HTMLButtonElement;

    let hovered = false;
</script>

<div
    class="flex gap-1 items-center shrink"
    on:mouseenter={() => (hovered = true)}
    on:mouseleave={() => (hovered = false)}
    role="tab"
    tabindex={index}
>
    {#if hovered}
        <button
            class="close-button"
            transition:slide={{ duration: 100, axis: "x" }}
            on:click={() => removeTab(index)}
        >
            <Close />
        </button>
    {/if}

    <button
        class={`tab${$isLoading && active ? " loading" : ""}`}
        class:active
        transition:slide={{ duration: 100 }}
        bind:this={tabElement}
        on:click={() => {
            if (!active) {
                selectTab(index);
            }
        }}
    >
        <div>
            {tab.title || tab.url}
        </div>
    </button>
</div>
