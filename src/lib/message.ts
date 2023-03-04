export const unreadCount = async (fn?: typeof fetch): Promise<number> => {
  let json = await (fn || fetch)('/api/message/count').then(res => res.json());
  return json.count;
}
