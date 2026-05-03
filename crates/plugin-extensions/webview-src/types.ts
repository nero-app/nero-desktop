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

export interface ExtensionOptions {
  cacheDir: string;
  maxCacheSize?: number;
}

export interface ExtensionInfo {
  filePath: string;
  metadata: Metadata;
  options: ExtensionOptions;
}

export interface PluginStatus {
  extension: ExtensionInfo | null;
  torrentEnabled: boolean;
  torrentOutputFolder: string | null;
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
