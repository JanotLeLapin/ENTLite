import { fetchMessages } from '$lib/message';
import { fetchUser, type User } from '$lib/user';
import type { PageServerLoad } from './$types';

export const load = (async ({ cookies, locals }) => {
  try {
    const messages = await fetchMessages(cookies);
    const authors: { [key: string]: User } = {};
    await Promise.all(messages.map(async msg => {
      const uid = msg.from;
      if (!authors[uid]) authors[uid] = await fetchUser(cookies, uid);
    }));
    return {
      user: locals.user,
      unread: locals.unread,
      authors,
      messages,
    };
  } catch (e) {
    return {
      user: null,
      unread: 0,
    }
  }
}) satisfies PageServerLoad;