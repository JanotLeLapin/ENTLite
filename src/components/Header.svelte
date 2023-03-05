<script lang="ts">
	import type { UserInfo } from '$lib/user';

  export let user: UserInfo | null;
  export let unread: number = 0;
</script>

<div class="flex justify-between items-center px-32 py-4 border-b border-gray-200">
  <div class="flex items-center">
    <a href="/"><h3 class="text-3xl font-bold cursor-pointer">ENTLite</h3></a>
    <ul class="ml-16 space-x-8">
      <a href="/message"><li>Messages <span class="text-slate-100 text-xs bg-indigo-500 font-medium rounded-full ml-2 px-1.5 py-0.5">{unread}</span></li></a>
    </ul>
  </div>
  <div class="space-x-4">
    {#if user}
      <p>{user.firstName} {user.lastName}</p>
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
