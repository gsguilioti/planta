#include <stdlib.h>
#include <stdio.h>
#include <locale.h>

#include "lang.h"

int main()
{
    setlocale(LC_ALL, "");
    struct lang* lang = (struct lang*)malloc(sizeof(struct lang));

    read_consonants(lang);
    wprintf(L"Consonants entered: %ls\n", lang->consonants);
    return 0;
}
