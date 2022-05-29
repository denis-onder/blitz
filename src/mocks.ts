import { Post } from "./types/Post";
import { Feed } from "./types/Feed";

const generateRandomNumber = (): number => Math.ceil(Math.random() * 999);

const generatePost = (): Post => {
  return {
    author: "Ernest Hemingway",
    link: "https://google.com/",
    title: `${generateRandomNumber()}: A post about something`,
    subtitle: "Lorem Ipsum Dolor Sit Amet.",
    thumbnail:
      "https://thealmanian.com/wp-content/uploads/2019/01/product_image_thumbnail_placeholder.png",
  };
};

const generateFeed = (): Feed => {
  return {
    title: `${generateRandomNumber()} - Feed`,
    posts: Array(10).fill(generatePost()),
  };
};

export const mocks = Array(3).fill(generateFeed());
