<script lang="ts">
    import "../app.css";

    import TopBar from "$lib/components/TopBar.svelte";
    import Sidebar from "$lib/components/Sidebar.svelte";
    import BrowserWindow from "$lib/components/BrowserWindow.svelte";

    import { invoke } from "@tauri-apps/api/tauri";
    import { loadInput, updateVigiState } from "$lib/utils";
    import { isLoading } from "$lib/stores";

    let sidebarOpen = true;

    (async () => {
        isLoading.set(true);
        await invoke("setup");
        await updateVigiState();
        await loadInput();
        isLoading.set(false);
    })();

    document.addEventListener("keypress", (e: KeyboardEvent) => {
        if (
            ["INPUT", "TEXTAREA", "SELECT", "OPTION"].includes(
                (e.target as Element).tagName,
            )
        ) {
            return;
        }
        if (e.code === "KeyQ") sidebarOpen = !sidebarOpen;
    });
</script>

<div
    class={`common-window${sidebarOpen ? "" : " collapsed"}`}
    data-tauri-drag-region
>
    <Sidebar bind:collapsed={sidebarOpen} />

    <div class="main-window">
        <TopBar bind:sidebarOpen />
        <BrowserWindow />
    </div>
</div>
