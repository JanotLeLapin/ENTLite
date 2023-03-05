<script lang="ts">
	import Header from "../components/Header.svelte";
	import type { PageServerData } from "./$types";

	export let data: PageServerData;
</script>

<Header user={data.user} unread={data.unread} />
<h1>Bonjour {data.user?.firstName}</h1>