<script lang="ts">
	import type { PageData } from "./$types";

	export let data: PageData;
</script>

<h1>Bonjour {data?.user?.firstName}</h1>