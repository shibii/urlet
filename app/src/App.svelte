<script>
  import { generate_urlet } from "./services/api";

  let url;

  const generate = () => {
    generate_urlet(url)
      .then((res) => console.log(res))
      .catch((err) => console.log(err));
  };
</script>

<form on:submit|preventDefault={generate}>
  <input type="text" bind:value={url} />
  <input type="submit" value="generate" />
</form>
