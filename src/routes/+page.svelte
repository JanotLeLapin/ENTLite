<script lang="ts">
	import type { PageServerData } from "./$types";

	export let data: PageServerData;
</script>

<h1>Bonjour {data.user.firstName}</h1>