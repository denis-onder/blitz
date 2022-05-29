<script lang="ts">
  import { Post } from "../types/Post";

  export let post: Post;

  const navigateToPost = (link) => {
    console.log("link", link);
  };
</script>

<article>
  {#if post.thumbnail}
    <img src={post.thumbnail} alt={post.title} />
  {/if}
  {#if post.author}
    <span>{post.author}</span>
  {/if}
  <h2 on:click={() => navigateToPost(post.link)}>{post.title}</h2>
  {#if post.subtitle}
    <h4>{post.subtitle}</h4>
  {/if}
</article>

<style>
  img {
    max-width: 40px;
  }
</style>
