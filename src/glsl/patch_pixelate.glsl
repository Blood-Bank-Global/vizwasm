#include "utils.glsl"
#include "font_8x16.glsl"
#include "font_arcade.glsl"
#include "font_cyber.glsl"
#include "font_bubbly.glsl"
vec4 patch_pixelate(vec2 coord, vec2 pixel_size, sampler2D tex, vec2 reso) {
    vec2 pixelated_coord = floor(coord / pixel_size) * pixel_size + pixel_size * 0.5;
    return texture(tex, pixelated_coord / reso);
}

//!STR density_characters " -',;:\\*)|/1ftlabkDRPBXS2Z0QM&%#@@"

vec4 patch_textelate(vec2 coord, float scale, sampler2D tex, vec2 reso) {
    vec2 pixel_size = vec2(font_8x16_width, font_8x16_height) * scale;
    vec2 pixelated_coord = floor(coord / pixel_size) * pixel_size;
    vec4 color = texture(tex, pixelated_coord / reso);
    vec3 hsv = rgb2hsv(color.rgb);
    float luma = hsv.z;
    uint density = uint(luma * uint(density_characters_length));
    uint c[] = uint[](density_characters[density]);
    if (font_8x16(coord/scale, pixelated_coord/scale, c, 0, 1)) {
        return color;
    } else {
        return vec4(0.0, 0.0, 0.0, color.a);
    }
}

vec4 patch_textelate_arcade(vec2 coord, float scale, sampler2D tex, vec2 reso) {
    vec2 pixel_size = vec2(font_arcade_width, font_arcade_height) * scale;
    vec2 pixelated_coord = floor(coord / pixel_size) * pixel_size;
    vec4 color = texture(tex, pixelated_coord / reso);
    vec3 hsv = rgb2hsv(color.rgb);
    float luma = hsv.z;
    uint density = uint(luma * uint(density_characters_length));
    uint c[] = uint[](density_characters[density]);
    if (font_arcade(coord/scale, pixelated_coord/scale, c, 0, 1)) {
        return color;
    } else {
        return vec4(0.0, 0.0, 0.0, color.a);
    }
}

vec4 patch_textelate_cyber(vec2 coord, float scale, sampler2D tex, vec2 reso) {
    vec2 pixel_size = vec2(font_cyber_width, font_cyber_height) * scale;
    vec2 pixelated_coord = floor(coord / pixel_size) * pixel_size;
    vec4 color = texture(tex, pixelated_coord / reso);
    vec3 hsv = rgb2hsv(color.rgb);
    float luma = hsv.z;
    uint density = uint(luma * uint(density_characters_length));
    uint c[] = uint[](density_characters[density]);
    if (font_cyber(coord/scale, pixelated_coord/scale, c, 0, 1)) {
        return color;
    } else {
        return vec4(0.0, 0.0, 0.0, color.a);
    }
}

vec4 patch_textelate_bubbly(vec2 coord, float scale, sampler2D tex, vec2 reso) {
    vec2 pixel_size = vec2(font_bubbly_width, font_bubbly_height) * scale;
    vec2 pixelated_coord = floor(coord / pixel_size) * pixel_size;
    vec4 color = texture(tex, pixelated_coord / reso);
    vec3 hsv = rgb2hsv(color.rgb);
    float luma = hsv.z;
    uint density = uint(luma * uint(density_characters_length));
    uint c[] = uint[](density_characters[density]);
    if (font_bubbly(coord/scale, pixelated_coord/scale, c, 0, 1)) {
        return color;
    } else {
        return vec4(0.0, 0.0, 0.0, color.a);
    }
}

//!VAR uint[] boxel_characters 0x20u 0xB0 0xB1 0xB2 0xB2 
vec4 patch_boxelate(vec2 coord, float scale, sampler2D tex, vec2 reso) {
    vec2 pixel_size = vec2(font_8x16_width, font_8x16_height) * scale;
    vec2 pixelated_coord = floor(coord / pixel_size) * pixel_size;
    vec4 color = texture(tex, pixelated_coord / reso);
    vec3 hsv = rgb2hsv(color.rgb);
    float luma = hsv.z;
    uint density = uint(luma * uint(boxel_characters.length()));
    uint c[] = uint[](boxel_characters[density]);
    if (font_8x16(coord/scale, pixelated_coord/scale, c, 0, 1)) {
        return color;
    } else {
        return vec4(0.0, 0.0, 0.0, color.a);
    }
}