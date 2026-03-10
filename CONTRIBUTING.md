# Contributing to HEDG Protocol

Thank you for your interest in contributing to HEDG Protocol. This document provides guidelines and instructions for contributing.

## Development Setup

1. Install prerequisites:
   - Rust 1.75+ and Cargo
   - Solana CLI 1.18+
   - Anchor CLI 0.30+
   - Node.js 18+ and npm

2. Clone the repository:
   ```bash
   git clone https://github.com/hedg-lol/hedg.git
   cd fyrst
   ```

3. Build the contracts:
   ```bash
   anchor build
   ```

4. Run the test suite:
   ```bash
   anchor test
   ```

5. Build the SDK:
   ```bash
   cd sdk && npm install && npm run build
   ```

## Branching Strategy

- `main` - stable releases
- `develop` - active development
- `feat/*` - feature branches
- `fix/*` - bug fix branches

## Pull Request Process

1. Fork the repository and create your branch from `develop`.
2. Write clear, concise commit messages following conventional commits format.
3. Add tests for any new functionality.
4. Ensure all existing tests pass.
5. Update documentation if your changes affect the public API.
6. Submit your pull request with a clear description of the changes.

## Commit Message Format

We follow the conventional commits specification:

```
type(scope): description

[optional body]
```

Types: `feat`, `fix`, `docs`, `chore`, `test`, `refactor`, `style`, `ci`, `perf`

Examples:
- `feat(escrow): add early release penalty mechanism`
- `fix(bonding-curve): correct overflow in price calculation`
- `docs: update SDK usage examples`

## Code Style

- Rust: follow `rustfmt` defaults
- TypeScript: follow the project ESLint configuration
- Keep functions focused and well-documented
- Use meaningful variable names

## Reporting Issues

When reporting issues, please include:
- A clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Solana version, Anchor version)

## Security

If you discover a security vulnerability, please report it privately via email rather than creating a public issue. See the security section in the README for contact information.

## License

By contributing to HEDG Protocol, you agree that your contributions will be licensed under the MIT License.
