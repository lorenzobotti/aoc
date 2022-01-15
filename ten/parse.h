#ifndef parse_h
#define parse_h

#define OPEN_ONE '('
#define OPEN_TWO '['
#define OPEN_THREE '{'
#define OPEN_FOUR '<'

#define CLOSE_ONE ')'
#define CLOSE_TWO ']'
#define CLOSE_THREE '}'
#define CLOSE_FOUR '>'

char opposite(char of);
void invert_parenthesis_str(char* str, unsigned int len);

int points(char ch);
unsigned long completion_points(char* ch, unsigned int len);

int is_open(char ch);
int is_close(char ch);

#endif //parse_h