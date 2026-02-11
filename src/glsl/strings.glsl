//taken from https://www.shadertoy.com/view/mtGcWd
//referenced by https://vdmx.vidvox.net/tutorials/techniques-for-drawing-text-in-glsl

uint data[ 1024 ] = uint[](
        0u, 0u, 4278255360u, 0u, 
        0u, 0u, 4278255360u, 0u, 
        0u, 0u, 4278255360u, 0u, 
        8289792u, 1579008u, 4278255390u, 1010794240u, 
        8519532u, 272382976u, 4278255374u, 1714643736u, 
        10869758u, 943488512u, 4282172186u, 1715437336u, 
        8519678u, 2095578904u, 3882260786u, 1714447323u, 
        8519678u, 4276617020u, 3275931000u, 1714447164u, 
        12436478u, 2095545916u, 3275931084u, 1009804263u, 
        10086268u, 941103128u, 3882260940u, 405824316u, 
        8519480u, 270014464u, 4282172364u, 2121295835u, 
        8519440u, 3947520u, 4278255564u, 418440984u, 
        8289792u, 0u, 4278255480u, 417392152u, 
        0u, 0u, 4278255360u, 49152u, 
        0u, 0u, 4278255360u, 0u, 
        0u, 0u, 4278255360u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        2147614720u, 8126464u, 0u, 0u, 
        3221624934u, 2143682584u, 404226048u, 0u, 
        3759029350u, 3680501820u, 1008205824u, 0u, 
        4028530278u, 3677880446u, 2115502080u, 4350u, 
        4164819046u, 3681288216u, 404232240u, 2636030u, 
        4278065254u, 2076573720u, 404229216u, 3228317820u, 
        4164819046u, 465960984u, 404291326u, 3237903484u, 
        4028530278u, 460127870u, 404229216u, 3228335160u, 
        3759029248u, 456719932u, 410916912u, 4264099384u, 
        3221624934u, 453836312u, 406585344u, 65040u, 
        2147614822u, 466026110u, 404226048u, 0u, 
        0u, 8126464u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 402653184u, 0u, 0u, 
        26112u, 402653232u, 0u, 0u, 
        1598976u, 2080389168u, 204472320u, 0u, 
        3958380u, 3321916464u, 404226048u, 0u, 
        3941484u, 3267521632u, 806092800u, 2u, 
        3932414u, 3234215936u, 806118936u, 6u, 
        1572972u, 2081191424u, 806108184u, 12u, 
        1572972u, 102292480u, 806158206u, 16646168u, 
        1572972u, 103861248u, 806108184u, 48u, 
        254u, 2254490624u, 806118936u, 402653280u, 
        1572972u, 3334917120u, 404226048u, 402659520u, 
        1572972u, 2089186816u, 204472320u, 402659456u, 
        0u, 402653184u, 0u, 805306368u, 
        0u, 402653184u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        941128828u, 217987326u, 2088501248u, 124u, 
        1815660230u, 482369734u, 3334864896u, 100688070u, 
        3329754630u, 1019265030u, 3334871064u, 201339078u, 
        3323464710u, 1824571398u, 3334871064u, 410916876u, 
        3591903292u, 3439131660u, 2088632320u, 805309464u, 
        3591909382u, 4261856792u, 3322281984u, 1610614296u, 
        3323486214u, 201770544u, 3322281984u, 813567000u, 
        3323510790u, 201770544u, 3322288152u, 402659328u, 
        1813563078u, 214353456u, 3322681368u, 201338904u, 
        947846780u, 511474736u, 2088239152u, 100687896u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        1113148u, 4177460796u, 3325828838u, 4039558780u, 
        2084071014u, 1818650214u, 3323464806u, 1626269382u, 
        3328992962u, 1717723842u, 3323464806u, 1627322054u, 
        3334891200u, 1718118592u, 3323464812u, 1627324102u, 
        3737550016u, 1719171264u, 4262988920u, 1624694470u, 
        3741214400u, 1718118622u, 3323464824u, 1623641798u, 
        3737544384u, 1717592262u, 3323513964u, 1623639750u, 
        3703989954u, 1717723334u, 3323513958u, 1657194182u, 
        3234227814u, 1818648678u, 3323513958u, 1724303046u, 
        2093415484u, 4177457210u, 3325851878u, 4274439804u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 4096u, 
        0u, 0u, 0u, 14336u, 
        4236049532u, 2126956230u, 3328638524u, 3959808u, 
        1724278470u, 2126956230u, 3328624176u, 2148320768u, 
        1724278470u, 1522976454u, 1818658352u, 3222011904u, 
        1724278368u, 415680198u, 2087062576u, 3758882816u, 
        2093382712u, 415680214u, 943462448u, 1879834624u, 
        1623616524u, 415680214u, 941109296u, 940310528u, 
        1623614982u, 415680214u, 2081972272u, 470548480u, 
        1624663750u, 415657214u, 1813561904u, 235667456u, 
        1625188038u, 415643886u, 3323512368u, 101449728u, 
        4034717308u, 1014763628u, 3325886012u, 37486592u, 
        786432u, 0u, 0u, 0u, 
        917504u, 0u, 0u, 255u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        805306368u, 0u, 0u, 0u, 
        805306368u, 0u, 0u, 0u, 
        402710528u, 469776384u, 3759671008u, 939524096u, 
        24576u, 201354240u, 1612187232u, 402653184u, 
        24576u, 201352192u, 1610612832u, 402653184u, 
        7895164u, 1014784118u, 1815613030u, 418176124u, 
        814278u, 1824977100u, 1981285996u, 419325638u, 
        8152768u, 3439222988u, 1712850552u, 416704198u, 
        13395648u, 3435159756u, 1712850552u, 416704198u, 
        13395648u, 3435159756u, 1712850540u, 416704198u, 
        13395654u, 3435552972u, 1712850534u, 416704198u, 
        7765116u, 1987899516u, 3862693606u, 1019635324u, 
        0u, 12u, 26112u, 0u, 
        0u, 204u, 26112u, 0u, 
        0u, 120u, 15360u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 268435456u, 7u, 205011712u, 
        0u, 805306368u, 12u, 202141184u, 
        0u, 805306368u, 12u, 202113032u, 
        3698777212u, 4234556259u, 1667464972u, 202113052u, 
        1724675782u, 812004195u, 912483896u, 458806u, 
        1724671584u, 812004203u, 476253196u, 202113123u, 
        1724670008u, 812004203u, 476256268u, 202113123u, 
        1724669964u, 812004203u, 476262412u, 202113123u, 
        1724670150u, 912662143u, 912483084u, 202113151u, 
        2088562812u, 473631798u, 1665105671u, 204996608u, 
        1611399168u, 0u, 196608u, 0u, 
        1611399168u, 0u, 393216u, 0u, 
        4028497920u, 0u, 8126464u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 56u, 
        3088u, 6305792u, 268460032u, 408995436u, 
        1020008504u, 3425725440u, 952512614u, 1009778744u, 
        1711288428u, 1587200u, 1811945472u, 1712852992u, 
        3254779904u, 60u, 0u, 14392u, 
        3234626680u, 2021161062u, 2088533048u, 943221868u, 
        3234645516u, 202116192u, 3334915608u, 404276934u, 
        3234659964u, 2088533088u, 4278124056u, 404276934u, 
        3268198604u, 3435973734u, 3233857560u, 404291326u, 
        1724694732u, 3435973692u, 3233857560u, 404276934u, 
        1020053196u, 3435973644u, 3334915608u, 404276934u, 
        209091702u, 1987474950u, 2088533052u, 1010616006u, 
        100663296u, 60u, 0u, 0u, 
        2080374784u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        402653184u, 0u, 0u, 0u, 
        805306384u, 6303840u, 13026840u, 939587598u, 
        1610628664u, 3325065264u, 3321888792u, 1818676251u, 
        27756u, 1625112u, 8177212u, 1684458520u, 
        4261465088u, 0u, 13026918u, 1614608408u, 
        1724697724u, 2088553676u, 3334915680u, 4028154904u, 
        1618411206u, 3334917324u, 3334915680u, 1618922622u, 
        2083966150u, 3334917324u, 3334915680u, 1612242456u, 
        1618922694u, 3334917324u, 3334915686u, 1618922520u, 
        1624820934u, 3334917324u, 3334915644u, 1612237848u, 
        1725484230u, 3334917324u, 3334915608u, 3860384792u, 
        4268674684u, 2088531574u, 2122087448u, 4229482008u, 
        0u, 0u, 100663296u, 216u, 
        0u, 0u, 201326592u, 112u, 
        0u, 0u, 2013265920u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 7733248u, 0u, 0u, 
        403445784u, 14433336u, 192u, 3221225472u, 
        806891568u, 1979739244u, 805306560u, 3222798336u, 
        1613783136u, 3703991404u, 805306562u, 3256352768u, 
        0u, 15089208u, 198u, 3321888768u, 
        2016967884u, 3707109376u, 805306572u, 3424138968u, 
        202950348u, 1727954556u, 822017560u, 404253804u, 
        2081998540u, 1725825024u, 1623197232u, 806934582u, 
        3424175820u, 1724776448u, 3233810016u, 1715235948u, 
        3424175820u, 1724252160u, 3334473436u, 3460052696u, 
        3424175820u, 1724252160u, 3334473350u, 2654732288u, 
        1983675510u, 1724252160u, 2080374796u, 1041760256u, 
        0u, 0u, 24u, 100663296u, 
        0u, 0u, 62u, 100663296u, 
        0u, 0u, 0u, 0u, 
        290839832u, 404239872u, 3552768u, 909514752u, 
        1152022296u, 404239872u, 3552768u, 909514752u, 
        290839832u, 404239872u, 3552768u, 909514752u, 
        1152022296u, 404239872u, 3552768u, 909514752u, 
        290839832u, 404239872u, 3552768u, 909514752u, 
        1152022296u, 418919936u, 4176885502u, 4130797568u, 
        290839832u, 404239872u, 403060230u, 104208384u, 
        1152022296u, 4177065726u, 4176885494u, 4278122744u, 
        290839832u, 404239926u, 406206006u, 24u, 
        1152022296u, 404239926u, 406206006u, 24u, 
        290839832u, 404239926u, 406206006u, 24u, 
        1152022296u, 404239926u, 406206006u, 24u, 
        290839832u, 404239926u, 406206006u, 24u, 
        1152022296u, 404239926u, 406206006u, 24u, 
        290839832u, 404239926u, 406206006u, 24u, 
        1152022296u, 404239926u, 406206006u, 24u, 
        404226072u, 1579062u, 905983488u, 905983512u, 
        404226072u, 1579062u, 905983488u, 905983512u, 
        404226072u, 1579062u, 905983488u, 905983512u, 
        404226072u, 1579062u, 905983488u, 905983512u, 
        404226072u, 1579062u, 905983488u, 905983512u, 
        404226072u, 1580854u, 926939135u, 939522047u, 
        404226072u, 1579062u, 808452096u, 805306368u, 
        536870687u, 4294909751u, 1060634615u, 939522047u, 
        6168u, 1579062u, 3538998u, 905983488u, 
        6168u, 1579062u, 3538998u, 905983488u, 
        6168u, 1579062u, 3538998u, 905983488u, 
        6168u, 1579062u, 3538998u, 905983488u, 
        6168u, 1579062u, 3538998u, 905983488u, 
        6168u, 1579062u, 3538998u, 905983488u, 
        6168u, 1579062u, 3538998u, 905983488u, 
        6168u, 1579062u, 3538998u, 905983488u, 
        905969718u, 402653238u, 404226303u, 15732735u, 
        905969718u, 402653238u, 404226303u, 15732735u, 
        905969718u, 402653238u, 404226303u, 15732735u, 
        905969718u, 402653238u, 404226303u, 15732735u, 
        905969718u, 402653238u, 404226303u, 15732735u, 
        922681398u, 522125366u, 4279763199u, 15732735u, 
        905969718u, 404226102u, 404226303u, 15732735u, 
        4294967103u, 522141695u, 4294451199u, 4293922560u, 
        1586688u, 1586742u, 402659583u, 4293922560u, 
        1586688u, 1586742u, 402659583u, 4293922560u, 
        1586688u, 1586742u, 402659583u, 4293922560u, 
        1586688u, 1586742u, 402659583u, 4293922560u, 
        1586688u, 1586742u, 402659583u, 4293922560u, 
        1586688u, 1586742u, 402659583u, 4293922560u, 
        1586688u, 1586742u, 402659583u, 4293922560u, 
        1586688u, 1586742u, 402659583u, 4293922560u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        7929344u, 0u, 14366u, 7168u, 
        13420032u, 4261412864u, 2117626928u, 209020u, 
        13420286u, 3321914998u, 409781784u, 417990u, 
        1993130092u, 1618896604u, 1019659788u, 2122211526u, 
        3705192556u, 819488280u, 1724302910u, 3688594630u, 
        3637297260u, 416835096u, 1727949926u, 3688587462u, 
        3636904044u, 819488280u, 1724279910u, 3690160326u, 
        3636904044u, 1624800280u, 1019636838u, 2122211526u, 
        3704012908u, 3336069144u, 409758822u, 6303942u, 
        1993130092u, 4268777496u, 2117660220u, 12590278u, 
        0u, 49152u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 0u, 0u, 0u, 
        0u, 1572864u, 0u, 0u, 
        0u, 1572864u, 0u, 0u, 
        0u, 1572864u, 939524111u, 3631218688u, 
        0u, 236453888u, 1811939340u, 1826095104u, 
        12300u, 454557696u, 1811939340u, 1815085056u, 
        4262991896u, 454563840u, 939524108u, 1818262528u, 
        1575984u, 404232310u, 12u, 1825078272u, 
        8259168u, 404226268u, 12u, 1828224000u, 
        4262988848u, 404258304u, 1573100u, 31744u, 
        1579032u, 416809078u, 1579116u, 31744u, 
        12300u, 416815324u, 108u, 31744u, 
        4261412864u, 416815104u, 60u, 31744u, 
        16744062u, 409993216u, 28u, 0u, 
        0u, 402653184u, 0u, 0u, 
        0u, 402653184u, 0u, 0u, 
        0u, 402653184u, 0u, 0u
);

