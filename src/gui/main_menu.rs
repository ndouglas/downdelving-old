use crate::{rex_assets::RexAssets, RunState, State};
use rltk::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection {
    NewGame,
    LoadGame,
    Demos,
    Quit,
}

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuResult {
    NoSelection { selected: MainMenuSelection },
    Selected { selected: MainMenuSelection },
}

pub fn main_menu(gs: &mut State, ctx: &mut Rltk) -> MainMenuResult {
    let mut draw_batch = DrawBatch::new();
    let save_exists = crate::saveload_system::does_save_exist();
    let runstate = gs.ecs.fetch::<RunState>();
    let assets = gs.ecs.fetch::<RexAssets>();
    let black = RGB::named(rltk::BLACK);
    let wheat_on_black = ColorPair::new(RGB::named(rltk::WHEAT), black);
    let yellow_on_black = ColorPair::new(RGB::named(rltk::YELLOW), black);
    let magenta_on_black = ColorPair::new(RGB::named(rltk::MAGENTA), black);
    let cyan_on_black = ColorPair::new(RGB::named(rltk::CYAN), black);
    let gray_on_black = ColorPair::new(RGB::named(rltk::GRAY), black);
    let white_on_black = ColorPair::new(RGB::named(rltk::WHITE), black);
    ctx.render_xp_sprite(&assets.menu, 0, 0);

    draw_batch.draw_double_box(Rect::with_size(24, 18, 31, 10), wheat_on_black);

    draw_batch.print_color_centered(20, "Downdelving", yellow_on_black);
    draw_batch.print_color_centered(21, "by Nathan Douglas", cyan_on_black);
    draw_batch.print_color_centered(22, "Use Up/Down Arrows and Enter", gray_on_black);

    let mut y = 24;
    if let RunState::MainMenu {
        menu_selection: selection,
    } = *runstate
    {
        draw_batch.print_color_centered(
            y,
            "Begin New Game",
            if selection == MainMenuSelection::NewGame {
                magenta_on_black
            } else {
                white_on_black
            },
        );
        y += 1;

        if save_exists {
            draw_batch.print_color_centered(
                y,
                "Load Game",
                if selection == MainMenuSelection::LoadGame {
                    magenta_on_black
                } else {
                    white_on_black
                },
            );
            y += 1;
        }

        draw_batch.print_color_centered(
            y,
            "Demos",
            if selection == MainMenuSelection::Demos {
                magenta_on_black
            } else {
                white_on_black
            },
        );
        y += 1;

        draw_batch.print_color_centered(
            y,
            "Quit",
            if selection == MainMenuSelection::Quit {
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
                return MainMenuResult::NoSelection {
                    selected: selection,
                }
            }
            Some(key) => match key {
                VirtualKeyCode::Escape => {
                    return MainMenuResult::NoSelection {
                        selected: MainMenuSelection::Quit,
                    }
                }
                VirtualKeyCode::Up => {
                    let mut newselection;
                    match selection {
                        MainMenuSelection::NewGame => newselection = MainMenuSelection::Quit,
                        MainMenuSelection::LoadGame => newselection = MainMenuSelection::NewGame,
                        MainMenuSelection::Demos => newselection = MainMenuSelection::LoadGame,
                        MainMenuSelection::Quit => newselection = MainMenuSelection::Demos,
                    }
                    if newselection == MainMenuSelection::LoadGame && !save_exists {
                        newselection = MainMenuSelection::NewGame;
                    }
                    return MainMenuResult::NoSelection {
                        selected: newselection,
                    };
                }
                VirtualKeyCode::Down => {
                    let mut newselection;
                    match selection {
                        MainMenuSelection::NewGame => newselection = MainMenuSelection::LoadGame,
                        MainMenuSelection::LoadGame => newselection = MainMenuSelection::Demos,
                        MainMenuSelection::Demos => newselection = MainMenuSelection::Quit,
                        MainMenuSelection::Quit => newselection = MainMenuSelection::NewGame,
                    }
                    if newselection == MainMenuSelection::LoadGame && !save_exists {
                        newselection = MainMenuSelection::Demos;
                    }
                    return MainMenuResult::NoSelection {
                        selected: newselection,
                    };
                }
                VirtualKeyCode::Return => {
                    return MainMenuResult::Selected {
                        selected: selection,
                    }
                }
                _ => {
                    return MainMenuResult::NoSelection {
                        selected: selection,
                    }
                }
            },
        }
    }

    MainMenuResult::NoSelection {
        selected: MainMenuSelection::NewGame,
    }
}
