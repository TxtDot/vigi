import type { Tag } from "@txtdot/dalet";

export interface VigiState {
  current_tab_index: number;
  tabs: StateTab[];
  favorites_tabs: StateTab[];

  top_bar_input: string;
  current_data: Tag[];
}

type TabType = "HomePage" | "Text";

export interface StateTab {
  ty: TabType;
  title: string;
  url: string;
  id: number;
}
