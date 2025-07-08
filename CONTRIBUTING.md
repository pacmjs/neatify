# Contributing to Neatify

Thank you for considering contributing to Neatify! This document provides guidelines and instructions for contributing to this project.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. Please be respectful and considerate of others.

## How Can I Contribute?

### Reporting Bugs

Bugs are tracked as GitHub issues. Create an issue and provide the following information:

- Use a clear and descriptive title
- Describe the exact steps to reproduce the bug
- Provide specific examples to demonstrate the steps
- Describe the behavior you observed and what you expected to see
- Include screenshots if possible
- Include details about your environment

### Suggesting Enhancements

Enhancement suggestions are also tracked as GitHub issues. When creating an enhancement suggestion, include:

- A clear and descriptive title
- A detailed description of the proposed enhancement
- Explain why this enhancement would be useful to most users
- List some other applications where this enhancement exists, if applicable

### Pull Requests

- Fill in the required template
- Follow the Rust style guide
- Include tests for new features or bug fixes
- Update documentation as needed
- End all files with a newline

## Development Setup

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/neatify.git`
3. Add the original repository as upstream: `git remote add upstream https://github.com/pacmjs/neatify.git`
4. Create a new branch for your changes: `git checkout -b feature/your-feature-name`

## Testing

Run tests with:

```bash
cargo test
```

## Style Guide

We follow the standard Rust style guide. You can use `rustfmt` to format your code:

```bash
rustfmt src/*.rs
```

## Adding a New Formatter

To add support for a new language:

1. Create a new module in `src/formatters/`
2. Implement the `Tokenizer` trait for your language
3. Implement the `Formatter` trait for your language
4. Add your formatter to the list in `src/formatters/mod.rs`
5. Add tests for your formatter

## License

By contributing, you agree that your contributions will be licensed under the project's BSD 3-Clause License.