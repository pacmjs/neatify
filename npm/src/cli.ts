#!/usr/bin/env node

import { Command } from 'commander';
import chalk from 'chalk';
import { existsSync, statSync } from 'fs';
import { resolve } from 'path';
import { format, formatDir, check } from './core';
import { CliOptions } from './types';

const program = new Command();

program
  .name('neatify')
  .description('A code formatter for multiple languages')
  .version('0.1.1')
  .argument('[files...]', 'Files or directories to format')
  .option(
    '-c, --check',
    'Check if files need formatting without modifying them'
  )
  .option('-w, --write', 'Write formatted output back to files')
  .option('-l, --list-different', 'List files that need formatting')
  .option('--no-ignore', 'Ignore .neatignore file')
  .option('--ignore-path <path>', 'Specify custom ignore file path')
  .option('--include-hidden', 'Include hidden files')
  .option('-v, --verbose', 'Verbose output')
  .action(async (files: string[], options: CliOptions) => {
    try {
      await runCli(files, options);
    } catch (error) {
      console.error(
        chalk.red('Error:'),
        error instanceof Error ? error.message : String(error)
      );
      process.exit(1);
    }
  });

/**
 * Main CLI function
 */
async function runCli(files: string[], options: CliOptions): Promise<void> {
  const {
    check: checkMode = false,
    write = false,
    listDifferent = false,
    verbose = false,
  } = options;

  // If no files specified, format current directory
  if (files.length === 0) {
    files = ['.'];
  }

  // Validate that files exist
  for (const file of files) {
    if (!existsSync(file)) {
      console.error(
        chalk.red(`Error: File or directory does not exist: ${file}`)
      );
      process.exit(1);
    }
  }

  if (checkMode || listDifferent) {
    await runCheckMode(files, options);
  } else if (write) {
    await runWriteMode(files, options);
  } else {
    console.error(
      chalk.yellow('Warning: No action specified. Use --check or --write.')
    );
    console.log('Use --help for more information.');
    process.exit(1);
  }
}

/**
 * Run in check mode
 */
async function runCheckMode(
  files: string[],
  options: CliOptions
): Promise<void> {
  const { verbose = false, listDifferent = false } = options;

  if (verbose) {
    console.log(chalk.blue('Checking files for formatting...'));
  }

  const { needsFormatting, errors } = await check(files);

  // Report errors
  if (errors.length > 0) {
    console.error(chalk.red(`\nErrors encountered:`));
    for (const { path, error } of errors) {
      console.error(chalk.red(`  ${path}: ${error}`));
    }
  }

  // Report files that need formatting
  if (needsFormatting.length > 0) {
    if (listDifferent) {
      needsFormatting.forEach((file) => console.log(file));
    } else {
      console.log(chalk.yellow(`\nFiles that need formatting:`));
      needsFormatting.forEach((file) => {
        console.log(chalk.yellow(`  ${file}`));
      });
      console.log(
        chalk.yellow(`\nTotal: ${needsFormatting.length} files need formatting`)
      );
    }
    process.exit(1);
  } else {
    if (verbose && !listDifferent) {
      console.log(chalk.green('All files are properly formatted!'));
    }
    process.exit(0);
  }
}

/**
 * Run in write mode
 */
async function runWriteMode(
  files: string[],
  options: CliOptions
): Promise<void> {
  const { verbose = false } = options;

  if (verbose) {
    console.log(chalk.blue('Formatting files...'));
  }

  let totalFormatted = 0;
  let totalProcessed = 0;
  const errors: Array<{ path: string; error: string }> = [];

  for (const file of files) {
    const filePath = resolve(file);

    try {
      if (statSync(filePath).isDirectory()) {
        const stats = await formatDir(filePath, {
          write: true,
          ignore: options.noIgnore ? [] : undefined,
          includeHidden: options.includeHidden,
        });

        totalFormatted += stats.formattedFiles;
        totalProcessed += stats.totalFiles;

        if (verbose) {
          console.log(chalk.green(`Formatted directory: ${filePath}`));
          console.log(`  Files processed: ${stats.totalFiles}`);
          console.log(`  Files formatted: ${stats.formattedFiles}`);
        }
      } else {
        const needsFormatting = await format(filePath, { write: true });
        totalProcessed++;

        if (needsFormatting) {
          totalFormatted++;
          if (verbose) {
            console.log(chalk.green(`Formatted: ${filePath}`));
          }
        } else if (verbose) {
          console.log(chalk.gray(`Already formatted: ${filePath}`));
        }
      }
    } catch (error) {
      errors.push({
        path: filePath,
        error: error instanceof Error ? error.message : String(error),
      });
    }
  }

  // Report results
  if (errors.length > 0) {
    console.error(chalk.red(`\nErrors encountered:`));
    for (const { path, error } of errors) {
      console.error(chalk.red(`  ${path}: ${error}`));
    }
  }

  if (verbose || errors.length > 0) {
    console.log(chalk.blue(`\nSummary:`));
    console.log(`  Files processed: ${totalProcessed}`);
    console.log(`  Files formatted: ${totalFormatted}`);
    if (errors.length > 0) {
      console.log(chalk.red(`  Errors: ${errors.length}`));
    }
  }

  if (errors.length > 0) {
    process.exit(1);
  }
}

// Handle unhandled promise rejections
process.on('unhandledRejection', (reason, promise) => {
  console.error(chalk.red('Unhandled promise rejection:'), reason);
  process.exit(1);
});

// Handle uncaught exceptions
process.on('uncaughtException', (error) => {
  console.error(chalk.red('Uncaught exception:'), error);
  process.exit(1);
});

// Parse command line arguments
program.parse();
