#ifndef LANE_H
#define LANE_H

#include "sim_config.h"
#include "types.h"

typedef struct {
    LaneId id;
    const char *name;
    Vec2 spawn;
    Vec2 stop_line;
    Vec2 light_pos;
    float heading;
    float lane_length;
    int inbound;
} LaneInfo;

typedef struct {
    LaneId lane;
    RouteType route;
    int count;
    Vec2 waypoints[MAX_ROUTE_WAYPOINTS];
    float path_length;
} RoutePath;

#endif
