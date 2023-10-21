use bevy::prelude::*;

use smallvec::SmallVec;

pub mod ui_2d;
pub mod ui_3d;

#[derive(Component, Default)]
pub struct UiElement;

#[derive(Default, Resource)]
pub (crate) struct UiState {
    /// Contains entities whose Interaction should be set to None
    ui_3d_entities_to_reset: SmallVec<[Entity; 1]>,
    pub over_ui_2d_element: bool,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiState>()
            .add_systems(Update, (ui_2d::interaction_system, ui_3d::interaction_system).chain());
    }
}