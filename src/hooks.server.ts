import { fetchUnreadCount } from '$lib/message';
import { fetchUserInfo } from '$lib/user';
import type { Handle } from '@sveltejs/kit';

export const handle = (async ({ event, resolve }) => {
  try {
    const user = await fetchUserInfo(event.cookies);
    event.locals.user = user;
  } catch (e) {
    event.locals.user = null;
  }

  try {
    const unreadCount = await fetchUnreadCount(event.cookies);
    event.locals.unread = unreadCount;
  } catch (e) {
    event.locals.unread = 0;
  }

  return await resolve(event);
}) satisfies Handle;
