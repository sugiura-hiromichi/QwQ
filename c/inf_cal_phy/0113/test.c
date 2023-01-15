#include <assert.h>
#include <stdio.h>
/// NOTE: This file is `test.c`

#define SCIENTIFIC_NOTATION 1e5

void t1() { assert(SCIENTIFIC_NOTATION == 100000); }

int main() { t1(); }
