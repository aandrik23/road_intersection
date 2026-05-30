#ifndef SIMULATION_H
#define SIMULATION_H

#include "world.h"
#include <SDL.h>

typedef struct {
    float x;
    float y;
    float angle_deg;
    float width;
    float height;
    uint8_t r;
    uint8_t g;
    uint8_t b;
} VehicleDraw;

typedef struct {
    World world;
    SignalState lane_signals[LANE_COUNT];
    VehicleDraw vehicles[MAX_VEHICLES];
    int vehicle_count;
} Simulation;

void simulation_init(Simulation *sim);
void simulation_shutdown(Simulation *sim);

int get_lane_queue_count(const Simulation *sim, LaneId lane);

void simulation_set_lane_signal(Simulation *sim, LaneId lane, SignalState state);
SignalState simulation_lane_signal(const Simulation *sim, LaneId lane);

#endif
