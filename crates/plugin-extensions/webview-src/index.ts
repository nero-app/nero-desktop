import { invoke } from "@tauri-apps/api/core";

export interface ExtensionOptions {
  cacheDir: string;
  maxCacheSize?: number;
}

export interface Metadata {
  name?: string;
  description?: string;
  version?: string;
  revision?: string;
  authors?: string;
  licenses?: string;
  source?: string;
  homepage?: string;
  range?: { start: number; end: number };
  producers?: [string, Record<string, string>][];
  dependencies?: string[];
}

export interface Series {
  id: string;
  title: string;
  posterUrl?: string;
  synopsis?: string;
  type?: string;
}

export interface Episode {
  id: string;
  number: number;
  title?: string;
  thumbnailUrl?: string;
  description?: string;
}

export interface Page<T> {
  items: T[];
  hasNextPage: boolean;
}

export type SeriesPage = Page<Series>;
export type EpisodesPage = Page<Episode>;

export interface FilterCategory {
  id: string;
  displayName: string;
  filters: Filter[];
}

export interface Filter {
  id: string;
  displayName: string;
}

export interface SearchFilter {
  id: string;
  values: string[];
}

export interface Video {
  url: string;
  server: string;
  resolution: [number, number];
}

export async function enableTorrentSupport(
  outputFolder: string,
): Promise<void> {
  return await invoke("plugin:nero-extensions|enable_torrent_support", {
    outputFolder,
  });
}

export async function disableTorrentSupport(): Promise<void> {
  return await invoke("plugin:nero-extensions|disable_torrent_support");
}

export const MAX_CACHE_SIZE_MB = 250;
export const MAX_CACHE_SIZE_BYTES = MAX_CACHE_SIZE_MB * 1024 * 1024;

export class Extension {
  readonly filePath: string;
  readonly metadata: Metadata;
  readonly options: ExtensionOptions;

  private constructor(
    filePath: string,
    metadata: Metadata,
    options: ExtensionOptions,
  ) {
    this.filePath = filePath;
    this.metadata = metadata;
    this.options = options;
  }

  static async getMetadata(filePath: string): Promise<Metadata> {
    return await invoke("plugin:nero-extensions|get_extension_metadata", {
      filePath,
    });
  }

  static async load(
    filePath: string,
    options: ExtensionOptions,
  ): Promise<Extension> {
    if (
      options.maxCacheSize !== undefined &&
      options.maxCacheSize > MAX_CACHE_SIZE_BYTES
    ) {
      throw new Error(`maxCacheSize cannot exceed ${MAX_CACHE_SIZE_MB} MB`);
    }
    const metadata = await Extension.getMetadata(filePath);
    await invoke("plugin:nero-extensions|load_extension", {
      filePath,
      options,
    });
    return new Extension(filePath, metadata, options);
  }

  async getFilters(): Promise<FilterCategory[]> {
    return await invoke("plugin:nero-extensions|get_filters");
  }

  async search(
    query: string,
    page?: number,
    filters: SearchFilter[] = [],
  ): Promise<SeriesPage> {
    return await invoke("plugin:nero-extensions|search", {
      query,
      page,
      filters,
    });
  }

  async getSeriesInfo(seriesId: string): Promise<Series> {
    return await invoke("plugin:nero-extensions|get_series_info", {
      seriesId,
    });
  }

  async getSeriesEpisodes(
    seriesId: string,
    page?: number,
  ): Promise<EpisodesPage> {
    return await invoke("plugin:nero-extensions|get_series_episodes", {
      seriesId,
      page,
    });
  }

  async getSeriesVideos(
    seriesId: string,
    episodeId: string,
    episodeNumber: number,
  ): Promise<Video[]> {
    return await invoke("plugin:nero-extensions|get_series_videos", {
      seriesId,
      episodeId,
      episodeNumber,
    });
  }
}
