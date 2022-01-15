#ifndef buffer_h
#define buffer_h

#define STARTING_CAP 29
struct Stack {
    void* contents;

    int cap;
    int len;
};

typedef struct Stack Stack;

Stack new_buffer();

void buffer_push(Stack* buf, void* contents, unsigned int len);
int buffer_pop(Stack* buf, void* into, unsigned int len);
int buffer_get(Stack* buf, void* into, unsigned int len, unsigned int i);
unsigned int buffer_count(Stack* buf, unsigned int len);
void buffer_free(Stack* buf);

#endif //buffer_h