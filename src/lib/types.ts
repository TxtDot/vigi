import type { Tag } from "@txtdot/dalet";

export type TabType = "RENDER" | "BROWSER";

export interface DrovaError {
  body?: string;
  message: string;
}

export interface VigiState {
  current_tab: number;
  tab_counter: number;
  tabs: SiteTab[];
}

export interface SiteTab {
  id: number;
  current_link: number;
  links: TabLink[];
}

export interface TabLink {
  title?: string;
  body?: Tag[];

  ty: TabType;
  uri: string;

  error?: DrovaError;
  loading?: true;
}

export interface TemporalState {
  top_bar_open: boolean;
  first_load: boolean;
  sidebar_scroll: number;
  sidebar_open: boolean;
  toasts: ToastMessage[];
}

// History

export interface HistoryEntry {
  uri: string;
  title?: string;
  timestamp: number;
  visit_count: number;
}

// Bookmarks

export interface Bookmark {
  uri: string;
  title: string;
  tags: string[];
  created_at: number;
}

// Settings

export interface Settings {
  theme: string;
  custom_css?: string;
  search_engine: string;
  home_page: string;
}

// Toast notifications

export interface ToastMessage {
  id: number;
  type: "success" | "error" | "info";
  message: string;
}
