use super::SystemState;

use serde_json;

#[test]
fn test_system_state() {
    let state = SystemState::default();

    println!("{}", serde_json::to_string_pretty(&state).unwrap());
}
