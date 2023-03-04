import { messages } from "$lib/message";
import type { PageLoad } from "./$types";

export const load = (async ({ fetch }) => {
  const msgs = await messages('Inbox', 0, false, fetch);
  return {
    messages: msgs,
  }
}) satisfies PageLoad;
