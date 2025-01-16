#include "lang.h"

void read_consonants(struct lang* lang)
{
    wchar_t input[315];
    wprintf(L"Enter the consonants: \n");
    fgetws(input, sizeof(input)/sizeof(input[0]), stdin);

    size_t len = wcslen(input);
    if (len > 0 && input[len - 1] == L'\n') {
        input[len - 1] = L'\0';
    }

    lang->consonantsSize = wcslen(input) + 1;
    lang->consonants = (wchar_t*)malloc(lang->consonantsSize * sizeof(wchar_t));
    
    if (lang->consonants == NULL) {
        wprintf(L"error reading consonants.\n");
        exit(1);
    }

    wcscpy(lang->consonants, input);
}
