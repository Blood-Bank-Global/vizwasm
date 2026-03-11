#!/usr/bin/env perl


while (<>) {
    if (/^[0-9]{3,3}/) {
        @f=split;
        @t=split /:/, $f[4];
        shift @t; 
        printf("time_code_2_float(\"00:%s:%s:%s\"),\n", @t);
    }
}
