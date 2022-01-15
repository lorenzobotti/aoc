#include "buffer.h"
#include "utils.h"

#include <string.h>
#include <stdlib.h>
#include <stdio.h>

void buffer_push(Stack* buf, void* contents, unsigned int len) {
    unsigned int new_len = buf->len + len;

    if (new_len > buf->cap) {
        unsigned int new_cap = max(buf->cap * 2, buf->cap + len);

        buf->contents = realloc(buf->contents, new_cap);
        buf->cap = new_cap;
    }

    memcpy(buf->contents + buf->len, contents, len);
    buf->len = new_len;
}

int buffer_pop(Stack* buf, void* into, unsigned int len) {
    if (buf->len < len) {
        return -1;
    }

    unsigned int start = buf->len - len;
    memcpy(into, buf->contents + start, len);
    buf->len = start;
    return 0;
}

int buffer_get(Stack* buf, void* into, unsigned int len, unsigned int i) {
    i *= len;

    if (i + len - 1 >= buf->len) {
        return -1;
    }

    memcpy(into, buf->contents + i, len);
    return 0;
}

unsigned int buffer_count(Stack* buf, unsigned int len) {
    return buf->len / len; 
}

void buffer_free(Stack* buf) {
    free(buf->contents);
}


Stack new_buffer() {
    Stack buf;
    buf.contents = malloc(STARTING_CAP);
    buf.cap = STARTING_CAP;
    buf.len = 0;

    return buf;
}