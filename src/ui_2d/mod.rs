
use bevy::prelude::*;
use crate::{UiElement, UiState};

pub fn interaction_system(mut ui_state: ResMut<UiState>,
                          ui_2d_element_query: Query<&Interaction, (Changed<Interaction>, With<UiElement>)>) {

    let mut found_ui_element_interact: bool = false;

    for interaction in ui_2d_element_query.iter() {
        match interaction {
            Interaction::Pressed | Interaction::Hovered => {
                found_ui_element_interact = true;
                break;
            },
            Interaction::None => (),
        }
    }

    ui_state.over_ui_2d_element = found_ui_element_interact;
}