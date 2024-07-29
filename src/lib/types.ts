export interface VigiState {
  current_tab_index: number;
  tabs: StateTab[];
  favorites_tabs: StateTab[];
}

type TabType = "HomePage" | "Text";

export interface StateTab {
  ty: TabType;
  title: string;
  url: string;
  id: number;
}
