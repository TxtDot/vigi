import { manageLink } from "$lib/management";
import type { DrovaError } from "$lib/types.js";
import {
  currentTab,
  currentLink,
  loadTab,
  updateLinkByTabId,
} from "$lib/utils.js";
import type { Tag } from "@txtdot/dalet";
import { redirect } from "@sveltejs/kit";

export async function load({ url }) {
  const uri = url.searchParams.get("uri");

  if (!uri) {
    redirect(307, "/browser/main");
  }

  const currTab = currentTab();

  updateLinkByTabId(currTab.id, { loading: undefined });

  const currLink = currentLink();

  let body: Tag[] | undefined;
  let error: DrovaError | undefined;

  if (
    currLink.ty === "RENDER" &&
    currLink.uri === uri &&
    currLink.body &&
    currLink.title
  ) {
    body = currLink.body;
  } else if (currLink.error) {
    error = currLink.error;
  } else {
    manageLink("RENDER", uri);

    const res = await loadTab(currTab.id, uri);

    body = res.body;
    error = res.error;
  }

  return {
    body,
    error,
  };
}
