CC=gcc
VAL=valgrind --leak-check=full --track-origins=yes 
CFLAGS=-std=c99 -g -pedantic -Wall -Wextra
OBJECT_DIR=build
OUTPUT_DIR=out

_OBJECTS = main.o
OBJECTS = $(patsubst %,$(OBJECT_DIR)/%,$(_OBJECTS))

OUTPUT = $(OUTPUT_DIR)/output
INPUT = ../../inputs/day01.txt

$(OBJECT_DIR)/%.o: %.c
	$(CC) -c -o $@ $< $(CFLAGS)

# all: Generate output, using compiled .obj files
all: $(OUTPUT)

$(OUTPUT): $(OBJECTS)
	$(CC) -o $(OUTPUT) $^ $(CFLAGS) $(LIBS)

.phony: run all meme mem clean
run: all
	$(OUTPUT) $(INPUT)

mem: all
	$(VAL) $(OUTPUT) $(INPUT)

meme: mem

clean:
	rm -rf $(OUTPUT_DIR)/* $(OBJECT_DIR)/*
