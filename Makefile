CC      ?= cc
CFLAGS  ?= -std=c11 -Wall -Wextra -Wpedantic -O2
LDFLAGS ?=

SDL_CFLAGS := $(shell sdl2-config --cflags 2>/dev/null)
SDL_LIBS   := $(shell sdl2-config --libs 2>/dev/null)

ifeq ($(SDL_LIBS),)
  SDL_CFLAGS := -I/opt/homebrew/include/SDL2 -D_THREAD_SAFE
  SDL_LIBS   := -L/opt/homebrew/lib -lSDL2
endif

INCLUDES = -Iinclude $(SDL_CFLAGS)
SRC      = src/main.c src/world.c src/simulation.c src/renderer.c
OBJ      = $(SRC:src/%.c=build/%.o)
TARGET   = road_intersection

.PHONY: all clean run

all: $(TARGET)

build:
	mkdir -p build

build/%.o: src/%.c | build
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

$(TARGET): $(OBJ)
	$(CC) $(CFLAGS) -o $@ $(OBJ) $(LDFLAGS) $(SDL_LIBS)

run: $(TARGET)
	./$(TARGET)

clean:
	rm -rf build $(TARGET)
