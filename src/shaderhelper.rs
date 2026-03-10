use regex::Regex;
use std::collections::HashSet;
//This generates GLSL_FILE_TABLE, a static HashMap of shader name to shader source, for all .glsl files in src/glsl
include!(concat!(env!("OUT_DIR"), "/glsl_table.rs"));

pub fn include_files<S: AsRef<str>>(shader: S) -> String {
    include_files_recusive(shader, &mut HashSet::new())
}

fn include_files_recusive<S: AsRef<str>>(shader: S, seen: &mut HashSet<String>) -> String {
    let mut output = String::new();
    let shader_str = shader.as_ref();
    let mut last = 0;
    let include_re = Regex::new(r#"(?m)^#include\s+"([^"]+)""#).unwrap();
    while last < shader_str.len() {
        if let Some(captures) = include_re.captures(&shader_str[last..]) {
            eprintln!("Found include: {}", captures.get(1).unwrap().as_str());
            let start = last + captures.get(0).unwrap().start();
            let end = last + captures.get(0).unwrap().end();
            output.push_str(&shader_str[last..start]);
            let include_name = captures.get(1).unwrap().as_str();
            if !seen.contains(include_name) {
                seen.insert(include_name.to_string());
                output.push_str("\n");
                if let Some(include_source) = GLSL_FILE_TABLE.get(include_name) {
                    output.push_str(&include_files_recusive(include_source, seen));
                } else {
                    eprintln!("Included file not found: {}", include_name);
                }
            }
            last = end;
        } else {
            output.push_str(&shader_str[last..]);
            break;
        }
    }

    return output;
}
