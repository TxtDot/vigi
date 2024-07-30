import { writable, type Writable } from "svelte/store";
import type { VigiState } from "./types";

export const state: Writable<VigiState> = writable({
  current_tab_index: 0,
  tabs: [{ id: 0, ty: "HomePage", title: "Home", url: "" }],
  favorites_tabs: [],
  top_bar_input: "",
  current_data: [],
});

export const isLoading: Writable<boolean> = writable(false);
