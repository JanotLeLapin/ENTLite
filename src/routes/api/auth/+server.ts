import type { RequestHandler } from "@sveltejs/kit";

type Auth = {
  username: string,
  password: string,
}

export const POST = (async ({ request, fetch }) => {
  const { username, password }: Auth = await request.json();

  const res = await fetch('https://ent.iledefrance.fr/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    body: `email=${username}&password=${password}&callBack=https%253A%252F%252Fent.iledefrance.fr%252Ftimeline%252Ftimeline&details=`,
    redirect: 'manual',
  });

  return new Response('hey', {
    headers: {
      'set-cookie': res.headers.get('set-cookie'),
    } as any,
  });
}) satisfies RequestHandler;