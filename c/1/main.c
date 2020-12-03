#define _DEFAULT_SOURCE
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    FILE *inputFile = NULL;
    char *buffer = malloc(64);
    size_t buffSize = 64;
    int number = 0;
    int *list = malloc(256 * sizeof(int));
    size_t numItems = 0;
    size_t capacity = 256;
    char *endP = NULL;
    size_t i = 0, j = 0, k = 0;
    bool done = false;

    if (argc >= 2) {
        inputFile = fopen(argv[1], "r");
        if (NULL == inputFile) {
            fprintf(stderr, "Unable to read file '%s'\n", argv[1]);
            return -1;
        }
    } else {
        inputFile = fopen("day1.txt", "r");
        if (NULL == inputFile) {
            fprintf(stderr, "Unable to read file: 'day1.txt'\n");
            return -1;
        }
    }

    while (getline(&buffer, &buffSize, inputFile) > 0) {
        number = strtod(buffer, &endP);
        if (endP > buffer) {
            if (numItems >= capacity) {
                list = realloc(list, capacity += 256);
            }
            list[numItems++] = number;
        }
    }

    for (i = 0; i < numItems; i++) {
        for (j = i + 1; j < numItems; j++) {
            if (2020 == list[i] + list[j]) {
                printf("%d * %d = %d\n", list[i], list[j], list[i] * list[j]);
                done = true;
                break;
            }
            if (done)
                break;
        }
    }
    done = false;
    for (i = 0; i < numItems; i++) {
        for (j = i + 1; j < numItems; j++) {
            for (k = j + 1; k < numItems; k++) {
                if (2020 == list[i] + list[j] + list[k]) {
                    printf("%d * %d * %d = %d\n", list[i], list[j], list[k],
                           list[i] * list[j] * list[k]);
                    done = true;
                    break;
                }
            }
            if (done)
                break;
        }
        if (done)
            break;
    }

    free(buffer);
    free(list);
    fclose(inputFile);
    return 0;
}
