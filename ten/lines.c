#include "lines.h"
#include "buffer.h"
#include <stdlib.h>

Stack separate_lines(Stack* buf) {
    Stack out = new_buffer();

    int last_line = 0;
    for (int i = 0; i < buf->len; i++) {
        char ch = ((char*)buf->contents)[i];

        if (ch == '\n' || i == buf->len - 1) {
            StrSlice line;

            line.contents = buf->contents + last_line;
            line.len = i - last_line;
            last_line = i;

            buffer_push(&out, &line, sizeof(StrSlice));
        }
    }

    return out;
}