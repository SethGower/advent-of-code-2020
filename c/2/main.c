#define _DEFAULT_SOURCE
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

size_t freq(const char *string, const char c);
int main(int argc, char *argv[]) {
    FILE *inputFile = NULL;
    char *buffer = malloc(64);
    size_t buffSize = 64;
    char *endP = NULL;
    char *tok = NULL;
    size_t bounds[2] = {0, 0};
    char letter = 0;
    size_t frequency;
    int validPart1 = 0;
    int validPart2 = 0;

    if (argc >= 2) {
        inputFile = fopen(argv[1], "r");
        if (NULL == inputFile) {
            fprintf(stderr, "Unable to read file '%s'\n", argv[1]);
            return -1;
        }
    } else {
        inputFile = fopen("day02.txt", "r");
        if (NULL == inputFile) {
            fprintf(stderr, "Unable to read file: 'day02.txt'\n");
            return -1;
        }
    }

    while (getline(&buffer, &buffSize, inputFile) > 0) {
        tok = strtok(buffer, "-");
        if (NULL == tok)
            continue;
        bounds[0] = strtod(tok, &endP);
        tok = strtok(NULL, " ");
        bounds[1] = strtod(tok, &endP);
        tok = strtok(NULL, ":");
        letter = tok[0];
        tok = strtok(NULL, " ");
        frequency = freq(tok, letter);
        if (frequency <= bounds[1] && frequency >= bounds[0])
            validPart1++;
        if ((tok[bounds[0] - 1] == letter) != (tok[bounds[1] - 1] == letter))
            validPart2++;
    }
    printf("Valid Passwords (part1): %d\n", validPart1);
    printf("Valid Passwords (part2): %d\n", validPart2);

    free(buffer);
    fclose(inputFile);
    return 0;
}
size_t freq(const char *string, const char c) {
    size_t i = 0;
    size_t num = 0;

    for (i = 0; i < strlen(string); i++) {
        if (c == string[i])
            num++;
    }

    return num;
}
