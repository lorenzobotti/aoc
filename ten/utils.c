#include "utils.h"
#include "buffer.h"
#include <stdio.h>

int min(int a, int b) {
    if (a < b)
        return a;
    else
        return b;
}
int max(int a, int b) {
    if (a > b)
        return a;
    else
        return b;
}

Stack read_all_stdin() {
    Stack buf = new_buffer();

    for ( ;; ) {
        char ch = getchar();
        if (ch == 0 || ch == -1) break;

        buffer_push(&buf, &ch, sizeof(char));
    }

    return buf;
}

void invert_str(char* str, unsigned int len) {
    for (int i = 0; i < len/2; i++) {
        int opposite = len - i - 1;
        char buf = str[i];
        str[i] = str[opposite]; 
        str[opposite] = buf; 
    }
}