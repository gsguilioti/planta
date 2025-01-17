#include "lang.h"

struct lang* init_lang()
{
    struct lang* lang = malloc(sizeof(struct lang));
    lang->consonants = NULL;
    lang->consonantsSize = 0;
    return lang;
}

void free_lang(struct lang* lang)
{
    free(lang->consonants);
    free(lang->vowels);
    free(lang->syllableStructure);
    free(lang);
}

void read_consonants(struct lang* lang)
{
    wchar_t consonant;
    wprintf(L"Enter the consonants: ");

    while((consonant = getwchar()) != L'\n' && consonant != WEOF)
    {
        lang->consonants = (wchar_t*)realloc(lang->consonants, (lang->consonantsSize + 1) * sizeof(wchar_t));

        if (lang->consonants == NULL) 
        {
            wprintf(L"error reading consonants.\n");
            exit(1);
        }

        lang->consonants[lang->consonantsSize] = consonant;
        lang->consonantsSize += 1;
    }
}

void read_vowels(struct lang* lang)
{
    wchar_t vowel;
    wprintf(L"Enter the vowels: ");

    while((vowel = getwchar()) != L'\n' && vowel != WEOF)
    {
        lang->vowels = (wchar_t*)realloc(lang->vowels, (lang->vowelsSize + 1) * sizeof(wchar_t));

        if (lang->vowels == NULL) 
        {
            wprintf(L"error reading vowels.\n");
            exit(1);
        }

        lang->vowels[lang->consonantsSize] = vowel;
        lang->vowelsSize += 1;
    }
}

void read_syllable_structure(struct lang* lang)
{
    wchar_t letter;
    wprintf(L"Enter the syllable structure(c/C, v/V) *capitals are optional: ");

    while((letter = getwchar()) != L'\n' && letter != WEOF)
    {
        lang->syllableStructure = (wchar_t*)realloc(lang->syllableStructure, (lang->syllableStructureSize + 1) * sizeof(wchar_t));

        if (lang->syllableStructure == NULL) 
        {
            wprintf(L"error reading syllable structure.\n");
            exit(1);
        }

        lang->syllableStructure[lang->syllableStructureSize] = letter;
        lang->syllableStructureSize += 1;
    }
}
