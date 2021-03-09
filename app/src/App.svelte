<script>
  let url;
  const submit = () => {};
</script>

<form on:submit|preventDefault={submit}>
  <input type="text" bind:value={url} />
  <input type="submit" value="generate" />
</form>
