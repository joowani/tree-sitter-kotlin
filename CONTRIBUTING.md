# Contributing to tree-sitter-kotlin

## Development Setup

Install Node.js (version 20, 22, or 24 or later), then run:

```bash
npm ci
```

## Key Files

| File                     | Description                                                               |
|--------------------------|---------------------------------------------------------------------------|
| `grammar.js`             | The grammar definition — edit this to change parsing rules                |
| `src/scanner.c`          | External scanner for ASI, strings, comments, and context-sensitive tokens |
| `src/parser.c`           | Generated parser (~36 MB) — do **not** edit manually                      |
| `test/corpus/*.txt`      | Test cases in tree-sitter corpus format                                   |
| `queries/highlights.scm` | Syntax highlighting queries                                               |
| `queries/tags.scm`       | Code navigation tags                                                      |

## Making Grammar Changes

1. Edit `grammar.js` (and/or `src/scanner.c` for lexer-level changes)
2. Regenerate the parser:
   ```bash
   npm run generate
   ```
3. Run the tests:
   ```bash
   npm test
   ```
4. Commit **all** changed files in `src/` along with your grammar changes

## Parser Verification CI Check

The CI build regenerates the parser and verifies that the output matches what is committed in `src/`. If the diff is
non-empty, the build fails.

`tree-sitter generate` produces the same output on macOS and Linux, so you can regenerate the parser locally. On Linux,
the Tree-sitter CLI binary that npm installs needs glibc 2.39 or later, as in Ubuntu 24.04 or Debian 13. On an older
system, generate it in Docker instead:

```bash
docker run --rm -v "$(pwd)":/work -w /work node:24-trixie bash -c "npm ci && npm run generate"
```

## Parser Size Limit

CI enforces a maximum parser size of **60 MiB** for `src/parser.c`. If your grammar changes cause the parser to exceed
this, you may need to simplify or restructure the rules.

## Test Format

Tests use tree-sitter's corpus format:

```
================================================================================
Test Name
================================================================================

kotlin source code here

--------------------------------------------------------------------------------

(source_file
  (expected_parse_tree))
```

Place test files in `test/corpus/`. Each file can contain multiple test cases.

## Useful Commands

```bash
# Parse a specific file
npx tree-sitter parse path/to/file.kt

# Parse with XML output (shows all node details)
npx tree-sitter parse path/to/file.kt --xml

# Run tags extraction
npx tree-sitter tags path/to/file.kt

# Test the Rust and Node.js bindings. After a grammar change, rebuild the Node.js addon first: `npm rebuild` keeps an
# existing build.
cargo test
npx node-gyp rebuild && npm run test:node

# Check compiled parser size
cc -O2 -I src -std=c11 -c src/parser.c -o /tmp/parser.o && wc -c /tmp/parser.o
```
