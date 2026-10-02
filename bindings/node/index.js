import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

// Resolve files relative to this module's URL, since a file system path breaks `import()` on Windows or when it
// contains characters such as `#`.
const root = new URL("../../", import.meta.url);

const binding = typeof process.versions.bun === "string"
  // Support `bun build --compile` by being statically analyzable enough to find the .node file at build-time.
  // `import()` returns a frozen module namespace, so take the native exports from its default export.
  ? (await import(`../../prebuilds/${process.platform}-${process.arch}/tree-sitter-kotlin.node`)).default
  : (await import("node-gyp-build")).default(fileURLToPath(root));

try {
  const nodeTypes = await import("../../src/node-types.json", { with: { type: "json" } });
  binding.nodeTypeInfo = nodeTypes.default;
} catch { }

const queries = [
  ["HIGHLIGHTS_QUERY", new URL("queries/highlights.scm", root)],
  ["INJECTIONS_QUERY", new URL("queries/injections.scm", root)],
  ["LOCALS_QUERY", new URL("queries/locals.scm", root)],
  ["TAGS_QUERY", new URL("queries/tags.scm", root)],
];

for (const [prop, path] of queries) {
  Object.defineProperty(binding, prop, {
    configurable: true,
    enumerable: true,
    get() {
      delete binding[prop];
      try {
        binding[prop] = readFileSync(path, "utf8");
      } catch { }
      return binding[prop];
    }
  });
}

export default binding;
