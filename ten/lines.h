#ifndef lines_h
#define lines_h

#include "buffer.h"

struct StrSlice {
    char* contents;
    unsigned int len;
};
typedef struct StrSlice StrSlice;

Stack separate_lines(Stack* buf);


#endif //lines_h