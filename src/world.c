#include "world.h"

#include <math.h>
#include <string.h>

static float vec_dist(Vec2 a, Vec2 b)
{
    float dx = b.x - a.x;
    float dy = b.y - a.y;
    return sqrtf(dx * dx + dy * dy);
}

static void path_push(RoutePath *path, Vec2 point)
{
    if (path->count >= MAX_ROUTE_WAYPOINTS) {
        return;
    }
    if (path->count > 0) {
        path->path_length += vec_dist(path->waypoints[path->count - 1], point);
    }
    path->waypoints[path->count++] = point;
}

static void path_reset(RoutePath *path, LaneId lane, RouteType route)
{
    memset(path, 0, sizeof(*path));
    path->lane = lane;
    path->route = route;
}

static void build_paths_for_lane(World *world, LaneId lane)
{
    const float cx = CENTER_X;
    const float cy = CENTER_Y;
    const float h = INTERSECTION_HALF;
    const float lw = LANE_WIDTH;
    const float arm = ARM_LENGTH;

    const float n_sb_x = cx + lw * 0.5f;
    const float n_nb_x = cx - lw * 0.5f;
    const float s_nb_x = cx - lw * 0.5f;
    const float s_sb_x = cx + lw * 0.5f;
    const float e_wb_y = cy - lw * 0.5f;
    const float e_eb_y = cy + lw * 0.5f;
    const float w_eb_y = cy + lw * 0.5f;
    const float w_wb_y = cy - lw * 0.5f;

    RoutePath *left = &world->routes[lane][ROUTE_LEFT];
    RoutePath *right = &world->routes[lane][ROUTE_RIGHT];
    RoutePath *straight = &world->routes[lane][ROUTE_STRAIGHT];
    path_reset(left, lane, ROUTE_LEFT);
    path_reset(right, lane, ROUTE_RIGHT);
    path_reset(straight, lane, ROUTE_STRAIGHT);

    switch (lane) {
    case LANE_NORTH_SB:
        path_push(straight, (Vec2){n_sb_x, cy - h});
        path_push(straight, (Vec2){n_sb_x, cy + h});
        path_push(straight, (Vec2){n_sb_x, cy + h + arm});

        path_push(left, (Vec2){n_sb_x, cy - h});
        path_push(left, (Vec2){cx + h, cy - h});
        path_push(left, (Vec2){cx + h, e_eb_y});
        path_push(left, (Vec2){cx + h + arm, e_eb_y});

        path_push(right, (Vec2){n_sb_x, cy - h});
        path_push(right, (Vec2){cx - h, cy - h});
        path_push(right, (Vec2){cx - h, w_wb_y});
        path_push(right, (Vec2){cx - h - arm, w_wb_y});
        break;

    case LANE_SOUTH_NB:
        path_push(straight, (Vec2){s_nb_x, cy + h});
        path_push(straight, (Vec2){s_nb_x, cy - h});
        path_push(straight, (Vec2){s_nb_x, cy - h - arm});

        path_push(left, (Vec2){s_nb_x, cy + h});
        path_push(left, (Vec2){cx - h, cy + h});
        path_push(left, (Vec2){cx - h, w_wb_y});
        path_push(left, (Vec2){cx - h - arm, w_wb_y});

        path_push(right, (Vec2){s_nb_x, cy + h});
        path_push(right, (Vec2){cx + h, cy + h});
        path_push(right, (Vec2){cx + h, e_eb_y});
        path_push(right, (Vec2){cx + h + arm, e_eb_y});
        break;

    case LANE_WEST_EB:
        path_push(straight, (Vec2){cx - h, w_eb_y});
        path_push(straight, (Vec2){cx + h, w_eb_y});
        path_push(straight, (Vec2){cx + h + arm, w_eb_y});

        path_push(left, (Vec2){cx - h, w_eb_y});
        path_push(left, (Vec2){cx - h, cy - h});
        path_push(left, (Vec2){n_nb_x, cy - h});
        path_push(left, (Vec2){n_nb_x, cy - h - arm});

        path_push(right, (Vec2){cx - h, w_eb_y});
        path_push(right, (Vec2){cx - h, cy + h});
        path_push(right, (Vec2){s_sb_x, cy + h});
        path_push(right, (Vec2){s_sb_x, cy + h + arm});
        break;

    case LANE_EAST_WB:
        path_push(straight, (Vec2){cx + h, e_wb_y});
        path_push(straight, (Vec2){cx - h, e_wb_y});
        path_push(straight, (Vec2){cx - h - arm, e_wb_y});

        path_push(left, (Vec2){cx + h, e_wb_y});
        path_push(left, (Vec2){cx + h, cy + h});
        path_push(left, (Vec2){s_sb_x, cy + h});
        path_push(left, (Vec2){s_sb_x, cy + h + arm});

        path_push(right, (Vec2){cx + h, e_wb_y});
        path_push(right, (Vec2){cx + h, cy - h});
        path_push(right, (Vec2){n_nb_x, cy - h});
        path_push(right, (Vec2){n_nb_x, cy - h - arm});
        break;

    case LANE_NORTH_NB:
        path_push(straight, (Vec2){n_nb_x, cy - h - arm * 0.35f});
        path_push(straight, (Vec2){n_nb_x, cy - h - arm});
        path_push(left, (Vec2){n_nb_x, cy - h});
        path_push(left, (Vec2){cx - h, cy - h});
        path_push(left, (Vec2){cx - h, w_wb_y});
        path_push(right, (Vec2){n_nb_x, cy - h});
        path_push(right, (Vec2){cx + h, cy - h});
        path_push(right, (Vec2){cx + h, e_eb_y});
        break;

    case LANE_SOUTH_SB:
        path_push(straight, (Vec2){s_sb_x, cy + h + arm * 0.35f});
        path_push(straight, (Vec2){s_sb_x, cy + h + arm});
        path_push(left, (Vec2){s_sb_x, cy + h});
        path_push(left, (Vec2){cx + h, cy + h});
        path_push(left, (Vec2){cx + h, e_eb_y});
        path_push(right, (Vec2){s_sb_x, cy + h});
        path_push(right, (Vec2){cx - h, cy + h});
        path_push(right, (Vec2){cx - h, w_wb_y});
        break;

    case LANE_EAST_EB:
        path_push(straight, (Vec2){cx + h + arm * 0.35f, e_eb_y});
        path_push(straight, (Vec2){cx + h + arm, e_eb_y});
        path_push(left, (Vec2){cx + h, e_eb_y});
        path_push(left, (Vec2){cx + h, cy + h});
        path_push(left, (Vec2){s_sb_x, cy + h});
        path_push(right, (Vec2){cx + h, e_eb_y});
        path_push(right, (Vec2){cx + h, cy - h});
        path_push(right, (Vec2){n_nb_x, cy - h});
        break;

    case LANE_WEST_WB:
        path_push(straight, (Vec2){cx - h - arm * 0.35f, w_wb_y});
        path_push(straight, (Vec2){cx - h - arm, w_wb_y});
        path_push(left, (Vec2){cx - h, w_wb_y});
        path_push(left, (Vec2){cx - h, cy - h});
        path_push(left, (Vec2){n_nb_x, cy - h});
        path_push(right, (Vec2){cx - h, w_wb_y});
        path_push(right, (Vec2){cx - h, cy + h});
        path_push(right, (Vec2){s_sb_x, cy + h});
        break;

    default:
        break;
    }
}

