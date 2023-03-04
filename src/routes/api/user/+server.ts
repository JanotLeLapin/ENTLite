import { apiEndpoint } from '$lib/api';

export const GET = apiEndpoint('GET', (_) => { return { url: 'https://ent.iledefrance.fr/auth/oauth2/userinfo', } });
