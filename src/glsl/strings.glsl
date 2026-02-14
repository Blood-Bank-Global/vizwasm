//this code has been heavily reworked but some references:
//taken from https://www.shadertoy.com/view/mtGcWd
//referenced by https://vdmx.vidvox.net/tutorials/techniques-for-drawing-text-in-glsl


#define str_bounds(uv,pos,font_w,font_h,len) \
    ((step((pos).x, (uv).x) >= 0.5 \
    && (step((pos).y, (uv).y)) >= 0.5 \
    && step((uv).x, (pos).x + float((font_w)) * float((len))) >= 0.5 \
    && step((uv).y, (pos).y + float((font_h))) >= 0.5))

#define str_char(uv,pos, txt, start,len,font_w) \
    ((txt)[uint((start) + uint(clamp((float((uv).x) - float((pos).x))/float((font_w)), 0, (len)-1)))])

#define font_sample(char,offset,font_w,font_h,map_w,font_name) \
    (font_data_ ## font_name [ \
    ((uint((char)) % 16) * uint((font_w)) + uint((offset).x) \
    + ((uint((char)) / 16) * uint((font_h)) + uint((offset).y)) * uint((map_w)))/32u \
    ])

#define font_bitmask(char,offset_x,font_w) (1u << ((((uint(char) % 16) * 8u + uint(offset_x)) % 32u) ) )
    
#define fontstr(uv,pos,txt,start,len,font_w,font_h,map_w,font_name) \
    (str_bounds((uv),(pos),(font_w),(font_h),(len)) \
    && (font_sample(str_char((uv),(pos),(txt),(start),(len),(font_w)), \
        ivec2(uint(mod((float((uv).x) - float((pos).x)), float((font_w)))), uint(mod((float((uv).y) - float((pos).y)), float((font_h)))) ), \
        (font_w), (font_h), (map_w), font_name) \
        & font_bitmask(str_char((uv),(pos),(txt),(start),(len),(font_w)) , uint(mod((float((uv).x) - float((pos).x)), float((font_w)))) , (font_w)) ) != 0u)

#define multiline_bounds(uv,pos,font_w,font_h,starts,lens) \
    (lens).length() == (starts).length() \
    && (step((pos).y, (uv).y) >= 0.5 \
    && step((uv).y, ((pos).y + float((font_h)) * float((starts.length())))) >= 0.5 \
    && step((pos).x, (uv).x) >= 0.5 \
    && step(uint((uv).y/float((font_h))), (lens).length()) >= 0.5 \
    && step((uv).x, (pos).x + float((font_w)) * float((lens)[uint((uv).y/float((font_h)))])) >= 0.5)

#define multiline_font(uv,pos,txt,starts,lens,font_w,font_h,map_w,font_name) \
    ((multiline_bounds((uv),(pos),float((font_w)),float((font_h)),(starts),(lens))) \
    && (fontstr( \
        (uv), \
        vec2((pos).x, (pos).y + floor(((uv).y - (pos.y))/float((font_h)))*float((font_h))), \
        (txt), \
        (starts)[uint((((uv).y - (pos.y))/float((font_h))))], \
        (lens)[uint((((uv).y - (pos.y))/float((font_h))))], \
        (uint((font_w))), \
        (uint((font_h))), \
        (uint((map_w))), \
        font_name \
    )))


// this is mostly for debugging
int float2txt( float val, out int[10] txt ) {
    if (val > 9999.9999 || val < -9999.9999) {
        txt[0] = 0x4F; // no character type so this is the asci for OOPS
        txt[1] = 0x4F;
        txt[2] = 0x50;
        txt[3] = 0x53;
        return 4;
    }

    int len = 0;
    int whole[4] = { 0, 0, 0, 0 };
    int whole_val = int( val );

    int i;
    for ( i = 0; i < 4; i++ ) {
        if (whole_val <= int(pow(10.0, float(i))))  break;
        whole[i] = (int( whole_val / int(pow(10.0, float(i))) ) % 10) + 48;
    }

    if (i == 0) {
        txt[0] = 48; // '0'
        len = 1;
    } else {
        len = i;
        for ( ; i >= 4; i-- ) {
            txt[4 - i] = whole[i - 1];
        }
    }

    int decimal[4] = { 0, 0, 0, 0 };
    int decimal_val = int( fract( val ) * 10000.0 );
    int j;
    for (j = 0; j < 4; j++ ) {
        if (decimal_val <= int(pow(10.0, float(j))))  break;
        decimal[j] = (int( decimal_val / int(pow(10.0, float(3 - j))) ) % 10) + 48;
    }
    if (j == 0) {
        return len;
    } else {
        txt[len] = 46; // '.'
        len += 1;
        for ( int k = 0; k < j; k++ ) {
            txt[len + k] = decimal[k];
        }
        len += j;
    }
}
