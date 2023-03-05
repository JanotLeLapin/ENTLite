import type { Cookies } from '@sveltejs/kit';
import { serverFetch } from './api';

export type UserInfo = {
  login: string,
  firstName: string,
  lastName: string,
}

export const fetchUserInfo = async (cookie: Cookies): Promise<UserInfo> => await serverFetch(cookie, 'GET', 'https://ent.iledefrance.fr/auth/oauth2/userinfo');
