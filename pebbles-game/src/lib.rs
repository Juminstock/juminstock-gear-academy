use crate::pebbles_game_io::{PebblesInit, Player, GameState, };
use gstd::{msg, prelude::*};
use exec::random;

static mut GAME_STATE: Option<GameState> = None;

#[no_mangle]
extern "C" fn init(init_data: PebblesInit) {
    // Less the msg::load for obtain the value by the user
    if init_data.pebbles_count == 0 || init_data.max_pebbles_per_turn == 0 {
        panic!("Invalid initialization data: pebbles_count and max_pebbles_per_turn must be greater than 0.");
    }

    let first_player = if random::<bool>() { Player::User } else { Player::Program };

    let game_state = GameState {
        pebbles_count: init_data.pebbles_count,
        max_pebbles_per_turn: init_data.max_pebbles_per_turn,
        pebbles_remaining: init_data.pebbles_count,
        difficulty: init_data.difficulty.clone(),
        first_player,
        winner: None,
    };

    unsafe {
        GAME_STATE = Some(game_state);
    }
}

#[no_mangle]
extern "C" fn handle() {
    let action: PebblesAction = msg::load().expect("Failed to load action");

    let game_state = unsafe {
        if let Some(state) = &mut GAME_STATE {
            state
        } else {
            panic!("Game state not initialized.");
        }
    };

    match action {
        PebblesAction::Turn(pebbles_to_remove) => {
            if pebbles_to_remove == 0 || pebbles_to_remove > game_state.max_pebbles_per_turn {
                panic!("Invalid turn: pebbles_to_remove must be between 1 and max_pebbles_per_turn.");
            }

            game_state.pebbles_remaining -= pebbles_to_remove;

            if game_state.pebbles_remaining == 0 {
                game_state.winner = Some(Player::User);
            }

            if game_state.pebbles_remaining > 0 {
                process_program_turn(game_state);
            }

            let event = if game_state.pebbles_remaining == 0 {
                PebblesEvent::Won(Player::User)
            } else {
                PebblesEvent::CounterTurn(pebbles_to_remove)
            };

            msg::reply(event, 0).expect("Failed to send event to user");
        }
        _ => panic!("Invalid action: unsupported action type."),
    }
}