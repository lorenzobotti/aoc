#include "slice.h"
#include "utils.h"
#include <stdlib.h>
#include <string.h>

Slice new_slice() {
    Slice out;
    out.contents = malloc(SLICE_START);
    out.cap = SLICE_START;
    out.len = 0;

    return out;
}

void slice_push(Slice* s, unsigned char* src, unsigned int size) {
    unsigned int new_len = s->len + size;

    if (new_len > s->cap) {
        s->contents = realloc(s->contents, min(s->cap, new_len));
    }

    memcpy(s->contents + s->len, src, size);
}

int slice_get(Slice* s, unsigned char* dest, unsigned int size, unsigned int i) {
    if (i > s->len) {
        return -1;
    }

    memcpy(dest, s->contents + size * i, size);
    return 0;
}

int slice_pop(Slice* s, unsigned char* dest, unsigned int size) {
    if (s->len <= 0) {
        return -1;
    } 


    memcpy(dest, s->contents + (s->len - 1) * size, size);
    return 0;
}
