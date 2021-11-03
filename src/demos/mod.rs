use crate::gui;
use crate::gui::DemoMenuSelection;
use crate::RunState;
use crate::State;
use rltk;
use rltk::Rltk;

pub mod astar_pathfinding;
pub mod field_of_view;
pub mod walking_around;

#[derive(PartialEq, Copy, Clone)]
pub enum Demo {
    AStarPathfinding,
    WalkingAround,
    FieldOfView,
}

pub fn select_demo(selected: DemoMenuSelection) -> RunState {
    match selected {
        DemoMenuSelection::AStarPathfindingDemo => RunState::Demo {
            demo: Demo::AStarPathfinding,
        },
        DemoMenuSelection::WalkingAroundDemo => RunState::Demo {
            demo: Demo::WalkingAround,
        },
        DemoMenuSelection::FieldOfViewDemo => RunState::Demo {
            demo: Demo::FieldOfView,
        },
        DemoMenuSelection::Exit => RunState::MainMenu {
            menu_selection: gui::MainMenuSelection::Demos,
        },
    }
}

#[allow(clippy::cognitive_complexity)]
pub fn tick(_state: &mut State, ctx: &mut Rltk, runstate: &RunState) -> RunState {
    let mut newrunstate = *runstate;

    match runstate {
        RunState::Demo { demo } => match demo {
            Demo::AStarPathfinding => {
                newrunstate = astar_pathfinding::tick(ctx, runstate);
            }
            Demo::WalkingAround => {
                newrunstate = walking_around::tick(ctx, runstate);
            }
            Demo::FieldOfView => {
                newrunstate = field_of_view::tick(ctx, runstate);
            }
        },
        _ => {}
    }

    newrunstate
}
