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
    let loading = false;

    isLoading.subscribe((val) => {
        loading = val;
    });
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
        class="tab"
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
            {#if loading && active}
                <GooLoad />
            {/if}
        </div>

        <div>
            {tab.title}
        </div>
    </button>
</div>
