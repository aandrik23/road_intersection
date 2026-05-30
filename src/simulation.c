#include "simulation.h"

#include <string.h>

void simulation_init(Simulation *sim)
{
    memset(sim, 0, sizeof(*sim));
    world_init(&sim->world);
    for (int i = 0; i < LANE_COUNT; i++) {
        sim->lane_signals[i] = SIGNAL_RED;
    }
}

void simulation_shutdown(Simulation *sim)
{
    (void)sim;
}

int get_lane_queue_count(const Simulation *sim, LaneId lane)
{
    (void)sim;
    (void)lane;
    return 0;
}

void simulation_set_lane_signal(Simulation *sim, LaneId lane, SignalState state)
{
    if (lane >= 0 && lane < LANE_COUNT) {
        sim->lane_signals[lane] = state;
    }
}

SignalState simulation_lane_signal(const Simulation *sim, LaneId lane)
{
    if (lane < 0 || lane >= LANE_COUNT) {
        return SIGNAL_RED;
    }
    return sim->lane_signals[lane];
}
