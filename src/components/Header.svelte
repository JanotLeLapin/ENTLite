<script lang="ts">
	import { unreadCount } from '$lib/message';
	import { onMount } from 'svelte';

  let unread = 0;
  onMount(async () => {
    unread = await unreadCount();
  })
</script>

<div class="flex justify-between items-center px-32 py-4 border-b border-gray-200">
  <div class="flex items-center">
    <a href="/"><h3 class="text-3xl font-bold cursor-pointer">ENTLite</h3></a>
    <ul class="ml-16 space-x-8">
      <li>Messages <span class="text-slate-100 text-xs bg-indigo-500 font-medium rounded-full ml-2 px-1.5 py-0.5">{unread}</span></li>
    </ul>
  </div>
  <div class="space-x-4">
    <a href="/auth">Connexion</a>
  </div>
</div>

<style>
  li {
    @apply inline;
  }
</style>
