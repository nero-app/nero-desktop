import type {
  ExtensionOptions,
  Metadata,
  FilterCategory,
  SeriesPage,
  EpisodesPage,
  Series,
  Video,
  SearchFilter,
  LoadedExtension,
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
): Promise<void> {
  if (
    options.maxCacheSize !== undefined &&
    options.maxCacheSize > MAX_CACHE_SIZE_BYTES
  ) {
    throw new Error(`maxCacheSize cannot exceed ${MAX_CACHE_SIZE_MB} MB`);
  }
  return invoke("plugin:nero-extensions|load_extension", { filePath, options });
}

export async function unloadExtension(extensionId: string): Promise<void> {
  return invoke("plugin:nero-extensions|unload_extension", { extensionId });
}

export async function getLoadedExtensions(): Promise<LoadedExtension[]> {
  return invoke("plugin:nero-extensions|get_loaded_extensions");
}

export async function getFilters(
  extensionId: string,
): Promise<FilterCategory[]> {
  return invoke("plugin:nero-extensions|get_filters", { extensionId });
}

export async function search(
  extensionId: string,
  query: string,
  page?: number,
  filters: SearchFilter[] = [],
): Promise<SeriesPage> {
  return invoke("plugin:nero-extensions|search", {
    extensionId,
    query,
    page,
    filters,
  });
}

export async function getSeriesInfo(
  extensionId: string,
  seriesId: string,
): Promise<Series> {
  return invoke("plugin:nero-extensions|get_series_info", {
    extensionId,
    seriesId,
  });
}

export async function getSeriesEpisodes(
  extensionId: string,
  seriesId: string,
  page?: number,
): Promise<EpisodesPage> {
  return invoke("plugin:nero-extensions|get_series_episodes", {
    extensionId,
    seriesId,
    page,
  });
}

export async function getSeriesVideos(
  extensionId: string,
  seriesId: string,
  episodeId: string,
  episodeNumber: number,
): Promise<Video[]> {
  return invoke("plugin:nero-extensions|get_series_videos", {
    extensionId,
    seriesId,
    episodeId,
    episodeNumber,
  });
}
