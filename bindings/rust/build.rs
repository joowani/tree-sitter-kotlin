fn main() {
    let src_dir = std::path::Path::new("src");

    let mut c_config = cc::Build::new();
    c_config.std("c11").include(src_dir);

    // The crate is published as `tree-sitter-kt` because `tree-sitter-kotlin` is taken on crates.io. Renaming the C
    // symbols to match lets it link into the same binary as another Kotlin grammar crate, which would otherwise define
    // the same `tree_sitter_kotlin` symbols and silently replace one grammar with the other.
    for suffix in [
        "",
        "_external_scanner_create",
        "_external_scanner_destroy",
        "_external_scanner_scan",
        "_external_scanner_serialize",
        "_external_scanner_deserialize",
    ] {
        let renamed = format!("tree_sitter_kt{suffix}");
        c_config.define(&format!("tree_sitter_kotlin{suffix}"), renamed.as_str());
    }

    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    if std::env::var("TARGET").is_ok_and(|target| target == "wasm32-unknown-unknown") {
        let Ok(wasm_headers) = std::env::var("DEP_TREE_SITTER_LANGUAGE_WASM_HEADERS") else {
            panic!("Environment variable DEP_TREE_SITTER_LANGUAGE_WASM_HEADERS must be set by the language crate");
        };

        c_config.include(&wasm_headers);
    }

    let parser_path = src_dir.join("parser.c");
    c_config.file(&parser_path);
    println!("cargo:rerun-if-changed={}", parser_path.display());

    let scanner_path = src_dir.join("scanner.c");
    if scanner_path.exists() {
        c_config.file(&scanner_path);
        println!("cargo:rerun-if-changed={}", scanner_path.display());
    }

    c_config.compile("tree-sitter-kt");

    println!("cargo:rustc-check-cfg=cfg(with_highlights_query)");
    if !"queries/highlights.scm".is_empty() && std::path::Path::new("queries/highlights.scm").exists() {
        println!("cargo:rustc-cfg=with_highlights_query");
    }
    println!("cargo:rustc-check-cfg=cfg(with_injections_query)");
    if !"queries/injections.scm".is_empty() && std::path::Path::new("queries/injections.scm").exists() {
        println!("cargo:rustc-cfg=with_injections_query");
    }
    println!("cargo:rustc-check-cfg=cfg(with_locals_query)");
    if !"queries/locals.scm".is_empty() && std::path::Path::new("queries/locals.scm").exists() {
        println!("cargo:rustc-cfg=with_locals_query");
    }
    println!("cargo:rustc-check-cfg=cfg(with_tags_query)");
    if !"queries/tags.scm".is_empty() && std::path::Path::new("queries/tags.scm").exists() {
        println!("cargo:rustc-cfg=with_tags_query");
    }
}
