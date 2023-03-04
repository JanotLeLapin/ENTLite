<script lang="ts">
	import Header from "../components/Header.svelte";
	import type { PageServerData } from "./$types";

	export let data: PageServerData;
</script>

<Header />
<h1>Bonjour {data.user.firstName}</h1>