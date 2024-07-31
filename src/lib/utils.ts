import { invoke } from "@tauri-apps/api";
import { isLoading, state } from "./stores";
import type { VigiState } from "./types";

export async function updateVigiState() {
  try {
    let st = await invoke("get_js_state");
    state.set(st as VigiState);
  } catch (e) {
    writeError(e);
  }
}

export async function updateInput(input: string) {
  isLoading.set(true);

  try {
    await invoke("update_input", { input });
    await updateVigiState();
  } catch (e) {
    writeError(e, input);
  } finally {
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

  try {
    await invoke("load_tab");
    await updateVigiState();
  } catch (e) {
    writeError(e);
  } finally {
    isLoading.set(false);
  }
}

function writeError(e: unknown, input?: string) {
  state.update((st) => {
    st.current_data = [{ id: 0, body: `Error: ${e}`, argument: null }];

    if (input) st.top_bar_input = input;

    return st;
  });
}
