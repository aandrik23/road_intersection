#ifndef WORLD_H
#define WORLD_H

#include "lane.h"

typedef struct {
    LaneInfo lanes[LANE_COUNT];
    RoutePath routes[LANE_COUNT][ROUTE_COUNT];
} World;

void world_init(World *world);

const LaneInfo *world_lane(const World *world, LaneId lane);
const RoutePath *world_route(const World *world, LaneId lane, RouteType route);

LaneId world_lane_for_spawn_direction(int direction_index);

int world_lane_capacity(const LaneInfo *lane);

#endif
