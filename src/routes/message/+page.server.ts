import { fetchMessages } from '$lib/message';
import type { PageServerLoad } from './$types';

export const load = (async ({ cookies, locals }) => {
  try {
    const messages = await fetchMessages(cookies);
    return {
      user: locals.user,
      unread: locals.unread,
      messages,
    };
  } catch (e) {
    return {
      user: null,
      unread: 0,
    }
  }
}) satisfies PageServerLoad;