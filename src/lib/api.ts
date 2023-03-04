import type { Cookies, RequestEvent, RequestHandler } from '@sveltejs/kit';

export type FetchAuth = {
  cookie: Cookies,
  fetch: typeof fetch,
}

export type ApiRequest = {
  url: string,
  body?: any,
}

export const apiEndpoint = (method: string, req: (r: RequestEvent) => ApiRequest): RequestHandler => {
  return (async (request) => {
    const r = req(request);
    
    const cookie = request.request.headers.get('cookie') as string;
    const xsrf = cookie.split('XSRF-TOKEN=')[1].split(';')[0];
    const json = await fetch(r.url, {
      method,
      headers: {
        'Cookie': cookie,
        'X-XSRF-TOKEN': xsrf,
      },
      body: r.body,
    }).then(res => res.json());
    return new Response(JSON.stringify(json));
  })
}
