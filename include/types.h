#ifndef TYPES_H
#define TYPES_H

#include <stdint.h>

typedef struct {
    float x;
    float y;
} Vec2;

typedef enum {
    ROUTE_LEFT = 0,
    ROUTE_RIGHT,
    ROUTE_STRAIGHT,
    ROUTE_COUNT
} RouteType;

/* Eight lanes: each road arm × two directions (see docs/LANE_DATA.md). */
typedef enum {
    LANE_NORTH_SB = 0,
    LANE_NORTH_NB,
    LANE_SOUTH_NB,
    LANE_SOUTH_SB,
    LANE_EAST_WB,
    LANE_EAST_EB,
    LANE_WEST_EB,
    LANE_WEST_WB,
    LANE_COUNT = 8
} LaneId;

typedef struct {
    uint8_t r;
    uint8_t g;
    uint8_t b;
} ColorRGB;

static inline ColorRGB route_color(RouteType route)
{
    switch (route) {
    case ROUTE_LEFT:
        return (ColorRGB){60, 140, 255};
    case ROUTE_RIGHT:
        return (ColorRGB){255, 210, 40};
    case ROUTE_STRAIGHT:
    default:
        return (ColorRGB){50, 200, 90};
    }
}

typedef enum {
    SIGNAL_RED = 0,
    SIGNAL_GREEN
} SignalState;

#endif
