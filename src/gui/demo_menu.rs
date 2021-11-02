use crate::{rex_assets::RexAssets, RunState, State};
use rltk::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum DemoMenuSelection {
    AStarPathfindingDemo,
    WalkingAroundDemo,
    Exit,
}

#[derive(PartialEq, Copy, Clone)]
pub enum DemoMenuResult {
    NoSelection { selected: DemoMenuSelection },
    Selected { selected: DemoMenuSelection },
}

pub fn demo_menu(gs: &mut State, ctx: &mut Rltk) -> DemoMenuResult {
    let mut draw_batch = DrawBatch::new();
    let runstate = gs.ecs.fetch::<RunState>();
    let assets = gs.ecs.fetch::<RexAssets>();
    let black = RGB::named(rltk::BLACK);
    let wheat_on_black = ColorPair::new(RGB::named(rltk::WHEAT), black);
    let yellow_on_black = ColorPair::new(RGB::named(rltk::YELLOW), black);
    let magenta_on_black = ColorPair::new(RGB::named(rltk::MAGENTA), black);
    let white_on_black = ColorPair::new(RGB::named(rltk::WHITE), black);
    ctx.render_xp_sprite(&assets.menu, 0, 0);

    draw_batch.draw_double_box(Rect::with_size(24, 18, 31, 10), wheat_on_black);

    draw_batch.print_color_centered(20, "Demos", yellow_on_black);

    let mut y = 24;
    if let RunState::DemoMenu {
        menu_selection: selection,
    } = *runstate
    {
        y += 1;
        draw_batch.print_color_centered(
            y,
            "A* Pathfinding",
            if selection == DemoMenuSelection::AStarPathfindingDemo {
                magenta_on_black
            } else {
                white_on_black
            },
        );
        y += 1;
        draw_batch.print_color_centered(
            y,
            "Walking Around",
            if selection == DemoMenuSelection::WalkingAroundDemo {
                magenta_on_black
            } else {
                white_on_black
            },
        );
        y += 1;
        draw_batch.print_color_centered(
            y,
            "Exit",
            if selection == DemoMenuSelection::Exit {
                magenta_on_black
            } else {
                white_on_black
            },
        );
        draw_batch
            .submit(6000)
            .map_err(|err| println!("{:?}", err))
            .ok();

        match ctx.key {
            None => {
                return DemoMenuResult::NoSelection {
                    selected: selection,
                }
            }
            Some(key) => match key {
                VirtualKeyCode::Escape => {
                    return DemoMenuResult::NoSelection {
                        selected: DemoMenuSelection::Exit,
                    }
                }
                VirtualKeyCode::Up => {
                    let newselection;
                    match selection {
                        DemoMenuSelection::AStarPathfindingDemo => {
                            newselection = DemoMenuSelection::Exit
                        }
                        DemoMenuSelection::WalkingAroundDemo => {
                            newselection = DemoMenuSelection::AStarPathfindingDemo
                        }
                        DemoMenuSelection::Exit => {
                            newselection = DemoMenuSelection::WalkingAroundDemo
                        }
                    }
                    return DemoMenuResult::NoSelection {
                        selected: newselection,
                    };
                }
                VirtualKeyCode::Down => {
                    let newselection;
                    match selection {
                        DemoMenuSelection::AStarPathfindingDemo => {
                            newselection = DemoMenuSelection::WalkingAroundDemo
                        }
                        DemoMenuSelection::WalkingAroundDemo => {
                            newselection = DemoMenuSelection::Exit
                        }
                        DemoMenuSelection::Exit => {
                            newselection = DemoMenuSelection::AStarPathfindingDemo
                        }
                    }
                    return DemoMenuResult::NoSelection {
                        selected: newselection,
                    };
                }
                VirtualKeyCode::Return => {
                    return DemoMenuResult::Selected {
                        selected: selection,
                    }
                }
                _ => {
                    return DemoMenuResult::NoSelection {
                        selected: selection,
                    }
                }
            },
        }
    }

    DemoMenuResult::NoSelection {
        selected: DemoMenuSelection::Exit,
    }
}
