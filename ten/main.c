#include "buffer.h"
#include "lines.h"
#include "parse.h"
#include "utils.h"
#include <string.h>
#include <stdio.h>

int main(int argc, char** argv) {
    Stack input = read_all_stdin();
    Stack lines = separate_lines(&input);
    unsigned int lines_count = buffer_count(&lines, sizeof(StrSlice));
    

    Stack scores = new_buffer();

    for (int lineI = 0; lineI < lines_count; lineI++) {
        StrSlice line;
        buffer_get(&lines, &line, sizeof(StrSlice), lineI);

        Stack parenthesis = new_buffer();
        
        for (int i = 0; i < line.len; i++) {
            char ch = ((char*)line.contents)[i];
            if (is_open(ch)) { // push open parenthesis to the stack
                buffer_push(&parenthesis, &ch, sizeof(char));
            } else if (is_close(ch)) { // pop closed parenthesis and check it's correct
                char equivalent;
                buffer_pop(&parenthesis, &equivalent, sizeof(char));
                char expected = opposite(equivalent);

                if (ch != expected) {
                    // int award = points(ch);
                    // score += award;
                    // printf("wrong parenthesis: %c, expected %c, here's %d points\n", ch, expected, award);
                    goto drop; // skip line if incorrect
                }
            }
        }

        if (parenthesis.len == 0) {
            goto drop;
        }

        // if we are here the line was incomplete. invert it and calculate the score
        invert_str(parenthesis.contents, parenthesis.len);
        invert_parenthesis_str(parenthesis.contents, parenthesis.len);

        unsigned long points = completion_points(parenthesis.contents, parenthesis.len);
        printf("%s: %ld points\n", (char*)parenthesis.contents, points);
        
        buffer_push(&scores, &points, sizeof(unsigned long));
    drop:
        buffer_free(&parenthesis);
    }

    unsigned long* actual_scores = (unsigned long*)scores.contents;
    int len = buffer_count(&scores, sizeof(unsigned long));

    for (int i = 0; i < len; i++) {
        for (int j = 0; j < len - 1; j++) {
            if (actual_scores[j] > actual_scores[j+1]) {
                int buf = actual_scores[j];
                actual_scores[j] = actual_scores[j+1];
                actual_scores[j+1] = buf;
            }
        }
    }

    for (int i = 0; i < len; i++) {
        printf("%d: %ld\n", i, actual_scores[i]);
    }

    int middle = len/2;
    printf("solution: %ld\n", actual_scores[middle]);

    return 0;
}