int float2txt( float val, out int[128] txt ) {
    if (val > 9999.9999) {
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

// referencing a pixel, for an extended ASCII character in Code Page 37
//    expected ranges of char are 0-255
//    expected ranges of offset are within the 8x16 neighborhood
int fontRef( uint char, ivec2 offset ) {
    bool offsetOOB = any( lessThan(         offset, ivec2( 0,  0 ) ) ) ||
                     any( greaterThanEqual( offset, ivec2( 8, 16 ) ) );
    bool charOOB = bool( clamp( char, 0u, 255u ) != char );
    if ( offsetOOB || charOOB ) {
        return -1; // oob
    }
    uvec2 sampleLoc = uvec2( char % 16u, char / 16u ) * uvec2( 8u, 16u ) + uvec2( offset );
    uint idx = ( sampleLoc.x + sampleLoc.y * 128u ) / 32u;
    uint packedData = data[ idx ];
    uint bitMask = 1u << ( 31u - sampleLoc.x % 32u );
    return int( ( packedData & bitMask ) != 0u );
}

vec4 draw_text(in vec4 in_color, in vec2 uv_in, in vec2 pos_in, in vec2 resolution_in, in int[128] textData, in int len) {
    ivec2 uv = ivec2( uv_in );
    ivec2 pos = ivec2( pos_in );
    ivec2 resolution = ivec2( resolution_in );
    if (uv.x < pos.x || uv.y < pos.y|| uv.x >= (pos.x + int(8 * len)) || uv.y >= (pos.y + 16)) {
        return in_color;
    }

    int charIndex = (uv.x - pos.x) / 8;
    int charCode = int( textData[ uint(charIndex) ] & 0xFFu );
    int onGlyph = fontRef(charCode, ivec2(mod(uv.x - pos.x, 8), mod(uv.y - pos.y, 16)) );
    vec3 col = vec3( 0.0f );
    switch ( onGlyph ) {
        case -1: col = vec3( 0.1618f ); break; // out of bounds
        case  0: col = vec3( 0.0f );    break; // "off" pixel
        case  1: col = vec3( 0.7f );  break; // "on" pixel
    }

    // Output to screen
    return vec4( col, 1.0f );
}


vec4 char8x16(in vec4 in_color, in vec2 uv, in vec2 pos, float scale, in int charCode) {
    float scaledWidth = 8.0f * scale;
    float scaledHeight = 16.0f * scale;
    if (uv.x < pos.x || uv.y < pos.y|| uv.x >= (pos.x + scaledWidth) || uv.y >= (pos.y + scaledHeight)) {
        return in_color;
    }

    int bit_w = int((uv.x - pos.x) / scaledWidth * 8.0);
    int bit_h = int((uv.y - pos.y) / scaledHeight * 16.0);
    int onGlyph = fontRef(charCode, ivec2(bit_w, bit_h) );
    vec3 col = vec3( 0.0f );
    switch ( onGlyph ) {
        case -1: col = vec3( 0.1618f ); break; // out of bounds
        case  0: col = vec3( 0.0f );    break; // "off" pixel
        case  1: col = vec3( 0.7f );  break; // "on" pixel
    }

    // Output to screen
    return vec4( col, 1.0f );
}

/*
medievalish_font (PNM).
DB_Medievalish_Chonker_8x8_1bbp_bmp_font
copyright by Dennis Busch, https:/www.dennisbusch.de

Dennis Busch grants purchasers of this font the following license:


PERSONAL LICENSE

The non-transferable, non-exclusive rights to use the assets in this pack for
personal projects and products and for release as part of non-commercial,
non-for-profit projects and products with the explicit ex-clusion of the right
to re-distribute the pack in full or in parts or derivatives of it on its own or
as part of other packs.
*/

/*
  m.h (GIF).
*/
 uint medievalish_font[512] = uint[](
0x0, 0x0, 0x0, 0x0,
0x7070700, 0x7070707, 0x7070707, 0x7070707,
0x5050500, 0x5050505, 0x5050505, 0x5050505,
0x5050500, 0x5050505, 0x5050505, 0x5050505,
0x5050500, 0x5050505, 0x5050505, 0x5050505,
0x5050500, 0x5050505, 0x5050505, 0x5050505,
0x7070700, 0x7070707, 0x7070707, 0x7070707,
0x0, 0x0, 0x0, 0x0,
0x0, 0x0, 0x0, 0x0,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x0, 0x0, 0x0, 0x0,
0x33771800, 0x380E6048, 0x10220E70, 0x60000000,
0x66661800, 0x301B6C3E, 0x18361818, 0x60000000,
0xFF331800, 0x184E6C0B, 0xFE1C300C, 0x70007E00,
0x66000800, 0x2E303E, 0x7F7F300C, 0x30003F00,
0xFF000000, 0x3B1868, 0x181C300C, 0x18000000,
0x66001800, 0xB36C3E, 0x836381C, 0xC18001C,
0x33001800, 0x5E6209, 0x221E78, 0x2180018,
0x0, 0x0, 0x0, 0x0,
0x3C08303C, 0x7C183C30, 0x3C3C, 0x3C0E0070,
0x121C3866, 0x3E0C0618, 0x6666, 0x6E187C18,
0x8322456, 0x10063E2C, 0x18187C3C, 0x64303E0C,
0x3C18304E, 0x183E207E, 0x1C1C6066, 0x30600006,
0x300C3066, 0x3C662230, 0x6066, 0x307C0C,
0x3A2E307E, 0x187E3630, 0x1C387676, 0x18183E18,
0x1C3E183C, 0x83C1C10, 0x18183C3C, 0x180E0070,
0x0, 0x0, 0x0, 0x0,
0x3F1F1F3E, 0x3F3F3F3F, 0x637C3C33, 0x3E33C70B,
0x62163243, 0x62626262, 0x36100866, 0x63666E06,
0x77F275B, 0x7070747, 0x1F381C67, 0x676F7B07,
0x6463E7B, 0x761E1E46, 0xE30187E, 0x637E5606,
0x46462603, 0x66060646, 0x1E341866, 0x63764606,
0x7E7E667B, 0x6E066E76, 0x36321066, 0x7F66466E,
0x3B3B433E, 0x3B033B3B, 0x631E3C63, 0x3E63C33B,
0x0, 0x0, 0x0, 0x0,
0x3E3F3E3F, 0xC743437E, 0x787F3343, 0x81E06,
0x53626362, 0x4666661B, 0xC656666, 0x1C3006,
0x3676767, 0x4767671B, 0xC306E3F, 0x76300E,
0x3E36633E, 0x5666661E, 0xC1C3C1C, 0x63300C,
0x601E4306, 0x7E6E6618, 0xC06181E, 0x3018,
0x75363606, 0x6E3C7E18, 0xC5318B7, 0xFE003030,
0x3E636F03, 0xC718DC30, 0x787F0C63, 0x7F001E40,
0x0, 0x0, 0x0, 0x0,
0x20006, 0x700010, 0x2201002, 0xC,
0x40044006, 0x180020, 0x44301804, 0x18,
0x3C3C3C0C, 0x7C7C7C3C, 0x6C00000C, 0x3C365A18,
0x266C2618, 0x26182626, 0x3C30183C, 0x667DFF18,
0x64C3600, 0x36181636, 0x6C301C6C, 0x6E6CD618,
0x7E7C3E00, 0x7C187E3E, 0x4C34184C, 0x666CC658,
0x3C3E7C00, 0x210C3C7C, 0x46323C46, 0x3C266330,
0x0, 0x1E000000, 0x1C0000, 0x0,
0x0, 0xC, 0x70000000, 0xE10,
0x40000000, 0x18, 0x107E3366, 0x7200818,
0x3C3B5C3E, 0x8343437E, 0x1831661D, 0x54C1818,
0xE6E664C, 0xC6666618, 0xE186638, 0x57E7018,
0x3C066E6C, 0xD6666618, 0x184C7C1C, 0x5321818,
0x71067C3E, 0xFE3C7E18, 0x107E31B8, 0x5040818,
0x3E04600C, 0xB6183C30, 0x703F1E6D, 0x7000E08,
0x6006, 0x0, 0x6, 0x0,
0x0, 0x0, 0x0, 0x0,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x0, 0x0, 0x0, 0x0,
0x0, 0x0, 0x0, 0x0,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x0, 0x0, 0x0, 0x0,
0x0, 0x0, 0x0, 0x0,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x0, 0x0, 0x0, 0x0,
0x0, 0x0, 0x0, 0x0,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x0, 0x0, 0x0, 0x0,
0x0, 0x0, 0x0, 0x0,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x0, 0x0, 0x0, 0x0,
0x0, 0x0, 0x0, 0x0,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x0, 0x0, 0x0, 0x0,
0x0, 0x0, 0x0, 0x0,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x5050505, 0x5050505, 0x5050505, 0x5050505,
0x7070707, 0x7070707, 0x7070707, 0x7070707,
0x0, 0x0, 0x0, 0x0,
0x0, 0x0, 0x0, 0x0,
0x7070707, 0x7070707, 0x7070707, 0x70707,
0x5050505, 0x5050505, 0x5050505, 0x50505,
0x5050505, 0x5050505, 0x5050505, 0x50505,
0x5050505, 0x5050505, 0x5050505, 0x50505,
0x5050505, 0x5050505, 0x5050505, 0x50505,
0x7070707, 0x7070707, 0x7070707, 0x70707,
0x0, 0x0, 0x0, 0x0
);

// Additional fontref and draw_text functions for medievalish_font
int fontRefMedieval( uint char, ivec2 offset ) {
    // 8x8 font checks
    bool offsetOOB = any( lessThan(         offset, ivec2( 0,  0 ) ) ) ||
                     any( greaterThanEqual( offset, ivec2( 8,  8 ) ) );
    bool charOOB = bool( clamp( char, 0u, 255u ) != char );
    if ( offsetOOB || charOOB ) {
        return -1; // oob
    }
    // 8x8 grid, 16 columns
    uvec2 sampleLoc = uvec2( char % 16u, char / 16u ) * uvec2( 8u, 8u ) + uvec2( offset );
    
    // Image width is 128.
    uint totalBitIdx = sampleLoc.x + sampleLoc.y * 128u;
    uint wordIdx = totalBitIdx / 32u;
    uint bitMask = 1u << ( (totalBitIdx % 32u) );
    
    if (wordIdx >= 515u) return 0;
    
    return int( ( medievalish_font[wordIdx] & bitMask ) != 0u );
}

vec4 draw_textMedieval(in vec4 in_color, in vec2 uv_in, in vec2 pos_in, in vec2 resolution_in, in int[128] textData, in int len) {
    ivec2 uv = ivec2( uv_in );
    ivec2 pos = ivec2( pos_in );
    // Check bounds for 8x8 font (width 8*len, height 8)
    if (uv.x < pos.x || uv.y < pos.y|| uv.x >= (pos.x + int(8 * len)) || uv.y >= (pos.y + 8)) {
        return in_color;
    }

    int charIndex = (uv.x - pos.x) / 8;
    // Look up character code
    int charCode = int( textData[ uint(charIndex) ] & 0xFFu );
    
    // Look up pixel. Note 8 height.
    int onGlyph = fontRefMedieval(uint(charCode), ivec2((uv.x - pos.x) % 8, (uv.y - pos.y) % 8) );
    
    vec3 col = vec3( 0.0f );
    switch ( onGlyph ) {
        case -1: col = vec3( 0.1618f ); break; // out of bounds
        case  0: col = vec3( 0.0f );    break; // "off" pixel
        case  1: col = vec3( 0.7f );    break; // "on" pixel
    }

    // Output to screen
    return vec4( col, 1.0f );
}


vec4 fantasy_char(in vec4 in_color, in ivec2 uv, in ivec2 pos, in ivec2 resolution, in int charCode) {
    if (uv.x < pos.x || uv.y < pos.y|| uv.x >= (pos.x + 8) || uv.y >= (pos.y + 16)) {
        return in_color;
    }

    int onGlyph = fontRefMedieval(charCode, ivec2(mod(uv.x - pos.x, 8), mod(uv.y - pos.y, 16)) );
    vec3 col = vec3( 0.0f );
    switch ( onGlyph ) {
        case -1: col = vec3( 0.1618f ); break; // out of bounds
        case  0: col = vec3( 0.0f );    break; // "off" pixel
        case  1: col = vec3( 0.7f );  break; // "on" pixel
    }

    // Output to screen
    return vec4( col, 1.0f );
}


int font8x8[512] = {
    0x6C3C3C00, 0x103810, 0xE0FF00FF, 0x1218103C,
    0xFE7E4200, 0x387C38, 0xC0FF00FF, 0xD6683066,
    0xFEDBA500, 0x187C387C, 0x7CC33CE7, 0x7C987066,
    0xFEDBA500, 0x3CFEFEFE, 0x669966C3, 0x27E85066,
    0x7CFF8100, 0x3CFEFE7C, 0x669966C3, 0xE48E503C,
    0x38C3BD00, 0x186C6C38, 0x66C33CE7, 0x3E861018,
    0x10665A00, 0x101010, 0x3CFF00FF, 0x6BE01C3C,
    0x3C3C00, 0x383800, 0xFF00FF, 0x48600C18,
    0x36188002, 0x18001EFC, 0x1818, 0xFE100000,
    0x363CE00E, 0x3C003756, 0x1830183C, 0x7C102800,
    0x367EF83E, 0x7E000E56, 0xC60187E, 0x7C386C06,
    0x3618FEFE, 0x18001B5C, 0xFEFE5A5A, 0x3838FE06,
    0x3618F83E, 0x7E7E3650, 0xFEFE7E18, 0x387CFEFE,
    0x7EE00E, 0x3C7E1C50, 0xC603C18, 0x107C6CFE,
    0x363C8002, 0x187E3B58, 0x18301818, 0x10FE2800,
    0x180000, 0xFF001E00, 0x0, 0x0,
    0x506C0C00, 0x18388010, 0xC60, 0xC0000000,
    0x506C0C00, 0x186CFC7C, 0x186C1830, 0x60000000,
    0xFC280C00, 0xC384A16, 0x18383018, 0x30000000,
    0x28000C00, 0xDC367C, 0x7EFE3018, 0x18007E00,
    0x7E000C00, 0x76D8D0, 0x18383018, 0xC000000,
    0x14000000, 0x66AC7E, 0x186C1830, 0x6180018,
    0x14000C00, 0xDC6E10, 0xC60, 0x2180018,
    0x0, 0x0, 0x0, 0xC,
    0x3C3C1818, 0x7E387E30, 0x3C3C, 0x3C0C0060,
    0x66661834, 0x660C0638, 0x18186666, 0x66180030,
    0x30661C66, 0x32063E34, 0x1818666E, 0x64307E18,
    0x38301866, 0x303E6636, 0x7C3C, 0x3060000C,
    0x60181866, 0x1866607E, 0x6076, 0x18307E18,
    0x664C182C, 0x18666630, 0x18183066, 0x180030,
    0x3C7E3C18, 0x1C3C3C78, 0x18181C3C, 0x180C0060,
    0x0, 0x0, 0xC000000, 0x0,
    0xB83E387C, 0xB8FEFE3E, 0xE6F83C66, 0x38C2821E,
    0xCC6C3082, 0xCC8C8C6C, 0x66601866, 0x6CC6C60C,
    0x866C38BA, 0x862C2CCC, 0x36601866, 0xC6CEEE0C,
    0x67C68AA, 0x63C3CCC, 0x1E60187E, 0xC6DEFE0C,
    0x6CC7CFA, 0xE62C2CCC, 0x36661866, 0xC6F6D68C,
    0x8CCCC402, 0xCC0C8C6C, 0x66661866, 0x6CE6C6CC,
    0x787EC67C, 0xF81EFE3E, 0xE63C3C66, 0x38C6C6FE,
    0x0, 0x0, 0x0, 0x0,
    0x7C3E387E, 0xC6EEC67E, 0x787EE766, 0x103C06,
    0x66666CCC, 0xC646C65A, 0x18666666, 0x38300C,
    0xE66C6CC, 0xD66CC618, 0x18322C3C, 0x6C3018,
    0x3C3EC67C, 0xD62CC618, 0x18181818, 0xC63030,
    0x7036D60C, 0x7C38C618, 0x184C183C, 0x3060,
    0x66666C0C, 0x6C18C618, 0x18661866, 0x30C0,
    0x3EE6381E, 0x44107C3C, 0x787E3C66, 0x3C80,
    0xE000, 0x0, 0x0, 0x7F000000,
    0xE0018, 0x700070, 0xE30180E, 0x1C,
    0xC0018, 0xD80060, 0xC00000C, 0x18,
    0x7C7C3C30, 0xBC183C6C, 0xCC381C6C, 0x3C6E6E18,
    0x66DC6600, 0x667C6676, 0x6C3018DC, 0x66DCFE18,
    0x6CC7800, 0x1C187E66, 0x3C3018CC, 0x66CCD618,
    0x66DC6600, 0x7C180666, 0x6C3618CC, 0x66CCD618,
    0x3C76DE00, 0xC63C7CDC, 0xEE363CCE, 0x3CCED63C,
    0x0, 0x7C000000, 0x1C0000, 0x0,
    0x0, 0x10, 0x70000000, 0x109C0E18,
    0x0, 0x18, 0x18000000, 0x38721818,
    0x7C765C76, 0xD6CE667C, 0x187EEECE, 0x38001818,
    0xEDC66CC, 0xD64C6618, 0xE32CC6C, 0x6C007000,
    0x3CCC66CC, 0xFE6C6618, 0x18185838, 0x64001818,
    0x700C7C7C, 0x6C386658, 0x184C306C, 0xC6001818,
    0x3E1E600C, 0x4410DC30, 0x707E36E6, 0xFE000E18,
    0xF01E, 0x0, 0x1C00, 0xFE000000,
    0x187036B8, 0x180C6C, 0x360C6C18, 0x386C0C08,
    0x3C1800CC, 0x181800, 0x18003C, 0x2810181C,
    0x3C3C6686, 0x7C3C3C3C, 0x1C3C3C3C, 0x38381010,
    0x66666606, 0x66666666, 0x18666666, 0x78681C1C,
    0x787E6606, 0x6787878, 0x187E7E7E, 0x6C6C1818,
    0x6606668C, 0x6C666666, 0x18060606, 0xFCFC1818,
    0xDE7CDC70, 0x38DEDEDE, 0x3C7C7C7C, 0xC6C63C3C,
    0x38, 0x1C000000, 0x0, 0x0,
    0x18FC00E0, 0xE180E66, 0x606CC66C, 0x701E6678,
    0x3CBC0030, 0x183C1800, 0x20823800, 0xD83666CC,
    0x3C346EFE, 0x66423C3C, 0x3CC66CEE, 0x18363CCC,
    0x667CD88C, 0x66666666, 0x76C6C6CC, 0x7C5E7E3E,
    0x6636FC3C, 0x66666666, 0x16C6C658, 0x1866189C,
    0x66B6368C, 0x66666666, 0x56C66C30, 0x18F67EDA,
    0x3CF6EEFE, 0xDCDC3C3C, 0x3C7C3836, 0x1A66186E,
    0x0, 0x0, 0xC00001C, 0xEC00000,
    0x70703870, 0x38785858, 0x6000018, 0xC06,
    0x18180C18, 0x6C6C3434, 0x66000000, 0x36D80066,
    0x663C183C, 0x6C6CCE6E, 0x36000018, 0x6C6C0C36,
    0x66661C66, 0x38F8DEDC, 0x7E7E7E0C, 0xD8360CDE,
    0x66661878, 0xF6CC, 0xDC600626, 0xD8360CEC,
    0x66661866, 0x7CFCE6CC, 0x66600666, 0x6C6C0CD6,
    0xDC3C3CDE, 0xC6CE, 0x3200003C, 0x36D80CF2,
    0x0, 0x0, 0xF0000000, 0xC0,
    0x1877AA88, 0x2C1818, 0x2C2C00, 0x182C2C,
    0x18DD5522, 0x2C1818, 0x2C2C00, 0x182C2C,
    0x1877AA88, 0x2C1F18, 0x1F2C2F0F, 0x1F2C2F,
    0x18DD5522, 0x1F2F1F1F, 0x3F2C2F1F, 0x1F1F3F2F,
    0x1877AA88, 0x3F2F181F, 0x202C2018, 0x1F183F20,
    0x18DD5522, 0x2C2C1F18, 0x2F2C2F1F, 0x181F003F,
    0x1877AA88, 0x2C2C1818, 0x2C2C2C18, 0x18000000,
    0x18DD5522, 0x2C2C1818, 0x2C2C2C18, 0x18000000,
    0x18001818, 0x2C181800, 0x2C002C, 0x182C002C,
    0x18001818, 0x2C181800, 0x2C002C, 0x182C002C,
    0x18001818, 0x2CF81800, 0xFFEFFCEC, 0xFFEFFFEC,
    0xF8FFFFF8, 0xECF8FFFF, 0xFFEFFCEC, 0xFFEFFFEC,
    0xF8FFFFF8, 0xEC18FFFF, 0xC0C, 0xC,
    0x18180000, 0x2CF81800, 0xEFFFECF8, 0xFFEFFFEC,
    0x18180000, 0x2C181800, 0x2C002C00, 0x2C002C,
    0x18180000, 0x2C181800, 0x2C002C00, 0x2C002C,
    0x2C00002C, 0x2C000018, 0xFF001818, 0xFFF00F00,
    0x2C00002C, 0x2C000018, 0xFF001818, 0xFFF00F00,
    0x2C00FF2C, 0x2C00F8F8, 0xFF0018FF, 0xFFF00F00,
    0xFCFFFFFF, 0xFFFCF8F8, 0xFFF81FFF, 0xFFF00F00,
    0xF8FF00FF, 0xFFFC1818, 0xFFF81F18, 0xF00FFF,
    0x2CFF00, 0x2C2CF8F0, 0xFF1800FF, 0xF00FFF,
    0x2C1800, 0x2C2C1800, 0xFF180018, 0xF00FFF,
    0x2C1800, 0x2C2C1800, 0xFF180018, 0xF00FFF,
    0xFE7000, 0x7F, 0x38383838, 0x3C008000,
    0x7ECCD800, 0xFC000066, 0xC6C6C10, 0x6600C06C,
    0x3F8CCCBC, 0x7ECCFC0C, 0x18C6C67C, 0x66787CFE,
    0x150C7C76, 0xACC3618, 0x34C6FED6, 0x660CE6B2,
    0x140CCC66, 0x18CC360C, 0x666CC67C, 0x667CD69A,
    0x360CCE76, 0x38EC3666, 0x66AA6C10, 0x660CCEFE,
    0x331E76DC, 0x30B61C7F, 0x3CEE3838, 0x66787C6C,
    0x600, 0x60000, 0x0, 0x200,
    0x60061800, 0x181870, 0xC0000038, 0x3C6E,
    0x381C187E, 0xDC1818D8, 0x4000006C, 0x66DC,
    0xE707E00, 0x76001858, 0x6000106C, 0x3C30CC,
    0x381C187E, 0x7E1818, 0x24183838, 0x3C1CCC,
    0x60061800, 0xDC001A18, 0x36183800, 0x3C7ECC,
    0x7E, 0x76181B18, 0x1C001000, 0x3C0000,
    0x7E7E7E00, 0x180E18, 0x18000000, 0x0,
    0x0, 0x18, 0x8000000, 0x0,
};

int fontRef8x8( int char, ivec2 offset ) {
    // 8x8 font checks
    bool offsetOOB = any( lessThan(         offset, ivec2( 0,  0 ) ) ) ||
                     any( greaterThanEqual( offset, ivec2( 8,  8 ) ) );
    bool charOOB = bool( clamp( char, 0, 127 ) != char );
    if ( offsetOOB || charOOB ) {
        return -1; // oob
    }
    // 8x8 grid, 16 columns
    ivec2 sampleLoc = ivec2( char % 16, char / 16 ) * ivec2( 8, 8 ) + ivec2( offset );
    
    // Image width is 128
    int totalBitIdx = sampleLoc.x + sampleLoc.y * 128;
    int wordIdx = totalBitIdx / 32;
    int bitMask = 1 << ( (totalBitIdx % 32) );
    
    if (wordIdx >= 512) return 0;
    
    return int( ( font8x8[wordIdx] & bitMask ) != 0 );
}

vec4 char8x8(in vec4 in_color, in vec2 uv, in ivec2 mn, float scale, in int charCode) {
    float scaledWidth = 8.0f * scale;
    float scaledHeight = 8.0f * scale;
    vec2 pos = vec2(mn) * vec2(scaledWidth, scaledHeight);

    if (uv.x < pos.x || uv.y < pos.y|| uv.x >= (pos.x + scaledWidth) || uv.y >= (pos.y + scaledHeight)) {
        return in_color;
    }

    int bit_w = int((uv.x - pos.x) / scaledWidth * 8.0);
    int bit_h = int((uv.y - pos.y) / scaledHeight * 8.0);
    int onGlyph = fontRef8x8(charCode, ivec2(bit_w, bit_h) );
    vec3 col = vec3( 0.0f );
    switch ( onGlyph ) {
        case -1: col = vec3( 0.1618f ); break; // out of bounds
        case  0: col = vec3( 0.0f );    break; // "off" pixel
        case  1: col = vec3( 0.7f );  break; // "on" pixel
    }

    // Output to screen
    return vec4( col, 1.0f );
}