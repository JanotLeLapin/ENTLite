<script lang="ts">
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
  }
</script>

<input type="text" bind:value={username} />
<input type="password" bind:value={password} />
<button on:click={login}>Se connecter</button>