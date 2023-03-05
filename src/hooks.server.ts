import { fetchUserInfo } from '$lib/user';
import type { Handle } from '@sveltejs/kit';

export const handle = (async ({ event, resolve }) => {
  try {
    const user = await fetchUserInfo(event.cookies);
    event.locals.user = user;
  } catch (e) {
    event.locals.user = null;
  }

  return await resolve(event);
}) satisfies Handle;
