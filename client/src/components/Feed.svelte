<script lang="ts">
  import { Feed } from "../types/Feed";
  import Post from "./Post.svelte";
  export let feed: Feed;
</script>

<section>
  <span>{feed.title}</span>
  {#if feed.posts.length > 0}
    {#each feed.posts as post}
      <Post {post} />
    {/each}
  {/if}
</section>
