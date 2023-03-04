<script lang="ts">
	import Header from "../components/Header.svelte";
	import type { PageData } from "./$types";

	export let data: PageData;
</script>

<Header />
<h1>Bonjour {data.user.firstName}</h1>