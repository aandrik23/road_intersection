#ifndef RENDERER_H
#define RENDERER_H

#include "simulation.h"

typedef struct {
    SDL_Window *window;
    SDL_Renderer *renderer;
} AppRenderer;

int renderer_init(AppRenderer *app, const char *title);
void renderer_destroy(AppRenderer *app);

void renderer_draw_frame(AppRenderer *app, const Simulation *sim);

#endif
