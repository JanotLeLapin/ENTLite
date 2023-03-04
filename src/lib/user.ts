export type UserInfo = {
  login: string,
  firstName: string,
  lastName: string,
}

export const userInfo = async (fn?: typeof fetch): Promise<UserInfo> => await (fn || fetch)('/api/user').then(res => res.json()) as UserInfo;
