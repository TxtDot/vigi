import { manageLink } from "$lib/management";
import { redirect } from "@sveltejs/kit";

export const load = ({ url }) => {
  const urn = url.searchParams.get("urn");

  if (!urn) {
    redirect(307, "/browser/main");
  }

  manageLink("BROWSER", urn.replace(/\/$/, ""));
};
