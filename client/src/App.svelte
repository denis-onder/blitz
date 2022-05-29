<script lang="ts">
  import Feed from "./components/Feed.svelte";
  import { mocks } from "./mocks";
</script>

<main id="main">
  {#each mocks as feed}
    <Feed {feed} />
  {/each}
</main>

<style>
  main {
    min-height: 100vh;
    background-color: #333;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    font-family: "Roboto", Arial, Helvetica, sans-serif;
    color: #fff;
  }
</style>
