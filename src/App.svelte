<script lang="ts">
  import Feed from "./components/Feed.svelte";
  import { mocks } from "./mocks";
</script>

<main>
  {#each mocks as feed}
    <Feed {feed} />
  {/each}
</main>
