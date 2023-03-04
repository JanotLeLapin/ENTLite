import { apiEndpoint } from "$lib/api";

export const GET = apiEndpoint('https://ent.iledefrance.fr/auth/oauth2/userinfo', 'GET');
