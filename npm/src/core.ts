import { existsSync, readFileSync, writeFileSync, statSync } from 'fs';
import { join, extname, resolve, relative } from 'path';
import { glob } from 'glob';
import ignore from 'ignore';
import {
  FormattingStats,
  NeatifyError,
  FormatOptions,
  FormatDirectoryOptions,
  SupportedLanguage,
} from './types';

// Import WebAssembly bindings
let wasmModule: any = null;

/**
 * Initialize the WebAssembly module
 */
async function initWasm(): Promise<void> {
  if (wasmModule) return;

  try {
    // Try to load the WebAssembly module
    wasmModule = await import('../bin/neatify_wasm.js');
  } catch (error) {
    console.error(
      'WebAssembly module not available. Package corrupted or not built. ' +
        'Report this issue at https://github.com/pacmjs/neatify/issues'
    );
    throw NeatifyError.ioError(
      'WebAssembly module not available. Package corrupted or not built. ' +
        'Report this issue at https://github.com/pacmjs/neatify/issues'
    );
  }
}

/**
 * Check if a file is supported by neatify
 */
export function isSupported(filePath: string): boolean {
  const ext = extname(filePath).toLowerCase();
  return ext === '.js' || ext === '.mjs' || ext === '.cjs';
}

/**
 * Get the language for a file based on its extension
 */
export function getLanguage(filePath: string): SupportedLanguage | null {
  const ext = extname(filePath).toLowerCase();
  switch (ext) {
    case '.js':
    case '.mjs':
    case '.cjs':
      return 'javascript';
    default:
      return null;
  }
}

/**
 * Format code content directly using WebAssembly
 */
export async function formatCode(
  content: string,
  language: SupportedLanguage = 'javascript'
): Promise<string> {
  await initWasm();

  try {
    if (language === 'javascript') {
      return wasmModule.format_code(content);
    } else {
      throw NeatifyError.unsupportedFile(`Unsupported language: ${language}`);
    }
  } catch (error) {
    throw NeatifyError.formattingError(
      `Failed to format code: ${error instanceof Error ? error.message : String(error)}`
    );
  }
}

/**
 * Format a single file
 */
export async function format(
  filePath: string,
  options: FormatOptions = {}
): Promise<boolean> {
  const { write = false } = options;

  if (!existsSync(filePath)) {
    throw NeatifyError.ioError(`File does not exist: ${filePath}`);
  }

  if (!isSupported(filePath)) {
    throw NeatifyError.unsupportedFile(filePath);
  }

  try {
    const content = readFileSync(filePath, 'utf8');
    const formatted = await formatCode(content);

    const needsFormatting = content !== formatted;

    if (needsFormatting && write) {
      writeFileSync(filePath, formatted, 'utf8');
    }

    return needsFormatting;
  } catch (error) {
    throw NeatifyError.ioError(
      `Failed to format file: ${filePath}`,
      error as Error
    );
  }
}

/**
 * Load ignore patterns from .neatignore file
 */
function loadIgnorePatterns(dirPath: string, ignorePath?: string): string[] {
  const ignoreFile = ignorePath || join(dirPath, '.neatignore');

  if (!existsSync(ignoreFile)) {
    return [];
  }

  try {
    const content = readFileSync(ignoreFile, 'utf8');
    return content
      .split('\n')
      .map((line: string) => line.trim())
      .filter((line: string) => line && !line.startsWith('#'));
  } catch {
    return [];
  }
}

/**
 * Get all files to format in a directory
 */
function getFilesToFormat(
  dirPath: string,
  options: FormatDirectoryOptions = {}
): string[] {
  const { ignore: customIgnore = [], includeHidden = false } = options;

  // Load ignore patterns
  const ignorePatterns = loadIgnorePatterns(dirPath);
  const allIgnorePatterns = [
    'node_modules/**',
    '.git/**',
    'dist/**',
    'build/**',
    'coverage/**',
    '*.min.js',
    ...ignorePatterns,
    ...customIgnore,
  ];

  if (!includeHidden) {
    allIgnorePatterns.push('.*');
  }

  // Find all supported files
  const patterns = ['**/*.js', '**/*.mjs', '**/*.cjs'];
  const files: string[] = [];

  for (const pattern of patterns) {
    const matches = glob.sync(pattern, {
      cwd: dirPath,
      absolute: true,
      nodir: true,
    });
    files.push(...matches);
  }

  // Apply ignore patterns
  const ig = ignore().add(allIgnorePatterns);

  return files.filter((file) => {
    const relativePath = relative(dirPath, file).replace(/\\/g, '/');
    return !ig.ignores(relativePath);
  });
}

/**
 * Format all supported files in a directory
 */
export async function formatDir(
  dirPath: string,
  options: FormatDirectoryOptions = {}
): Promise<FormattingStats> {
  const { write = false } = options;

  if (!existsSync(dirPath)) {
    throw NeatifyError.ioError(`Directory does not exist: ${dirPath}`);
  }

  if (!statSync(dirPath).isDirectory()) {
    throw NeatifyError.ioError(`Path is not a directory: ${dirPath}`);
  }

  const files = getFilesToFormat(resolve(dirPath), options);

  const stats: FormattingStats = {
    formattedFiles: 0,
    filesNeedingFormatting: 0,
    totalFiles: files.length,
  };

  for (const file of files) {
    try {
      const needsFormatting = await format(file, { write });

      if (needsFormatting) {
        if (write) {
          stats.formattedFiles++;
        } else {
          stats.filesNeedingFormatting++;
        }
      }
    } catch (error) {
      if (error instanceof NeatifyError && error.type === 'UNSUPPORTED_FILE') {
        // Skip unsupported files silently
        continue;
      }
      throw error;
    }
  }

  return stats;
}

/**
 * Check if files in a directory need formatting
 */
export async function check(paths: string[]): Promise<{
  needsFormatting: string[];
  errors: Array<{ path: string; error: string }>;
}> {
  const needsFormatting: string[] = [];
  const errors: Array<{ path: string; error: string }> = [];

  for (const path of paths) {
    try {
      if (statSync(path).isDirectory()) {
        const stats = await formatDir(path, { write: false });
        if (stats.filesNeedingFormatting > 0) {
          // Get the specific files that need formatting
          const files = getFilesToFormat(path);
          for (const file of files) {
            try {
              const needs = await format(file, { write: false });
              if (needs) {
                needsFormatting.push(file);
              }
            } catch (error) {
              errors.push({
                path: file,
                error: error instanceof Error ? error.message : String(error),
              });
            }
          }
        }
      } else {
        const needs = await format(path, { write: false });
        if (needs) {
          needsFormatting.push(path);
        }
      }
    } catch (error) {
      errors.push({
        path,
        error: error instanceof Error ? error.message : String(error),
      });
    }
  }

  return { needsFormatting, errors };
}
