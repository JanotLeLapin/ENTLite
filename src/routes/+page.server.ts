import type { PageServerLoad } from './$types';

export const load = (async ({ locals }) => {
  try {
    return { user: locals.user }
  } catch (e) {
    return { user: null };
  }
}) satisfies PageServerLoad;