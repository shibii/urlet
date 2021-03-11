<script>
  import { generate_urlet } from "./services/api";

  let url;
  let valid;
  $: {
    try {
      let test_url = new URL(url);
      valid = test_url.protocol.match(/https?:/) ? true : false;
    } catch (err) {
      valid = false;
    }
  }

  const generate = () => {
    generate_urlet(url)
      .then((res) => console.log(res))
      .catch((err) => console.log(err));
  };
</script>

<form on:submit|preventDefault={generate}>
  <input type="text" bind:value={url} />
  <input type="submit" value="generate" disabled={!valid} />
</form>
