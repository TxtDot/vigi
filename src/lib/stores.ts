import { writable, type Writable } from "svelte/store";
import type { VigiState } from "./types";

export const topBarInput: Writable<string> = writable("");

export const state: Writable<VigiState> = writable();
