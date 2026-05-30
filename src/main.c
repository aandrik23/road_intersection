#include "renderer.h"
#include "simulation.h"
#include "world.h"

#include <stdio.h>

static void print_lane_table(const Simulation *sim)
{
    printf("\n--- Lane data (Person 1) ---\n");
    printf("capacity = floor(lane_length / (vehicle_length + safety_gap))\n");
    printf("vehicle_length=%.0f safety_gap=%.0f slot=%.0f\n\n",
           VEHICLE_LENGTH,
           SAFETY_GAP,
           VEHICLE_LENGTH + SAFETY_GAP);
    for (int i = 0; i < LANE_COUNT; i++) {
        const LaneInfo *lane = &sim->world.lanes[i];
        printf("%-8s spawn(%6.0f,%6.0f) stop(%6.0f,%6.0f) len=%6.1f cap=%d inbound=%d\n",
               lane->name,
               lane->spawn.x,
               lane->spawn.y,
               lane->stop_line.x,
               lane->stop_line.y,
               lane->lane_length,
               world_lane_capacity(lane),
               lane->inbound);
    }
    printf("\nRoute colors: LEFT=blue RIGHT=yellow STRAIGHT=green\n");
    printf("Controls (Person 3): arrows spawn, r random, Esc quit\n");
    printf("Queue hook get_lane_queue_count() -> stub 0\n\n");
}

int main(int argc, char **argv)
{
    (void)argc;
    (void)argv;

    Simulation sim;
    AppRenderer app = {0};

    simulation_init(&sim);
    print_lane_table(&sim);

    if (renderer_init(&app, "Road Intersection — Person 1 (Environment)") != 0) {
        return 1;
    }

    int running = 1;
    Uint32 last_ticks = SDL_GetTicks();

    while (running) {
        SDL_Event event;
        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_QUIT) {
                running = 0;
            } else if (event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_ESCAPE) {
                running = 0;
            }
        }

        Uint32 now = SDL_GetTicks();
        float dt = (now - last_ticks) / 1000.0f;
        last_ticks = now;
        (void)dt;

        renderer_draw_frame(&app, &sim);
        SDL_Delay(16);
    }

    renderer_destroy(&app);
    simulation_shutdown(&sim);
    return 0;
}
