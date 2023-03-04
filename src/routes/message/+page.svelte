<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;
</script>

<div class="">
  {#each data.messages as msg}
  <div class={`cursor-pointer border-b border-slate-300 px-4 py-2 ${msg.unread ? 'bg-slate-100 border-slate-200 font-bold' : 'bg-slate-200 border-slate-300'}`}>
      <div class={`flex items-center justify-between`}>
        <h3>{msg.subject.slice(0, 64).trim() + (msg.subject.length > 64 ? '...' : '')}</h3>
      </div>
    </div>
  {/each}
</div>