use vizwasm::shaderlookup;

pub fn main() {
    println!(
        "{}",
        shaderlookup::include_files(
            "//this is a test\n#include \"font_8x8.glsl\"\n#include \"font_8x16.glsl\"\n"
        )
    )
}
