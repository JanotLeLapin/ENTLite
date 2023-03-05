import type { Cookies } from '@sveltejs/kit';
import { serverFetch } from './api';

export type UserInfo = {
  login: string,
  firstName: string,
  lastName: string,
}

export type User = {
  id: string,
  login: string,
  displayName: string,
  type: string[],
}

export const fetchUserInfo = async (cookie: Cookies): Promise<UserInfo> =>
  await serverFetch(cookie, 'GET', 'https://ent.iledefrance.fr/auth/oauth2/userinfo');

export const fetchUser = async (cookie: Cookies, id: string): Promise<User> => {
  const res = await serverFetch(cookie, 'GET', `https://ent.iledefrance.fr/userbook/api/person?id=${id}`);
  return res.result[0];
}
