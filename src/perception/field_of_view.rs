use crate::map::Map;
use rltk::BaseMap;

/// Recursive Symmetric Shadowcasting
///
/// Based on Sean Bowman's Gist:
/// https://gist.github.com/sbowman/eb277f162b4890346fe704653b4c2a53

// A sector to shadow cast.
struct Transform {
    xx: i32,
    xy: i32,
    yx: i32,
    yy: i32,
}

// Viewer holds information about the location and viewing radius of the character we're
// calculating the FOV for.
struct Viewer {
    x: i32,
    y: i32,
    radius: i32,
}

// Calculates the field of view on the map using shadow casting.  See:
//
// * http://www.roguebasin.com/index.php?title=FOV_using_recursive_shadowcasting
// * http://www.roguebasin.com/index.php?title=C%2B%2B_shadowcasting_implementation
//
// Returns the vector of points visible to the viewer located at (x, y) with the given viewing
// radius.
pub fn field_of_view(x: i32, y: i32, radius: i32, map: &Map) -> Vec<rltk::Point> {
    let transforms: Vec<Transform> = vec![
        // 0 E-NE
        Transform {
            xx: 1,
            xy: 0,
            yx: 0,
            yy: 1,
        },
        // 1 NE-N
        Transform {
            xx: 0,
            xy: 1,
            yx: 1,
            yy: 0,
        },
        // 2 N-NW
        Transform {
            xx: 0,
            xy: -1,
            yx: 1,
            yy: 0,
        },
        // 3 NW-W
        Transform {
            xx: -1,
            xy: 0,
            yx: 0,
            yy: 1,
        },
        // 4 W-SW
        Transform {
            xx: -1,
            xy: 0,
            yx: 0,
            yy: -1,
        },
        // 5 SW-S
        Transform {
            xx: 0,
            xy: -1,
            yx: -1,
            yy: 0,
        },
        // 6 S-SE
        Transform {
            xx: 0,
            xy: 1,
            yx: -1,
            yy: 0,
        },
        // 7 SE-E
        Transform {
            xx: 1,
            xy: 0,
            yx: 0,
            yy: -1,
        },
    ];

    let viewer = Viewer { x, y, radius };

    let mut visible: Vec<rltk::Point> = Vec::new();

    // The viewer's location is always visible
    visible.push(rltk::Point { x, y });

    for transform in transforms {
        cast_light(&mut visible, map, &viewer, 1, 1.0, 0.0, &transform);
    }

    visible
}

fn cast_light(
    visible: &mut Vec<rltk::Point>,
    map: &Map,
    viewer: &Viewer,
    row: i32,
    start_slope: f32,
    end_slope: f32,
    transform: &Transform,
) {
    if start_slope < end_slope {
        return;
    }

    let radius_sq = viewer.radius * viewer.radius;

    let mut start_slope = start_slope;
    let mut next_start_slope = start_slope;

    for i in row..=viewer.radius {
        let mut blocked = false;
        let dy = -i;

        for dx in -i..=0 {
            let left_slope = (dx as f32 - 0.5) / (dy as f32 + 0.5);
            let right_slope = (dx as f32 + 0.5) / (dy as f32 - 0.5);

            if start_slope < right_slope {
                continue;
            }

            if end_slope > left_slope {
                break;
            }

            let sax = dx * transform.xx + dy * transform.xy;
            let say = dx * transform.yx + dy * transform.yy;
            if (sax < 0 && sax.abs() > viewer.x) || (say < 0 && say.abs() > viewer.y) {
                continue;
            }

            let ax = viewer.x + sax;
            let ay = viewer.y + say;
            if ax >= map.width || ay >= map.height {
                continue;
            }

            if dx * dx + dy * dy < radius_sq {
                visible.push(rltk::Point { x: ax, y: ay });
            }

            if blocked {
                if map.is_opaque(map.xy_idx(ax, ay)) {
                    next_start_slope = right_slope;
                    continue;
                }

                blocked = false;
                start_slope = next_start_slope;
                continue;
            }

            if map.is_opaque(map.xy_idx(ax, ay)) {
                blocked = true;
                next_start_slope = right_slope;
                cast_light(
                    visible,
                    map,
                    viewer,
                    row + 1,
                    start_slope,
                    left_slope,
                    transform,
                );
            }
        }

        if blocked {
            break;
        }
    }
}
