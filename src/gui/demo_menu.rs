use crate::{rex_assets::RexAssets, RunState, State};
use rltk::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum DemoMenuSelection {
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
    ctx.render_xp_sprite(&assets.menu, 0, 0);

    draw_batch.draw_double_box(
        Rect::with_size(24, 18, 31, 10),
        ColorPair::new(RGB::named(rltk::WHEAT), RGB::named(rltk::BLACK)),
    );

    draw_batch.print_color_centered(
        20,
        "Demos",
        ColorPair::new(RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK)),
    );

    let mut y = 24;
    if let RunState::DemoMenu {
        menu_selection: selection,
    } = *runstate
    {
        y += 1;
        if selection == DemoMenuSelection::Exit {
            draw_batch.print_color_centered(
                y,
                "Exit",
                ColorPair::new(RGB::named(rltk::MAGENTA), RGB::named(rltk::BLACK)),
            );
        } else {
            draw_batch.print_color_centered(
                y,
                "Exit",
                ColorPair::new(RGB::named(rltk::WHITE), RGB::named(rltk::BLACK)),
            );
        }

        draw_batch.submit(6000).map_err(|err| println!("{:?}", err)).ok();

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
                        DemoMenuSelection::Exit => newselection = DemoMenuSelection::Exit,
                    }
                    return DemoMenuResult::NoSelection {
                        selected: newselection,
                    };
                }
                VirtualKeyCode::Down => {
                    let newselection;
                    match selection {
                        DemoMenuSelection::Exit => newselection = DemoMenuSelection::Exit,
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
