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

    wchar_t* vowels;
    size_t vowelsSize;

    wchar_t* syllableStructure;
    size_t syllableStructureSize;
};

struct lang* init_lang();
void free_lang(struct lang*);
void read_consonants(struct lang* lang);
void read_vowels(struct lang* lang);
void read_syllable_structure(struct lang* lang);

#endif
