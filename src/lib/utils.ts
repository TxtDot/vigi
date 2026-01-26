import { vigi } from "./state.svelte";
import type { DrovaError, SiteTab, TabLink, VigiState } from "./types";
import { goto } from "$app/navigation";
import { invoke } from "@tauri-apps/api/core";
import type { Page, Tag } from "@txtdot/dalet";

export function tabIndexById(id: number) {
  return vigi.tabs.findIndex((tab) => tab.id === id);
}

export function tabById(id: number) {
  return vigi.tabs[tabIndexById(id)];
}

export function currentLinkByTabId(id: number) {
  const tab = vigi.tabs[tabIndexById(id)];
  return tab.links[tab.current_link];
}

export function updateLinkByTabId(tab_id: number, val: Partial<TabLink>) {
  const tab = tabById(tab_id);

  tab.links[tab.current_link] = {
    ...tab.links[tab.current_link],
    ...val,
  };
}

export async function loadTab(id: number, uri: string) {
  let title: string | undefined;
  let body: Tag[] | undefined;
  let error: DrovaError | undefined;

  updateLinkByTabId(id, { loading: true });

  try {
    const page = (await invoke("process_url", {
      input: uri,
    })) as Page;

    body = page.body;
    title = page.title || undefined;

    const currLink = currentLinkByTabId(id);

    if (currLink.ty === "RENDER" && currLink.uri === uri)
      updateLinkByTabId(id, {
        body,
        title,
        error: undefined,
        loading: undefined,
      });
    else updateLinkByTabId(id, { loading: undefined });
  } catch (e) {
    error = convertError(e);
    updateLinkByTabId(id, { loading: undefined, error });
  }

  return { body, error };
}

export function convertError(e: any): DrovaError {
  if (typeof e === "string") return { message: e };
  else {
    let err: [string, string][] = Object.entries(e);

    return { message: err[0][0], body: err[0][1].toString() };
  }
}

export function saveState() {
  invoke("save_state", { state: vigi });
}

export async function loadState() {
  try {
    const new_state: VigiState = await invoke("get_state");

    vigi.current_tab = new_state.current_tab;
    vigi.tabs = new_state.tabs;
    vigi.tab_counter = new_state.tab_counter;
  } catch {}
}

export function currentTab() {
  return vigi.tabs[vigi.current_tab];
}

export function currentLink() {
  const current = currentTab();

  return current.links[current.current_link];
}

export function currentTabInnerURN() {
  return innerURN(currentLink());
}

export function innerURN(link: TabLink): string {
  if (link.ty === "RENDER") {
    return renderLink(link.uri);
  } else {
    // browser://main -> /browser/main, browser://settings -> /browser/settings
    // other browser:// URIs -> /browser?urn=...
    const knownPages = ["main", "history", "bookmarks", "settings"];
    if (knownPages.includes(link.uri)) {
      return `/browser/${link.uri}`;
    }
    return `/browser?urn=${encodeURIComponent(link.uri)}`;
  }
}

export function linkToURI(link: TabLink) {
  if (link.ty === "RENDER") return link.uri;
  else return `${link.ty.toLowerCase()}://${link.uri}`;
}

export function renderLink(uri: string, relative?: boolean) {
  if (relative) {
    try {
      return `/render?uri=${encodeURIComponent(
        new URL(uri, currentLink().uri).toString()
      )}`;
    } catch {}
  }

  return `/render?uri=${encodeURIComponent(uri)}`;
}

export function formatInputLink(link: TabLink): string {
  const urlString = linkToURI(link);
  try {
    if (link.ty === "RENDER") {
      const url = new URL(urlString);
      return decodeURI(urlString.replace(`${url.protocol}//`, ""));
    } else {
      return link.title || urlString;
    }
  } catch {
    return urlString;
  }
}

export function gotoTBI(tbi: string) {
  const url = isUrl(tbi);
  if (url) {
    if (url.protocol !== "browser:") {
      goto(renderLink(tbi));
    } else {
      // browser://main -> /browser/main
      const page = url.hostname + (url.pathname !== "/" ? url.pathname : "");
      const knownPages = ["main", "history", "bookmarks", "settings"];
      if (knownPages.includes(page)) {
        goto(`/browser/${page}`);
      } else {
        goto(`/browser?urn=${encodeURIComponent(page)}`);
      }
    }
  } else {
    goto(`/browser/search?q=${encodeURIComponent(tbi)}`);
  }
}

export function isUrl(s: string) {
  try {
    return new URL(s);
  } catch {
    return false;
  }
}

export const newTabLink: TabLink = {
  ty: "BROWSER",
  uri: "main",
  title: "New tab",
};
