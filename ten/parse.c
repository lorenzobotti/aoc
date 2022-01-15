#include "parse.h"

char opposite(char of) {
    switch (of) {
    case OPEN_ONE:
        return CLOSE_ONE;
    case OPEN_TWO:
        return CLOSE_TWO;
    case OPEN_THREE:
        return CLOSE_THREE;
    case OPEN_FOUR:
        return CLOSE_FOUR;
    case CLOSE_ONE:
        return OPEN_ONE;
    case CLOSE_TWO:
        return OPEN_TWO;
    case CLOSE_THREE:
        return OPEN_THREE;
    case CLOSE_FOUR:
        return OPEN_FOUR;
    
    default:
        return 0;
    }
}

void invert_parenthesis_str(char* str, unsigned int len) {
    for (int i = 0; i < len; i++) {
        str[i] = opposite(str[i]);
    }
}

int points(char ch) {
    switch (ch) {
    case CLOSE_ONE:
        return 3;
    case CLOSE_TWO:
        return 57;
    case CLOSE_THREE:
        return 1197;
    case CLOSE_FOUR:
        return 25137;
    default:
        return 0;
    }
}

int completion_points_ch(char ch) {
    switch (ch) {
    case CLOSE_ONE:
        return 1;
    case CLOSE_TWO:
        return 2;
    case CLOSE_THREE:
        return 3;
    case CLOSE_FOUR:
        return 4;
    case OPEN_ONE:
        return 1;
    case OPEN_TWO:
        return 2;
    case OPEN_THREE:
        return 3;
    case OPEN_FOUR:
        return 4;
    default:
        return 0;
    }
}

unsigned long completion_points(char *line, unsigned int len) {
    unsigned long score = 0;

    for (int i = 0; i < len; i++) {
        char ch = line[i];
        score = (score * 5) + completion_points_ch(ch);
    }

    return score;
}

int is_open(char ch) {
    if (ch == OPEN_ONE || ch == OPEN_TWO || ch == OPEN_THREE || ch == OPEN_FOUR) {
        return 1;
    } else {
        return 0;
    }
}

int is_close(char ch) {
    if (ch == CLOSE_ONE || ch == CLOSE_TWO || ch == CLOSE_THREE || ch == CLOSE_FOUR) {
        return 1;
    } else {
        return 0;
    }
}
