import { Source } from "./types/Source";
import { Feed } from "./types/Feed";

const BASE_URL = "http://localhost:8000";

export const fetchFeed = async (
  source: Source,
  target: string
): Promise<Feed> => {
  try {
    const response = await fetch(`${BASE_URL}/fetch/${source}/${target}`);
    const data = await response.json();

    return data;
  } catch (error) {
    console.error(error);
  }
};
