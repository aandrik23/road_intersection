#include "renderer.h"

#include "sim_config.h"
#include <math.h>
#include <stdio.h>

static void set_color(SDL_Renderer *r, uint8_t cr, uint8_t cg, uint8_t cb, uint8_t a)
{
    SDL_SetRenderDrawColor(r, cr, cg, cb, a);
}

static void fill_rect(SDL_Renderer *r, int x, int y, int w, int h)
{
    SDL_Rect rect = {x, y, w, h};
    SDL_RenderFillRect(r, &rect);
}

static void draw_rect_outline(SDL_Renderer *r, int x, int y, int w, int h)
{
    SDL_Rect rect = {x, y, w, h};
    SDL_RenderDrawRect(r, &rect);
}

static void draw_arrow(SDL_Renderer *r, int x, int y, int dx, int dy)
{
    set_color(r, 230, 230, 230, 255);
    SDL_RenderDrawLine(r, x, y, x + dx, y + dy);
    int tip_x = x + dx;
    int tip_y = y + dy;
    if (dy > 0) {
        SDL_RenderDrawLine(r, tip_x, tip_y, tip_x - 5, tip_y - 8);
        SDL_RenderDrawLine(r, tip_x, tip_y, tip_x + 5, tip_y - 8);
    } else if (dy < 0) {
        SDL_RenderDrawLine(r, tip_x, tip_y, tip_x - 5, tip_y + 8);
        SDL_RenderDrawLine(r, tip_x, tip_y, tip_x + 5, tip_y + 8);
    } else if (dx > 0) {
        SDL_RenderDrawLine(r, tip_x, tip_y, tip_x - 8, tip_y - 5);
        SDL_RenderDrawLine(r, tip_x, tip_y, tip_x - 8, tip_y + 5);
    } else if (dx < 0) {
        SDL_RenderDrawLine(r, tip_x, tip_y, tip_x + 8, tip_y - 5);
        SDL_RenderDrawLine(r, tip_x, tip_y, tip_x + 8, tip_y + 5);
    }
}

static void draw_roads(SDL_Renderer *r)
{
    const int cx = CENTER_X;
    const int cy = CENTER_Y;
    const int h = INTERSECTION_HALF;
    const int road_half = LANE_WIDTH + 6;

    set_color(r, 45, 48, 55, 255);
    fill_rect(r, 0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);

    set_color(r, 62, 66, 74, 255);
    fill_rect(r, cx - road_half, 0, road_half * 2, WINDOW_HEIGHT);
    fill_rect(r, 0, cy - road_half, WINDOW_WIDTH, road_half * 2);

    set_color(r, 45, 48, 55, 255);
    fill_rect(r, cx - h, cy - h, h * 2, h * 2);

    set_color(r, 90, 95, 105, 255);
    SDL_RenderDrawLine(r, cx, 0, cx, cy - h);
    SDL_RenderDrawLine(r, cx, cy + h, cx, WINDOW_HEIGHT);
    SDL_RenderDrawLine(r, 0, cy, cx - h, cy);
    SDL_RenderDrawLine(r, cx + h, cy, WINDOW_WIDTH, cy);

    set_color(r, 40, 42, 48, 255);
    fill_rect(r, cx - 2, 0, 4, WINDOW_HEIGHT);
    fill_rect(r, 0, cy - 2, WINDOW_WIDTH, 4);

    set_color(r, 255, 255, 255, 255);
    for (int y = cy - h - 8; y > 20; y -= 28) {
        SDL_RenderDrawLine(r, cx + LANE_WIDTH / 2 - 1, y, cx + LANE_WIDTH / 2 - 1, y + 14);
        SDL_RenderDrawLine(r, cx - LANE_WIDTH / 2, y, cx - LANE_WIDTH / 2, y + 14);
    }
    for (int y = cy + h + 8; y < WINDOW_HEIGHT - 20; y += 28) {
        SDL_RenderDrawLine(r, cx + LANE_WIDTH / 2 - 1, y, cx + LANE_WIDTH / 2 - 1, y + 14);
        SDL_RenderDrawLine(r, cx - LANE_WIDTH / 2, y, cx - LANE_WIDTH / 2, y + 14);
    }
    for (int x = cx - h - 8; x > 20; x -= 28) {
        SDL_RenderDrawLine(r, x, cy + LANE_WIDTH / 2 - 1, x + 14, cy + LANE_WIDTH / 2 - 1);
        SDL_RenderDrawLine(r, x, cy - LANE_WIDTH / 2, x + 14, cy - LANE_WIDTH / 2);
    }
    for (int x = cx + h + 8; x < WINDOW_WIDTH - 20; x += 28) {
        SDL_RenderDrawLine(r, x, cy + LANE_WIDTH / 2 - 1, x + 14, cy + LANE_WIDTH / 2 - 1);
        SDL_RenderDrawLine(r, x, cy - LANE_WIDTH / 2, x + 14, cy - LANE_WIDTH / 2);
    }
}

