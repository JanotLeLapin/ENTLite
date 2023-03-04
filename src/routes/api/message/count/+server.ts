import { apiEndpoint } from '$lib/api';

export const GET = apiEndpoint('GET', (_) => { return { url: 'https://ent.iledefrance.fr/zimbra/count/INBOX?unread=true', } });
