#ifndef LANG_H
#define LANG_H

#include <stdlib.h>
#include <wchar.h>
#include <string.h>
#include <stdio.h>

struct lang
{
    wchar_t* consonants;
    size_t consonantsSize;
};

void read_consonants(struct lang* lang);

#endif
