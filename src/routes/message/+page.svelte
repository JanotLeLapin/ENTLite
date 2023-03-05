<script lang="ts">
  import Header from "../../components/Header.svelte";
  import type { PageServerData } from "./$types";

  export let data: PageServerData;
</script>

<Header user={data.user} unread={data.unread} />
<div class="">
  {#if data.messages}
    {#each data.messages as msg}
      <div class={`cursor-pointer border-b border-slate-200 px-4 py-2 ${msg.unread ? 'bg-slate-50 border-slate-200 font-bold' : 'bg-slate-100 border-slate-200'}`}>
        <div class={`flex items-center justify-between`}>
          <h3>{data.authors[msg.from].displayName} • {msg.subject.slice(0, 64).trim() + (msg.subject.length > 64 ? '...' : '')}</h3>
        </div>
      </div>
    {/each}
  {/if}
</div>