<script lang="ts">
	import { goto } from "$app/navigation";

  let username = ''
  let password = ''

  const login = async () => {
    const res = await fetch('/api/auth', {
      method: 'POST',
      body: JSON.stringify({
        username,
        password,
      }),
      headers: {
        'content-type': 'application/json',
      },
    });

    if (res.status == 200)
      goto('/');
  }

  const keypress = (k: KeyboardEvent) => {
    if (k.key === 'Enter') login();
  }
</script>

<svelte:head>
  <title>Messages • ENTLite</title>
</svelte:head>

<input type="text" bind:value={username} on:keypress={keypress} />
<input type="password" bind:value={password} on:keypress={keypress}  />
<button on:click={login}>Se connecter</button>