static void setup_lane(World *world, LaneId id, const char *name, Vec2 spawn, Vec2 stop,
                       Vec2 light, float heading, int inbound)
{
    LaneInfo *lane = &world->lanes[id];
    lane->id = id;
    lane->name = name;
    lane->spawn = spawn;
    lane->stop_line = stop;
    lane->light_pos = light;
    lane->heading = heading;
    lane->inbound = inbound;
    lane->lane_length = vec_dist(spawn, stop);
}

void world_init(World *world)
{
    const float cx = CENTER_X;
    const float cy = CENTER_Y;
    const float h = INTERSECTION_HALF;
    const float lw = LANE_WIDTH;
    const float arm = ARM_LENGTH;

    memset(world, 0, sizeof(*world));

    setup_lane(world, LANE_NORTH_SB, "N_SB",
               (Vec2){cx + lw * 0.5f, cy - h - arm},
               (Vec2){cx + lw * 0.5f, cy - h},
               (Vec2){cx + lw * 0.5f, cy - h - 18.0f}, 90.0f, 1);

    setup_lane(world, LANE_NORTH_NB, "N_NB",
               (Vec2){cx - lw * 0.5f, cy - h - arm},
               (Vec2){cx - lw * 0.5f, cy - h},
               (Vec2){cx - lw * 0.5f, cy - h - 18.0f}, 270.0f, 0);

    setup_lane(world, LANE_SOUTH_NB, "S_NB",
               (Vec2){cx - lw * 0.5f, cy + h + arm},
               (Vec2){cx - lw * 0.5f, cy + h},
               (Vec2){cx - lw * 0.5f, cy + h + 18.0f}, 270.0f, 1);

    setup_lane(world, LANE_SOUTH_SB, "S_SB",
               (Vec2){cx + lw * 0.5f, cy + h + arm},
               (Vec2){cx + lw * 0.5f, cy + h},
               (Vec2){cx + lw * 0.5f, cy + h + 18.0f}, 90.0f, 0);

    setup_lane(world, LANE_EAST_WB, "E_WB",
               (Vec2){cx + h + arm, cy - lw * 0.5f},
               (Vec2){cx + h, cy - lw * 0.5f},
               (Vec2){cx + h + 18.0f, cy - lw * 0.5f}, 180.0f, 1);

    setup_lane(world, LANE_EAST_EB, "E_EB",
               (Vec2){cx + h + arm, cy + lw * 0.5f},
               (Vec2){cx + h, cy + lw * 0.5f},
               (Vec2){cx + h + 18.0f, cy + lw * 0.5f}, 0.0f, 0);

    setup_lane(world, LANE_WEST_EB, "W_EB",
               (Vec2){cx - h - arm, cy + lw * 0.5f},
               (Vec2){cx - h, cy + lw * 0.5f},
               (Vec2){cx - h - 18.0f, cy + lw * 0.5f}, 0.0f, 1);

    setup_lane(world, LANE_WEST_WB, "W_WB",
               (Vec2){cx - h - arm, cy - lw * 0.5f},
               (Vec2){cx - h, cy - lw * 0.5f},
               (Vec2){cx - h - 18.0f, cy - lw * 0.5f}, 180.0f, 0);

    for (int i = 0; i < LANE_COUNT; i++) {
        build_paths_for_lane(world, (LaneId)i);
    }
}

const LaneInfo *world_lane(const World *world, LaneId lane)
{
    if (lane < 0 || lane >= LANE_COUNT) {
        return NULL;
    }
    return &world->lanes[lane];
}

const RoutePath *world_route(const World *world, LaneId lane, RouteType route)
{
    if (lane < 0 || lane >= LANE_COUNT || route < 0 || route >= ROUTE_COUNT) {
        return NULL;
    }
    return &world->routes[lane][route];
}

LaneId world_lane_for_spawn_direction(int direction_index)
{
    static const LaneId map[4] = {
        LANE_SOUTH_NB,
        LANE_NORTH_SB,
        LANE_WEST_EB,
        LANE_EAST_WB,
    };
    if (direction_index < 0 || direction_index > 3) {
        return LANE_SOUTH_NB;
    }
    return map[direction_index];
}

int world_lane_capacity(const LaneInfo *lane)
{
    float slot = VEHICLE_LENGTH + SAFETY_GAP;
    if (slot <= 0.0f) {
        return 0;
    }
    return (int)floorf(lane->lane_length / slot);
}
