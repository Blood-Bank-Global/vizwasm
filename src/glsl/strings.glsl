#ifndef STRINGS_GLSL
#define STRINGS_GLSL
//this code has been heavily reworked but some references:
//taken from https://www.shadertoy.com/view/mtGcWd
//referenced by https://vdmx.vidvox.net/tutorials/techniques-for-drawing-text-in-glsl


#define str_bounds(coord,pos,font_w,font_h,len) \
    ((step((pos).x, (coord).x) >= 0.5 \
    && (step((pos).y, (coord).y)) >= 0.5 \
    && step((coord).x, (pos).x + float((font_w)) * float((len))) >= 0.5 \
    && step((coord).y, (pos).y + float((font_h))) >= 0.5))

#define str_char(coord,pos, txt, start,len,font_w) \
    ((txt)[uint((start) + uint(clamp((float((coord).x) - float((pos).x))/float((font_w)), 0, (len)-1)))])

#define font_sample(char,offset,font_w,font_h,map_w,font_name) \
    (font_data_ ## font_name [ \
    ((uint((char)) % 16) * uint((font_w)) + uint((offset).x) \
    + ((uint((char)) / 16) * uint((font_h)) + uint((offset).y)) * uint((map_w)))/32u \
    ])

#define font_bitmask(char,offset_x,font_w) (1u << ((((uint(char) % 16) * uint(font_w) + uint(offset_x)) % 32u) ) )
    
#define fontstr(coord,pos,txt,start,len,font_w,font_h,map_w,font_name) \
    (str_bounds((coord),(pos),(font_w),(font_h),(len)) \
    && (font_sample(str_char((coord),(pos),(txt),(start),(len),(font_w)), \
        ivec2(uint(mod((float((coord).x) - float((pos).x)), float((font_w)))), uint(mod((float((coord).y) - float((pos).y)), float((font_h)))) ), \
        (font_w), (font_h), (map_w), font_name) \
        & font_bitmask(str_char((coord),(pos),(txt),(start),(len),(font_w)) , uint(mod((float((coord).x) - float((pos).x)), float((font_w)))) , (font_w)) ) != 0u)

#define multiline_bounds(coord,pos,font_w,font_h,starts,lens) \
    (lens).length() == (starts).length() \
    && (step((pos).y, (coord).y) >= 0.5 \
    && step((coord).y, ((pos).y + float((font_h)) * float((starts.length())))) >= 0.5 \
    && step((pos).x, (coord).x) >= 0.5 \
    && (step(uint(((coord).y - (pos).y)/float((font_h))), (lens).length()) >= 0.5) \
    && step((coord).x, (pos).x + float((font_w)) * float((lens)[uint(((coord).y - (pos).y)/float((font_h)))])) >= 0.5)

#define multiline_font(coord,pos,txt,starts,lens,font_w,font_h,map_w,font_name) \
    ((multiline_bounds((coord),(pos),float((font_w)),float((font_h)),(starts),(lens))) \
    && (fontstr( \
        (coord), \
        vec2((pos).x, (pos).y + floor(((coord).y - (pos.y))/float((font_h)))*float((font_h))), \
        (txt), \
        (starts)[uint((((coord).y - (pos.y))/float((font_h))))], \
        (lens)[uint((((coord).y - (pos.y))/float((font_h))))], \
        (uint((font_w))), \
        (uint((font_h))), \
        (uint((map_w))), \
        font_name \
    )))


// this is mostly for debugging
int float2txt( float val, out uint txt[10] ) {
    if (val > 9999.9999 || val < -9999.9999) {
        txt[0] = 79u;  // 'O'
        txt[1] = 79u;  // 'O'
        txt[2] = 80u;  // 'P'
        txt[3] = 83u;  // 'S'
        return 4;
    }

    int len = 0;
    float aval = val;

    // Handle negative sign
    if (aval < 0.0) {
        txt[len] = 45u; // '-'
        len++;
        aval = -aval;
    }

    int whole_val = int(aval);
    float frac_val = aval - float(whole_val);

    // Write whole number digits MSB first
    if (whole_val == 0) {
        txt[len] = 48u; // '0'
        len++;
    } else {
        // Count digits
        int wdigits = 0;
        int tmp = whole_val;
        for (int d = 0; d < 4; d++) {
            if (tmp == 0) break;
            wdigits++;
            tmp /= 10;
        }
        // Compute starting divisor (10^(wdigits-1))
        int wdiv = 1;
        for (int d = 0; d < wdigits - 1; d++) wdiv *= 10;
        for (int d = 0; d < wdigits; d++) {
            txt[len] = uint(whole_val / wdiv % 10) + 48u;
            len++;
            wdiv /= 10;
        }
    }

    // Write decimal digits (up to 4 places, trailing zeros trimmed)
    int dval = clamp(int(round(frac_val * 10000.0)), 0, 9999);
    if (dval > 0) {
        int ddigits = 4;
        // Trim trailing zeros by dividing dval in place
        for (int d = 0; d < 3; d++) {
            if (dval % 10 != 0) break;
            dval /= 10;
            ddigits--;
        }
        txt[len] = 46u; // '.'
        len++;
        int ddiv = 1;
        for (int d = 0; d < ddigits - 1; d++) ddiv *= 10;
        for (int d = 0; d < ddigits; d++) {
            txt[len] = uint(dval / ddiv % 10) + 48u;
            len++;
            ddiv /= 10;
        }
    }

    return len;
}

bool is_cp437_space(uint char) {
    //CP437 whitespace characters
    return char == 0x00 
      || char == 0x20
      || char == 0xFF;
}

bool is_ascii_space(uint char) {
    //ASCII whitespace characters
    return char == 0x00 
      || char == 0x20
      || char == 0x09
      || char == 0x0A
      || char == 0x0B
      || char == 0x0C
      || char == 0x0D;
}

bool is_ascii_printable(uint char) {
    //ASCII printable characters
    return char >= 0x20 && char <= 0x7E;
}
#endif