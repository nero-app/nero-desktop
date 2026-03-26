import EmptyFeed from "../components/ui/EmptyFeed";
import { MediaLayout } from "../layouts/MediaLayout";

export default function HomePage() {
  return (
    <MediaLayout>
      <MediaLayout.Media>
        <div class="size-full bg-gray-200 object-cover" />
      </MediaLayout.Media>

      <MediaLayout.Content as="section" class="flex flex-col justify-center">
        <EmptyFeed />
      </MediaLayout.Content>
    </MediaLayout>
  );
}
