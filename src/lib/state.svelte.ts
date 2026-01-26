import type { TemporalState, VigiState } from "./types";

export const vigi: VigiState = $state({
  current_tab: 0,
  tab_counter: 1,
  tabs: [
    {
      id: 0,
      current_link: 0,

      links: [
        {
          ty: "RENDER",
          render_type: "auto",
          uri: "gemini://geminiprotocol.net/docs/gemtext-specification.gmi",
        },
      ],
    },
  ],
});

export const temporal: TemporalState = $state({
  top_bar_open: false,
  first_load: true,
  sidebar_open: true,
  sidebar_scroll: 0,
  toasts: [],
});

let toastCounter = 0;

export function showToast(
  message: string,
  type: "success" | "error" | "info" = "info"
) {
  const id = ++toastCounter;
  temporal.toasts.push({ id, message, type });

  setTimeout(() => {
    temporal.toasts = temporal.toasts.filter((t) => t.id !== id);
  }, 4000);
}

export function dismissToast(id: number) {
  temporal.toasts = temporal.toasts.filter((t) => t.id !== id);
}