static void draw_stop_lines(const Simulation *sim, SDL_Renderer *r)
{
    set_color(r, 240, 240, 240, 255);
    for (int i = 0; i < LANE_COUNT; i++) {
        const LaneInfo *lane = &sim->world.lanes[i];
        int sx = (int)lane->stop_line.x;
        int sy = (int)lane->stop_line.y;
        if (lane->heading == 90.0f || lane->heading == 270.0f) {
            SDL_RenderDrawLine(r, sx - 14, sy, sx + 14, sy);
        } else {
            SDL_RenderDrawLine(r, sx, sy - 14, sx, sy + 14);
        }
    }
}

static void draw_route_preview(const Simulation *sim, SDL_Renderer *r)
{
    set_color(r, 80, 85, 95, 80);
    for (int lane = 0; lane < LANE_COUNT; lane++) {
        if (!sim->world.lanes[lane].inbound) {
            continue;
        }
        const RoutePath *path = &sim->world.routes[lane][ROUTE_STRAIGHT];
        for (int i = 1; i < path->count; i++) {
            SDL_RenderDrawLine(
                r,
                (int)path->waypoints[i - 1].x,
                (int)path->waypoints[i - 1].y,
                (int)path->waypoints[i].x,
                (int)path->waypoints[i].y);
        }
    }
}

static void draw_traffic_light(SDL_Renderer *r, int x, int y, SignalState state)
{
    set_color(r, 30, 30, 35, 255);
    fill_rect(r, x - 8, y - 18, 16, 36);
    if (state == SIGNAL_GREEN) {
        set_color(r, 40, 200, 70, 255);
    } else {
        set_color(r, 40, 40, 45, 255);
    }
    SDL_Rect green_bulb = {x - 5, y + 2, 10, 10};
    SDL_RenderFillRect(r, &green_bulb);

    if (state == SIGNAL_RED) {
        set_color(r, 220, 50, 50, 255);
    } else {
        set_color(r, 55, 55, 60, 255);
    }
    SDL_Rect red_bulb = {x - 5, y - 14, 10, 10};
    SDL_RenderFillRect(r, &red_bulb);
}

static void draw_traffic_lights(const Simulation *sim, SDL_Renderer *r)
{
    for (int i = 0; i < LANE_COUNT; i++) {
        const LaneInfo *lane = &sim->world.lanes[i];
        draw_traffic_light(
            r,
            (int)lane->light_pos.x,
            (int)lane->light_pos.y,
            sim->lane_signals[i]);
    }
}

static void draw_direction_arrows(SDL_Renderer *r)
{
    const int cx = CENTER_X;
    const int cy = CENTER_Y;
    const int h = INTERSECTION_HALF;
    const int lw = LANE_WIDTH;

    draw_arrow(r, cx + lw / 2, 70, 0, 40);
    draw_arrow(r, cx - lw / 2, WINDOW_HEIGHT - 70, 0, -40);
    draw_arrow(r, 70, cy - lw / 2, 40, 0);
    draw_arrow(r, WINDOW_WIDTH - 70, cy + lw / 2, -40, 0);

    draw_arrow(r, cx + lw / 2, cy - h - 50, 0, 30);
    draw_arrow(r, cx - lw / 2, cy + h + 50, 0, -30);
    draw_arrow(r, cx + h + 50, cy - lw / 2, -30, 0);
    draw_arrow(r, cx - h - 50, cy + lw / 2, 30, 0);
}

