/**
 * Supported languages for formatting
 */
export type SupportedLanguage = 'javascript';

/**
 * Statistics about formatting operations
 */
export interface FormattingStats {
  /** Number of files that were formatted */
  formattedFiles: number;
  /** Number of files that needed formatting but were not modified (dry run) */
  filesNeedingFormatting: number;
  /** Total number of files processed */
  totalFiles: number;
}

/**
 * Error types for neatify operations
 */
export enum NeatifyErrorType {
  UNSUPPORTED_FILE = 'UNSUPPORTED_FILE',
  IO_ERROR = 'IO_ERROR',
  FORMATTING_ERROR = 'FORMATTING_ERROR',
}

/**
 * Custom error class for neatify operations
 */
export class NeatifyError extends Error {
  public readonly type: NeatifyErrorType;
  public readonly originalError?: Error;

  constructor(type: NeatifyErrorType, message: string, originalError?: Error) {
    super(message);
    this.name = 'NeatifyError';
    this.type = type;
    this.originalError = originalError;
  }

  static unsupportedFile(path: string): NeatifyError {
    return new NeatifyError(
      NeatifyErrorType.UNSUPPORTED_FILE,
      `Unsupported file: ${path}`
    );
  }

  static ioError(message: string, originalError?: Error): NeatifyError {
    return new NeatifyError(
      NeatifyErrorType.IO_ERROR,
      `IO error: ${message}`,
      originalError
    );
  }

  static formattingError(message: string, originalError?: Error): NeatifyError {
    return new NeatifyError(
      NeatifyErrorType.FORMATTING_ERROR,
      `Formatting error: ${message}`,
      originalError
    );
  }
}

/**
 * Options for formatting operations
 */
export interface FormatOptions {
  /** Whether to write the formatted output back to the file */
  write?: boolean;
}

/**
 * Options for directory formatting operations
 */
export interface FormatDirectoryOptions extends FormatOptions {
  /** Patterns to exclude from formatting */
  ignore?: string[];
  /** Whether to include hidden files */
  includeHidden?: boolean;
}

/**
 * CLI options interface
 */
export interface CliOptions {
  /** Check if files need formatting without modifying them */
  check?: boolean;
  /** Write formatted output back to files */
  write?: boolean;
  /** List files that need formatting */
  listDifferent?: boolean;
  /** Ignore .neatignore file */
  noIgnore?: boolean;
  /** Specify custom ignore file path */
  ignorePath?: string;
  /** Include hidden files */
  includeHidden?: boolean;
  /** Verbose output */
  verbose?: boolean;
}
