<div align="center">

<img src="assets/nametag.png" alt="Neatify Nametag" width="450" height="300">

<div style="display: flex; justify-content: center; gap: 8px; flex-wrap: wrap;">

[![npm version](https://badge.fury.io/js/neatify.svg)](https://badge.fury.io/js/neatify)
[![License](https://img.shields.io/npm/l/neatify.svg)](LICENSE)

A powerful, extensible code formatter library for multiple languages with a clean API interface. This is the Node.js/npm binding for the neatify Rust library.
</div>

## Features

- **Multi-language Support**: Format code in multiple languages (currently supports JavaScript)
- **TypeScript Ready**: Full TypeScript support with complete type definitions
- **CLI Tool**: Command-line interface similar to Prettier
- **Ignore File Support**: `.neatignore` file support for excluding files
- **API and CLI**: Use programmatically or via command line
- **Error Handling**: Robust error handling with detailed error messages
- **Formatting Statistics**: Get detailed statistics about formatting operations

## Installation

```bash
# Install globally for CLI usage
npm install -g neatify

# Install locally for API usage
npm install neatify

# Or with yarn
yarn add neatify
```

## CLI Usage

### Basic Usage

```bash
# Format files in current directory
neatify --write .

# Check if files need formatting
neatify --check .

# List files that need formatting
neatify --list-different src/

# Format specific files
neatify --write file1.js file2.js

# Verbose output
neatify --write --verbose .
```

### CLI Options

- `--check, -c`: Check if files need formatting without modifying them
- `--write, -w`: Write formatted output back to files
- `--list-different, -l`: List files that need formatting
- `--no-ignore`: Ignore .neatignore file
- `--ignore-path <path>`: Specify custom ignore file path
- `--include-hidden`: Include hidden files
- `--verbose, -v`: Verbose output

### .neatignore File

Create a `.neatignore` file in your project root to exclude files from formatting:

```
# Ignore node_modules (ignored by default)
node_modules/

# Ignore build outputs
dist/
build/
coverage/

# Ignore specific files
*.min.js
generated/

# Ignore hidden files (use --include-hidden to override)
.*
```

## API Usage

### TypeScript/JavaScript

```typescript
import neatify, { format, formatDir, formatCode, FormattingStats } from 'neatify';

// Format a single file
async function formatFile() {
  try {
    const needsFormatting = await format('src/index.js', { write: true });
    console.log(`File ${needsFormatting ? 'was' : 'was not'} formatted`);
  } catch (error) {
    console.error('Formatting failed:', error.message);
  }
}

// Format a directory
async function formatDirectory() {
  try {
    const stats = await formatDir('src/', { 
      write: true,
      includeHidden: false 
    });
    
    console.log('Formatting complete:');
    console.log(`  Files processed: ${stats.totalFiles}`);
    console.log(`  Files formatted: ${stats.formattedFiles}`);
  } catch (error) {
    console.error('Directory formatting failed:', error.message);
  }
}

// Format code directly
function formatCodeExample() {
  const unformatted = 'function test(){return"hello";}';
  const formatted = formatCode(unformatted);
  console.log(formatted);
  // Output: function test() {\n  return "hello";\n}\n
}

// Check files without formatting
import { check } from 'neatify';

async function checkFiles() {
  const { needsFormatting, errors } = await check(['src/']);
  
  if (needsFormatting.length > 0) {
    console.log('Files needing formatting:', needsFormatting);
  }
  
  if (errors.length > 0) {
    console.log('Errors:', errors);
  }
}
```

### CommonJS

```javascript
const neatify = require('neatify');

// Use default export
neatify.format('file.js', { write: true })
  .then(needsFormatting => {
    console.log(`File ${needsFormatting ? 'was' : 'was not'} formatted`);
  })
  .catch(error => {
    console.error('Error:', error.message);
  });
```

## API Reference

### Functions

#### `format(filePath: string, options?: FormatOptions): Promise<boolean>`

Format a single file.

- **Parameters:**
  - `filePath`: Path to the file to format
  - `options`: Optional formatting options
- **Returns:** Promise that resolves to `true` if file needed formatting, `false` otherwise
- **Throws:** `NeatifyError` if formatting fails

#### `formatDir(dirPath: string, options?: FormatDirectoryOptions): Promise<FormattingStats>`

Format all supported files in a directory.

- **Parameters:**
  - `dirPath`: Path to directory to format
  - `options`: Optional formatting options
- **Returns:** Promise that resolves to formatting statistics
- **Throws:** `NeatifyError` if formatting fails

#### `formatCode(content: string, language?: string): string`

Format code content directly.

- **Parameters:**
  - `content`: Code content to format
  - `language`: Language type (default: 'javascript')
- **Returns:** Formatted code string

#### `isSupported(filePath: string): boolean`

Check if a file is supported for formatting.

- **Parameters:**
  - `filePath`: Path to check
- **Returns:** `true` if file is supported, `false` otherwise

#### `check(paths: string[]): Promise<CheckResult>`

Check if files need formatting without modifying them.

- **Parameters:**
  - `paths`: Array of file/directory paths to check
- **Returns:** Promise with check results

### Types

#### `FormatOptions`
```typescript
interface FormatOptions {
  write?: boolean; // Whether to write formatted output back to file
}
```

#### `FormatDirectoryOptions`
```typescript
interface FormatDirectoryOptions extends FormatOptions {
  ignore?: string[];      // Additional patterns to ignore
  includeHidden?: boolean; // Whether to include hidden files
}
```

#### `FormattingStats`
```typescript
interface FormattingStats {
  formattedFiles: number;        // Number of files that were formatted
  filesNeedingFormatting: number; // Files needing formatting (dry run)
  totalFiles: number;            // Total number of files processed
}
```

#### `NeatifyError`
```typescript
class NeatifyError extends Error {
  type: 'UNSUPPORTED_FILE' | 'IO_ERROR' | 'FORMATTING_ERROR';
  originalError?: Error;
}
```

## Supported Languages

Currently supported:
- **JavaScript** (`.js`, `.mjs`, `.cjs`)

Planned support:
- TypeScript
- Rust
- Python
- HTML/CSS
- JSON
- YAML

## Integration with Build Tools

### npm scripts

```json
{
  "scripts": {
    "format": "neatify --write .",
    "format:check": "neatify --check .",
    "format:list": "neatify --list-different ."
  }
}
```

### Pre-commit hooks

With [husky](https://github.com/typicode/husky):

```json
{
  "husky": {
    "hooks": {
      "pre-commit": "neatify --check ."
    }
  }
}
```

### CI/CD

```yaml
# .github/workflows/format.yml
name: Check Formatting
on: [push, pull_request]
jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
      - run: npm ci
      - run: npx neatify --check .
```

## Contributing

This package is part of the larger neatify project. See the main [CONTRIBUTING.md](https://github.com/pacmjs/neatify/blob/main/CONTRIBUTING.md) for details.

## License

BSD 3-Clause License - see [LICENSE](https://github.com/pacmjs/neatify/blob/main/LICENSE.md) for details.
