<script>
  import "./global.css";
  import { generate_urlet } from "./services/api";

  let url = "";
  let valid = false;
  let urlet;
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
      .then((res) => (urlet = window.location.href + res))
      .catch((err) => console.log(err));
  };
  const copy = () => {
    try {
      navigator.clipboard.writeText(urlet).then();
    } catch {}
  };
</script>

<div class="content">
  {#if !urlet}
    <h1>urlet</h1>
    <p>generate an alternative url that redirects to the same address</p>
    <form on:submit|preventDefault={generate}>
      <label for="url">URL:</label>
      <input
        id="url"
        class:valid
        class:empty={url == ""}
        type="text"
        bind:value={url}
        autocomplete="off"
        placeholder="https://this.is/a?way%20too%20long%20link%20that%20just%20keeps%20on%20going%20and%20going%20and%20going%20and%20going"
      />
    </form>
  {:else}
    <p class="urlet">
      {urlet}
      <button class="copy-button" on:click={copy}>
        <img src="/copy.svg" class="copy" alt="copy" /></button
      >
    </p>
  {/if}
</div>

<style>
  img {
    display: inline;
    width: 32px;
    height: 32px;
    margin-left: 10px;
    vertical-align: bottom;
    transform: scale(-1, 1);
  }
  .copy-button {
    background: none;
    color: inherit;
    border: none;
    padding: 0;
    font: inherit;
    cursor: pointer;
    outline: inherit;
  }
  .content {
    display: flex;
    flex-direction: column;
    padding: 1rem;
    margin: auto;
    max-width: 800px;
  }
  h1 {
    font-size: 4rem;
    text-align: center;
    margin: 0.5rem 0;
  }
  p {
    font-size: 3rem;
    margin: 0.5rem 0;
  }
  p.urlet {
    font-size: 1.5rem;
    text-align: center;
  }
  label {
    display: block;
    margin-top: 1em;
    font-size: small;
  }
  input {
    display: block;
    font-family: inherit;
    font-size: larger;
    width: 100%;
    padding: 0.2em;
    margin-bottom: 1em;
    border: 3px solid darkgrey;
    border-radius: 5px;
    background-color: whitesmoke;
    outline: none;
  }
  input.valid {
    border-color: seagreen;
    box-shadow: 0 0 10px seagreen;
  }
  input:not(.empty, .valid) {
    border-color: maroon;
    box-shadow: 0 0 10px maroon;
  }
</style>
