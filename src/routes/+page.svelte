<script lang="ts">
	import Header from "../components/Header.svelte";
	import type { PageServerData } from "./$types";

	export let data: PageServerData;
</script>

<svelte:head>
  <title>ENTLite</title>
</svelte:head>

<Header user={data.user} unread={data.unread} />

{#if data.user}
	<div class="sm:px-24 sm:text-left py-32 text-center space-y-2">
		<h1 class="font-bold">Bonjour {data.user.firstName}</h1>
		<p class="text-xl">Bienvenue sur votre expérience ENT, redéfinie.</p>
	</div>
{:else}
	<div class="py-32 text-center">
	  <h1>Bienvenue</h1>
	</div>
{/if}

<style>
	h1 {
		@apply text-5xl font-bold;
	}
</style>
