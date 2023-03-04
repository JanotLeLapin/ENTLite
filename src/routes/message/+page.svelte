<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;
</script>

<div class="px-24 py-8 space-y-4">
  {#each data.messages as msg}
    <div class="border border-slate-200 rounded-md p-4">
      <div class="flex items-center justify-between">
        <h3>{msg.subject}</h3>
        {#if msg.unread}
          <div class="bg-indigo-400 rounded-full p-2" />
        {/if}
      </div>
    </div>
  {/each}
</div>