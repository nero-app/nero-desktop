import type { Episode, Series } from "@nero/plugin-extensions";

export const sampleSeries: Series = {
  id: "1",
  title: "Takt Op. Destiny",
  posterUrl:
    "https://m.media-amazon.com/images/M/MV5BMDQ3MTk5NjUtMzhjMC00NjFhLWEyZWMtNzU5M2JiZDJkYTcyXkEyXkFqcGc@._V1_.jpg",
  synopsis:
    "Lorem ipsum dolor sit amet consectetur adipisicing elit. Corporis ducimus commodi, porro saepe illum ullam quis fugiat molestiae voluptatibus quas velit aperiam, neque molestias consequuntur eum dignissimos, numquam doloremque odit.",
  type: "Series",
};

export const sampleEpisode: Episode = {
  id: "1",
  number: 1,
  title: "Lorem ipsum dolor sit amet",
  thumbnailUrl:
    "https://i0.wp.com/www.crowsworldofanime.com/wp-content/uploads/2021/10/takt_op.Destiny-Episode_01_Figure-06-scaled.jpg?ssl=1",
  description:
    "Lorem ipsum dolor sit amet consectetur adipisicing elit. Corporis ducimus commodi, porro saepe illum ullam quis fugiat molestiae voluptatibus quas velit aperiam, neque molestias consequuntur eum dignissimos, numquam doloremque odit.",
};
