import { apiEndpoint } from '$lib/api';

export const GET = apiEndpoint('GET', ({ params, url }) => {
  const sp = url.searchParams;
  return { url: `https://ent.iledefrance.fr/zimbra/list?folder=/${sp.get('folder')}&page=${sp.get('page')}&unread=${sp.get('unread')}` }
});