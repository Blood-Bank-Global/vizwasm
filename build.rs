use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("glsl_table.rs");
    let mut out = fs::File::create(&dest_path).unwrap();

    let glsl_dir = Path::new("src/glsl");
    let mut entries: Vec<String> = Vec::new();

    if glsl_dir.is_dir() {
        let mut files: Vec<_> = fs::read_dir(glsl_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "glsl"))
            .collect();
        files.sort_by_key(|e| e.file_name());

        for entry in &files {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();
            entries.push(format!(
                "    h.insert(\"{name}\", include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/glsl/{name}\")));"
            ));
        }
    }

    writeln!(
        out,
        "pub static GLSL_FILE_TABLE: std::sync::LazyLock<std::collections::HashMap<&'static str, &'static str>> = std::sync::LazyLock::new(|| {{\n\
         \x20   let mut h = std::collections::HashMap::new();\n\
         {entries}\n\
         \x20   h\n\
         }});",
        entries = entries.join("\n")
    )
    .unwrap();

    println!("cargo:rerun-if-changed=src/glsl");
    if glsl_dir.is_dir() {
        for entry in fs::read_dir(glsl_dir).unwrap().filter_map(|e| e.ok()) {
            if entry.path().extension().map_or(false, |ext| ext == "glsl") {
                println!("cargo:rerun-if-changed={}", entry.path().display());
            }
        }
    }
}
