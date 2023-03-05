import type { PageServerLoad } from './$types';

export const load = (async ({ locals }) => {
  try {
    return {
      user: locals.user,
      unread: locals.unread,
    };
  } catch (e) {
    return {
      user: null,
      unread: 0,
    };
  }
}) satisfies PageServerLoad;