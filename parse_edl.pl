#!/usr/bin/env perl


while (<>) {
    if (/^[0-9]{3,3}/) {
        @f=split;
        @t=split /:/, $f[4];
        shift @t; 
        printf("\"00:%s:%s:%s\",\n", @t);
    }
}
