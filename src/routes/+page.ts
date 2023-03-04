import { userInfo } from '$lib/user';
import type { PageLoad } from './$types';

export const load = (async ({ fetch }) => {
  try {
    const user = await userInfo(fetch);
    return { user }
  } catch (e) {
    return { unauthorized: true };
  }
}) satisfies PageLoad;