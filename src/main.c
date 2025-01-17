#include <stdlib.h>
#include <stdio.h>
#include <locale.h>

#include "lang.h"

int main()
{
    setlocale(LC_ALL, "");
    struct lang* lang = init_lang();

    read_consonants(lang);
    wprintf(L"Consonants entered: %ls\n\n", lang->consonants);

    read_vowels(lang);
    wprintf(L"Vowels entered: %ls\n\n", lang->vowels);

    read_syllable_structure(lang);
    wprintf(L"Structure entered: %ls\n\n", lang->syllableStructure);

    free_lang(lang);
    return 0;
}
