// T中至少存在Keys中的一个
export type RequireAtLeastOne<T, Keys extends keyof T = keyof T> = Pick<
  T,
  Exclude<keyof T, Keys>
> &
  {
    [K in Keys]-?: Required<Pick<T, K>> & Partial<Pick<T, Exclude<Keys, K>>>;
  }[Keys];

type BasicFileIcon = DefaultIcon & {
  /**
   * Define the file extensions that should use this icon.
   * E.g. `['js']`
   */
  fileExtensions?: string[];

  /**
   * Define if there are some static file names that should apply this icon.
   * E.g. `['sample.js']`
   */
  fileNames?: string[];

  /**
   * Define patterns for file names. Patterns are used to generate common file names and file extensions based on a key.
   */
  patterns?: Patterns;

  /**
   * Define if the icon should be disabled.
   */
  disabled?: boolean;
};



type RequireAtLeastOneFileIcon<T> = T extends BasicFileIcon
  ? RequireAtLeastOne<T, 'fileExtensions' | 'fileNames' | 'patterns'>
  : never;

/**
 * Type for a `FileIcon`. In addition to the `name` property, either a `fileExtensions`, `fileNames`, or `patterns` property is required.
 */
export type FileIcon = RequireAtLeastOneFileIcon<BasicFileIcon>;

export enum FileNamePattern {
  /** Adds the following extensions to the file name: `js`, `mjs`, `cjs`, `ts`, `mts`, `cts`. */
  Ecmascript = 'ecmascript',

  /** Adds the following extensions to the file name: `json`, `jsonc`, `json5`, `yaml`, `yml`, `toml`. */
  Configuration = 'configuration',

  /** Adds the following extensions to the file name: `js`, `mjs`, `cjs`, `ts`, `mts`, `cts`, `json`, `jsonc`, `json5`, `yaml`, `yml`, `toml`. */
  NodeEcosystem = 'nodeEcosystem',

  /** It adjusts the name with the following patterns: `.fileNamerc`, `.config/fileNamerc`, `fileName.config` and combines that with the pattern `NodeEcosystem` */
  Cosmiconfig = 'cosmiconfig',
}

export type DefaultIcon = {
  /**
   * Name of the icon, e.g. `src`
   */
  name: string;

  /**
   * Define if there is a light icon available.
   */
  light?: boolean;

  /**
   * Define if there is a high contrast icon available.
   */
  highContrast?: boolean;
};

export type FileIcons = {
  /**
   * Define the default icon for folders.
   */
  defaultIcon: DefaultIcon;

  /**
   * Defines all folder icons.
   */
  icons: FileIcon[];
};


export type Patterns = Record<string, FileNamePattern>;
export type FileIconWithPatterns = (FileIcon & { patterns?: Patterns })[];
