import { invoke as inv } from "@tauri-apps/api";
import { isLoading, state } from "./stores";
import type { VigiState } from "./types";
import { get } from "svelte/store";
import type { InvokeArgs } from "@tauri-apps/api/tauri";

export async function updateVigiState() {
  let st = await invoke("get_js_state");
  state.set(st as VigiState);
}

export async function updateAndLoadInput(input: string, newTab?: boolean) {
  await invoke("update_input", { input, newTab: !!newTab });
  await updateVigiState();

  await invoke("load_input_force");
  await updateVigiState();
}

export async function addTab() {
  await invoke("add_tab");
  await updateVigiState();
}

export async function selectTab(index: number) {
  await invoke("select_tab", { index });
  await updateVigiState();
  await loadInput();
}

export async function removeTab(index: number) {
  await invoke("remove_tab", { index });
  await updateVigiState();
  setTimeout(loadInput, 150);
}

export async function loadInput() {
  await invoke("load_input");
  await updateVigiState();
}

export async function goToLink(link: string, newTab?: boolean) {
  const top_bar_input = get(state).top_bar_input;
  const new_input = new URL(link, top_bar_input);
  await updateAndLoadInput(new_input.toString(), newTab);
}

function writeError(e: unknown, input?: string) {
  state.update((st) => {
    st.current_data = [{ id: 0, body: `Error: ${e}`, argument: null }];

    if (input) st.top_bar_input = input;

    return st;
  });
}

export async function invoke(f: string, args?: InvokeArgs): Promise<unknown> {
  isLoading.set(true);
  try {
    let result = await inv(f, args);
    isLoading.set(false);
    return result;
  } catch (e) {
    writeError(e);
    isLoading.set(false);
    throw new Error("Invoke failed");
  }
}
