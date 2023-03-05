import type { Cookies } from "@sveltejs/kit";
import { serverFetch } from "./api";

export type Message = {
  id: string,
  date: number,
  subject: string,
  unread: boolean,
  from: string,
  to: string[],
}

export type Folder = 'Inbox' | 'Sent' | 'Drafts' | 'Trash' | 'Junk';

export const fetchMessages = async (cookie: Cookies, folder?: Folder, page?: number, unread?: boolean): Promise<Message[]> =>
  await serverFetch(cookie, 'GET', `https://ent.iledefrance.fr/zimbra/list?folder=/${folder || 'Inbox'}&page=${page || 0}&unread=${unread || false}`);


export const fetchUnreadCount = async (cookie: Cookies): Promise<number> =>
  await serverFetch(cookie, 'GET', 'https://https://ent.iledefrance.fr/zimbra/count/INBOX?unread=true').then(res => res.count);
