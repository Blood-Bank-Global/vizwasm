//This generates GLSL_FILE_TABLE, a static HashMap of shader name to shader source, for all .glsl files in src/glsl
include!(concat!(env!("OUT_DIR"), "/glsl_table.rs"));
pub fn lookup_shader(name: &dyn AsRef<str>) -> Option<String> {
    GLSL_FILE_TABLE.get(name.as_ref()).map(|s| s.to_string())
}

pub fn include_files(src: &str) -> String {
    sdlrig::shaderhelper::include_files(src, lookup_shader)
}
