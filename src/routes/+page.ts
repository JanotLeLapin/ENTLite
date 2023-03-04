import { userInfo } from '$lib/user';
import type { PageLoad } from './$types';

export const load = (async ({ fetch }) => {
  const user = await userInfo(fetch);
  return { user }
}) satisfies PageLoad;