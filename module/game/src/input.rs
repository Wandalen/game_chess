use bevy::prelude::*;
use bevy_interact_2d::{Group, InteractionState};

pub fn interaction_system(
    mouse_button_input: Res<Input<MouseButton>>,
    interaction_state: Res<InteractionState>,
) {
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    for (entity, coords) in interaction_state.get_group(Group(1)).iter() {
        dbg!((entity, coords));
    }
}
