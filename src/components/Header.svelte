<script lang="ts">
	import { unreadCount } from '$lib/message';
	import type { UserInfo } from '$lib/user';
	import { onMount } from 'svelte';

  export let user: UserInfo;

  let loggedIn = false;

  let unread = 0;
  onMount(async () => {
    unreadCount().then(res => {
      unread = res;
      loggedIn = true;
    }).catch(console.error);
  })
</script>

<div class="flex justify-between items-center px-32 py-4 border-b border-gray-200">
  <div class="flex items-center">
    <a href="/"><h3 class="text-3xl font-bold cursor-pointer">ENTLite</h3></a>
    <ul class="ml-16 space-x-8">
      <a href="/message"><li>Messages <span class="text-slate-100 text-xs bg-indigo-500 font-medium rounded-full ml-2 px-1.5 py-0.5">{unread}</span></li></a>
    </ul>
  </div>
  <div class="space-x-4">
    {#if loggedIn}
      <p>{user?.firstName}</p>
    {:else}
      <a href="/auth">Connexion</a>
    {/if}
  </div>
</div>

<style>
  li {
    @apply inline;
  }
</style>
