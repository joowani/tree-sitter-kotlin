# tree-sitter-kotlin

Kotlin grammar for [Tree-sitter](https://github.com/tree-sitter/tree-sitter).

### Playground

```shell
# Install tree-sitter CLI
cargo install tree-sitter-cli --locked

# Clone this repository and navigate into it
git clone https://github.com/joowani/tree-sitter-kotlin && cd tree-sitter-kotlin

# Compile the parser
tree-sitter build --wasm

# Start the playground at http://127.0.0.1:8000
tree-sitter playground
```

### References

- [Tree-sitter Documentation](https://tree-sitter.github.io/tree-sitter/)
- [Kotlin Grammar](https://kotlinlang.org/docs/reference/grammar.html)

### Credits

The grammar is originally based
on [fwcd/tree-sitter-kotlin](https://github.com/fwcd/tree-sitter-kotlin).
