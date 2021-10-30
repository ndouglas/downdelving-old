use crate::gui;
use crate::gui::DemoMenuSelection;
use crate::RunState;
use crate::State;
use rltk;
use rltk::Rltk;

pub mod astar_pathfinding;

#[derive(PartialEq, Copy, Clone)]
pub enum Demo {
    AStarPathfinding,
}

pub fn select_demo(selected: DemoMenuSelection) -> RunState {
    match selected {
        DemoMenuSelection::AStarPathfindingDemo => RunState::Demo {
            demo: Demo::AStarPathfinding,
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
        },
        _ => {}
    }

    newrunstate
}
