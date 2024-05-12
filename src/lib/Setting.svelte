<script lang="ts">
  import { Store } from "tauri-plugin-store-api";
  import { onMount } from "svelte";

  const store = new Store("settings.json");

  let fontFamily = "";
  let cssSnippet = "";

  onMount(async () => {
    fontFamily = (await store.get("font-family")) ?? "";
    cssSnippet = (await store.get("css-snippet")) ?? "";
  });

  async function handleSubmit() {
    store.set("font-family", fontFamily);
    store.set("css-snippet", cssSnippet);

    store.save();
  }
</script>

<form on:submit|preventDefault={handleSubmit}>
  <div class="row">
    <label for="font-family">字体</label>
    <input name="font-family" bind:value={fontFamily} />
  </div>
  <div class="row">
    <label for="css-snippet">CSS片段</label>
    <textarea name="css-snippet" bind:value={cssSnippet} />
  </div>
  <div class="row" style="justify-content: flex-end; gap: 1rem"></div>

  <button type="submit" style="background: #2da5ff; color: #fff">应用</button>
</form>

<style>
</style>
