<script lang="ts">
	import { userInfo, type UserInfo } from '$lib/user';
	import { onMount } from 'svelte';
  import '../app.css';
  import Header from '../components/Header.svelte';

  let user: UserInfo;
  onMount(() => userInfo().then(res => user = res).catch(console.error));
</script>

<Header {user} />
<slot />