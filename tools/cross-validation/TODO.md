# Tree-Sitter Kotlin Grammar Issues

Structural mismatches between tree-sitter-kotlin and JetBrains PSI reference parser,
categorized by root cause and difficulty. Each issue represents a grammar.js fix needed
to match JetBrains' expected AST structure.

**Current state:** 111/134 clean parses match structurally (82.8%)

Fix these issues iteratively — after each grammar fix, re-run the cross-validation
and move fixed files from `excluded.txt` to the vendored corpus.

## How to Use

1. Pick an issue category below (start with EASY)
2. Fix the grammar in `grammar.js`
3. Run `npm run generate && npm test` to verify existing tests still pass
4. Run `npm run cross-validate` to check if affected files now match
5. Remove fixed files from `excluded.txt`
6. Re-run `npm run vendor-jetbrains` to update corpus
7. Commit

---

## EASY (2 files, 2 issues)

### duplicate_accessor (1 file)

**Problem:** Property with duplicate get/set accessor produces 3 `PROPERTY_ACCESSOR`
children where PSI expects only 2.

**Files:**
- `DuplicateAccessor`

**Likely fix:** Grammar allows more than 2 accessor declarations per property.
Add constraint or let the grammar accept it (PSI may handle this as an error).

---

### typealias_keyword (1 file)

**Problem:** `typealias` not recognized as a separate declaration in some contexts;
parsed as a property instead.

**Files:**
- `TypealiasIsKeyword`

**Likely fix:** `typealias` may not be in the keyword list or may conflict with
soft keyword handling.

---

## MEDIUM (4 files, 3 issues)

### by_delegation (1 file)

**Problem:** `DELEGATED_SUPER_TYPE_ENTRY` with `by` clause produces extra
`CALL_EXPRESSION` child and missing `CLASS_BODY`. The `by` expression is
being absorbed into the delegation entry instead of being a separate construct.

**Files:**
- `ByClauses`

**Likely fix:** `delegation_specifier` rule — the `by` clause needs to be
structured so the delegated expression is properly separated from the class body.

---

### class_body_nesting (2 files)

**Problem:** `CLASS_BODY` missing or inner class nesting structure differs from PSI.

**Files:**
- `NonTypeBeforeDotInBaseClass`
- `dependencyOnNestedClasses`

**Likely fix:** Check how nested class declarations interact with the parent
class body structure.

---

### super_expression (1 file)

**Problem:** `super` expression with type qualifier (`super<Foo>`) has wrong
structural nesting compared to PSI.

**Files:**
- `Super`

**Likely fix:** `super_expression` rule — type argument in angle brackets needs
correct nesting.

---

## HARD (10 files, 4 issues)

### lambda_nesting (2 files)

**Problem:** Lambda bodies have wrong `CALL_EXPRESSION` vs `BLOCK` nesting.
`FUNCTION_LITERAL` children are in the wrong order or at the wrong depth.

**Files:**
- `CommentsBindingInLambda`
- `FunctionLiterals`

**Likely fix:** `lambda_literal` and `function_literal` rules need restructuring.
The interaction between trailing lambdas and call expressions creates cascading
nesting differences.

---

### type_argument_placement (4 files)

**Problem:** `TYPE_ARGUMENT_LIST` placed at wrong nesting level in `USER_TYPE`.
Affects generics like `Foo<A>.Bar<B>` — tree-sitter attaches type arguments
to the wrong part of the qualified type chain.

**Files:**
- `complicateLTGT`
- `incorrectLTGTFallback`
- `definitelyNotNullTypes`
- `typeAliasExpansion`

**Likely fix:** `user_type` rule — type arguments need to be attached per-segment
in dot-qualified types, not hoisted to the outermost type.

---

### type_modifiers (1 file)

**Problem:** `suspend` and other type modifiers not mapped to correct PSI structure.
`MODIFIER_LIST` placement differs from JetBrains expectations.

**Files:**
- `suspendLambda`

**Likely fix:** Type modifier handling in function types and lambda types needs
restructuring to match PSI's `MODIFIER_LIST` placement.

---

### annotation_structure (3 files)

**Problem:** `ANNOTATION_ENTRY` / `ANNOTATION_TARGET` / `CONSTRUCTOR_CALLEE`
nesting differs from PSI. Annotations with arguments, use-site targets, and
annotations on type parameters all have structural differences.

**Files:**
- `annotationsOnParenthesizedTypes`
- `annotationValues`
- `TypeParametersBeforeName`

**Likely fix:** Annotation rules need comprehensive restructuring to match PSI's
three-level nesting: `ANNOTATION_ENTRY` > `ANNOTATION_TARGET` + `CONSTRUCTOR_CALLEE`.
