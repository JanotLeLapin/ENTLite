import type { Cookies, RequestHandler } from '@sveltejs/kit';

export type FetchAuth = {
  cookie: Cookies,
  fetch: typeof fetch,
}

export const apiEndpoint = (url: string, method: string, body?: (r: Request) => any): RequestHandler => {
  return (async ({ request, fetch }) => {
    const cookie = request.headers.get('cookie') as string;
    const xsrf = cookie.split('XSRF-TOKEN=')[1].split(';')[0];
    const json = await fetch(url, {
      method,
      headers: {
        'Cookie': cookie,
        'X-XSRF-TOKEN': xsrf,
      },
      body: body ? body(request) : undefined,
    }).then(res => res.json());
    return new Response(JSON.stringify(json));
  })
}
