import { invoke } from "@tauri-apps/api";
import { isLoading, state } from "./stores";
import type { VigiState } from "./types";

export async function updateVigiState() {
  try {
    let st = await invoke("get_js_state");
    state.set(st as VigiState);
  } catch (e) {
    console.log(e);
  }
}

export async function updateInput(input: string) {
  isLoading.set(true);

  try {
    await invoke("update_input", { input });
  } catch (e) {
    console.log(e);
  } finally {
    await updateVigiState();
    isLoading.set(false);
  }
}

export async function addTab() {
  await invoke("add_tab");
  await updateVigiState();
  await loadTab();
}

export async function selectTab(index: number) {
  await invoke("select_tab", { index });
  await updateVigiState();
  await loadTab();
}

export async function removeTab(index: number) {
  await invoke("remove_tab", { index });
  await updateVigiState();
  await loadTab();
}

export async function loadTab() {
  isLoading.set(true);

  await invoke("load_tab");
  await updateVigiState();

  isLoading.set(false);
}
