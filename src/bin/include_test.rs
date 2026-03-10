use vizwasm::shaderhelper;

pub fn main() {
    println!(
        "{}",
        shaderhelper::include_files(
            "//this is a test\n#include \"font_8x8.glsl\"\n#include \"font_8x16.glsl\"\n"
        )
    )
}
