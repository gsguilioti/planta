#include "lang.h"

void read_consonants(struct lang* lang)
{
    wchar_t consonants[315];
    wprintf(L"Enter the consonants: ");
    fgetws(consonants, sizeof(consonants)/sizeof(consonants[0]), stdin);

    size_t len = wcslen(consonants);
    if (len > 0 && consonants[len - 1] == L'\n') {
        consonants[len - 1] = L'\0';
    }

    lang->consonantsSize = wcslen(consonants) + 1;
    lang->consonants = (wchar_t*)malloc(lang->consonantsSize * sizeof(wchar_t));
    
    if (lang->consonants == NULL) {
        wprintf(L"error reading consonants.\n");
        exit(1);
    }

    wcscpy(lang->consonants, consonants);
}

void read_vowels(struct lang* lang)
{
    wchar_t vowels[56];
    wprintf(L"Enter the vowels: ");
    fgetws(vowels, sizeof(vowels)/sizeof(vowels[0]), stdin);

    size_t len = wcslen(vowels);
    if (len > 0 && vowels[len - 1] == L'\n') {
        vowels[len - 1] = L'\0';
    }

    lang->vowelsSize = wcslen(vowels) + 1;
    lang->vowels = (wchar_t*)malloc(lang->vowelsSize * sizeof(wchar_t));
    
    if (lang->vowels == NULL) {
        wprintf(L"error reading vowels.\n");
        exit(1);
    }

    wcscpy(lang->vowels, vowels);
}
