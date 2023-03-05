import type { Cookies } from '@sveltejs/kit';

export type FetchAuth = {
  cookie: Cookies,
  fetch: typeof fetch,
}

export type ApiRequest = {
  url: string,
  body?: any,
}

export const serverFetch = async (cookie: Cookies, method: string, url: string, body?: string) => {
  const xsrf = cookie.get('XSRF-TOKEN') || '';
  const cookieString = `oneSessionId=${cookie.get('oneSessionId')}; XSRF-TOKEN=${xsrf}`
  const res = await fetch(url, {
    method,
    headers: {
      'Cookie': cookieString,
      'X-XSRF-TOKEN': xsrf,
    },
    body,
  });
  return res.json();
}
