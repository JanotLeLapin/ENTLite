export type Message = {
  id: string,
  date: number,
  subject: string,
  unread: boolean,
  from: string,
  to: string[],
}

export type Folder = 'Inbox' | 'Sent' | 'Drafts' | 'Trash' | 'Junk';

export const messages = async (folder?: Folder, page?: number, unread?: boolean, fn?: typeof fetch): Promise<Message[]> => {
  return await (fn || fetch)(`/api/message?folder=${folder || 'Inbox'}&page=${page || 0}&unread=${unread || false}`).then(res => res.json());
}

export const unreadCount = async (fn?: typeof fetch): Promise<number> => {
  const json = await (fn || fetch)('/api/message/count').then(res => res.json());
  return json.count;
}
