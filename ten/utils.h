#ifndef utils_h
#define utils_h

#include "buffer.h"

int min(int a, int b);
int max(int a, int b);
void invert_str(char* str, unsigned int len);

Stack read_all_stdin();

#endif //utils_h