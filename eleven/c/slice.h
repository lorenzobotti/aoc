#ifndef slice_h
#define slice_h

#define SLICE_START 127

struct Slice {
    unsigned char* contents;
    unsigned int len;
    unsigned int cap;
};

typedef struct Slice Slice;

Slice new_slice();

void slice_push(Slice* s, unsigned char* src, unsigned int size);
int slice_get(Slice* s, unsigned char* dest, unsigned int size, unsigned int i);
int slice_pop(Slice* s, unsigned char* dest, unsigned int size);

#endif //slice_h