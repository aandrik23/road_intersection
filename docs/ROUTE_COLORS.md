# Route colors (audit)

Vehicles are painted by **route at spawn**. The route never changes after spawn.

| Route    | Color        | RGB (approx.)   |
|----------|--------------|-----------------|
| Left     | Blue         | `(60, 140, 255)` |
| Right    | Yellow       | `(255, 210, 40)` |
| Straight | Green        | `(50, 200, 90)`  |

Defined in `src/types.rs` as `route_color()`.

Random route at spawn: uniform over left / right / straight (`random_route_uniform` in `types.rs`).
