import { invoke } from "@tauri-apps/api";
import { state, topBarInput } from "./stores";
import type { StateTab, VigiState } from "./types";

export function updateVigiState() {
  invoke("get_state")
    .then((r) => {
      let st = r as VigiState;

      state.set(st);

      topBarInput.set(st.tabs[st.current_tab_index].url);
    })
    .catch((err) => console.log(err));
}

export async function addTab() {
  await invoke("add_tab");

  updateVigiState();
}

export async function selectTab(index: number) {
  await invoke("select_tab", { index });

  updateVigiState();
}

export async function removeTab(index: number) {
  await invoke("remove_tab", { index });
  updateVigiState();
}
