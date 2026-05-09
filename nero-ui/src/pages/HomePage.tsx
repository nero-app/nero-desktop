import EmptyFeed from "../components/ui/EmptyFeed";
import { MediaLayout } from "../layouts/MediaLayout";
import { ImageOffIcon } from "lucide-solid";

export default function HomePage() {
  return (
    <MediaLayout>
      <MediaLayout.Media>
        <div class="flex size-full items-center justify-center bg-neutral-200">
          <ImageOffIcon class="text-neutral-300" size={72} />
        </div>
      </MediaLayout.Media>

      <MediaLayout.Content as="section" class="flex flex-col justify-center">
        <EmptyFeed />
      </MediaLayout.Content>
    </MediaLayout>
  );
}
