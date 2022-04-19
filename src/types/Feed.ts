import { Post } from "./Post";

export interface Feed {
  title: string;
  posts: Array<Post>;
}
