import { fetchMessages } from '$lib/message';
import type { PageServerLoad } from './$types';

export const load = (async ({ cookies, locals }) => {
  try {
    const messages = await fetchMessages(cookies);
    console.log(messages)
    return {
      user: locals.user,
      messages,
    };
  } catch (e) {
    console.error(e)
    return {
      user: null,
    }
  }
}) satisfies PageServerLoad;