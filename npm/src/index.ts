/**
 * Neatify - A code formatter library for multiple languages
 *
 * This is the Node.js/npm binding for the neatify Rust library.
 */

export {
  format,
  formatDir,
  formatCode,
  isSupported,
  check,
  getLanguage,
} from './core';
export {
  FormattingStats,
  NeatifyError,
  NeatifyErrorType,
  FormatOptions,
  FormatDirectoryOptions,
  CliOptions,
  SupportedLanguage,
} from './types';

// Default export for CommonJS compatibility
import * as neatify from './core';
export default neatify;
