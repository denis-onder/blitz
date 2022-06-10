<script lang="ts">
  import { Feed as FeedType } from "./types/Feed";
  import { fetchFeed } from "./api";
  import { mocks } from "./mocks";
  import { Source } from "./types/Source";
  import { onMount } from "svelte";
  import Feed from "./components/Feed.svelte";

  let feeds: Array<FeedType> = [];

  onMount(async () => {
    const feed = await fetchFeed(Source.REDDIT, "javascript");
    feeds.push(feed);
  });
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
