import shockedCat from "../../assets/images/shocked_cat.svg";
import { t } from "../../lib/i18n";
import { Button } from "./Button";
import { Typography } from "./Typography";
import { useNavigate } from "@solidjs/router";
import { SearchIcon } from "lucide-solid";

export default function EmptyFeed() {
  const navigate = useNavigate();

  return (
    <article class="flex flex-col items-center gap-2 text-center">
      <img class="w-64 lg:w-80 xl:w-96" src={shockedCat} alt="Shocked cat" />

      <Typography>
        {t("common.error_title")}
        <br />
        {t("common.empty")}
      </Typography>

      <Button class="w-full" onClick={() => navigate("/search")}>
        <SearchIcon />
        <Typography as="span">{t("media.search_placeholder")}</Typography>
      </Button>
    </article>
  );
}
