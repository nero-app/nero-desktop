import type {
  ExtensionOptions,
  ExtensionInfo,
  Metadata,
  FilterCategory,
  SeriesPage,
  EpisodesPage,
  Series,
  Video,
  SearchFilter,
} from "./types";
import { invoke } from "@tauri-apps/api/core";

export const MAX_CACHE_SIZE_MB = 250;
export const MAX_CACHE_SIZE_BYTES = MAX_CACHE_SIZE_MB * 1024 * 1024;

export async function getExtensionMetadata(
  filePath: string,
): Promise<Metadata> {
  return invoke("plugin:nero-extensions|get_extension_metadata", { filePath });
}

export async function loadExtension(
  filePath: string,
  options: ExtensionOptions,
): Promise<ExtensionInfo> {
  if (
    options.maxCacheSize !== undefined &&
    options.maxCacheSize > MAX_CACHE_SIZE_BYTES
  ) {
    throw new Error(`maxCacheSize cannot exceed ${MAX_CACHE_SIZE_MB} MB`);
  }
  return invoke("plugin:nero-extensions|load_extension", { filePath, options });
}

export async function getFilters(): Promise<FilterCategory[]> {
  return invoke("plugin:nero-extensions|get_filters");
}

export async function search(
  query: string,
  page?: number,
  filters: SearchFilter[] = [],
): Promise<SeriesPage> {
  return invoke("plugin:nero-extensions|search", { query, page, filters });
}

export async function getSeriesInfo(seriesId: string): Promise<Series> {
  return invoke("plugin:nero-extensions|get_series_info", { seriesId });
}

export async function getSeriesEpisodes(
  seriesId: string,
  page?: number,
): Promise<EpisodesPage> {
  return invoke("plugin:nero-extensions|get_series_episodes", {
    seriesId,
    page,
  });
}

export async function getSeriesVideos(
  seriesId: string,
  episodeId: string,
  episodeNumber: number,
): Promise<Video[]> {
  return invoke("plugin:nero-extensions|get_series_videos", {
    seriesId,
    episodeId,
    episodeNumber,
  });
}