static void draw_cardinal_labels(SDL_Renderer *r)
{
    set_color(r, 200, 200, 210, 255);
    const int cx = CENTER_X;
    const int pad = 24;
    SDL_RenderDrawLine(r, cx - 6, pad, cx + 6, pad);
    SDL_RenderDrawLine(r, cx, pad - 6, cx, pad + 6);
    SDL_RenderDrawLine(r, cx - 6, WINDOW_HEIGHT - pad, cx + 6, WINDOW_HEIGHT - pad);
    SDL_RenderDrawLine(r, cx, WINDOW_HEIGHT - pad - 6, cx, WINDOW_HEIGHT - pad + 6);
    SDL_RenderDrawLine(r, pad - 6, CENTER_Y, pad + 6, CENTER_Y);
    SDL_RenderDrawLine(r, pad, CENTER_Y - 6, pad, CENTER_Y + 6);
    SDL_RenderDrawLine(r, WINDOW_WIDTH - pad - 6, CENTER_Y, WINDOW_WIDTH - pad + 6, CENTER_Y);
    SDL_RenderDrawLine(r, WINDOW_WIDTH - pad, CENTER_Y - 6, WINDOW_WIDTH - pad, CENTER_Y + 6);
}

static void draw_vehicles(SDL_Renderer *r, const Simulation *sim)
{
    for (int i = 0; i < sim->vehicle_count; i++) {
        const VehicleDraw *v = &sim->vehicles[i];
        set_color(r, v->r, v->g, v->b, 255);
        int hw = (int)(v->width * 0.5f);
        int hh = (int)(v->height * 0.5f);
        fill_rect(r, (int)v->x - hw, (int)v->y - hh, hw * 2, hh * 2);
    }
}

static void draw_intersection_box(SDL_Renderer *r)
{
    set_color(r, 75, 80, 90, 255);
    draw_rect_outline(
        r,
        CENTER_X - INTERSECTION_HALF,
        CENTER_Y - INTERSECTION_HALF,
        INTERSECTION_HALF * 2,
        INTERSECTION_HALF * 2);
}

int renderer_init(AppRenderer *app, const char *title)
{
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        fprintf(stderr, "SDL_Init failed: %s\n", SDL_GetError());
        return -1;
    }

    app->window = SDL_CreateWindow(
        title,
        SDL_WINDOWPOS_CENTERED,
        SDL_WINDOWPOS_CENTERED,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        SDL_WINDOW_SHOWN);
    if (!app->window) {
        fprintf(stderr, "SDL_CreateWindow failed: %s\n", SDL_GetError());
        SDL_Quit();
        return -1;
    }

    app->renderer = SDL_CreateRenderer(app->window, -1, SDL_RENDERER_ACCELERATED);
    if (!app->renderer) {
        fprintf(stderr, "SDL_CreateRenderer failed: %s\n", SDL_GetError());
        SDL_DestroyWindow(app->window);
        SDL_Quit();
        return -1;
    }

    return 0;
}

void renderer_destroy(AppRenderer *app)
{
    if (app->renderer) {
        SDL_DestroyRenderer(app->renderer);
        app->renderer = NULL;
    }
    if (app->window) {
        SDL_DestroyWindow(app->window);
        app->window = NULL;
    }
    SDL_Quit();
}

void renderer_draw_frame(AppRenderer *app, const Simulation *sim)
{
    SDL_Renderer *r = app->renderer;
    set_color(r, 0, 0, 0, 255);
    SDL_RenderClear(r);

    draw_roads(r);
    draw_route_preview(sim, r);
    draw_intersection_box(r);
    draw_stop_lines(sim, r);
    draw_direction_arrows(r);
    draw_cardinal_labels(r);
    draw_traffic_lights(sim, r);
    draw_vehicles(r, sim);

    SDL_RenderPresent(r);
}
