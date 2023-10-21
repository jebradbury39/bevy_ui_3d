use bevy::prelude::*;

pub mod ui_2d;
pub mod ui_3d;

#[derive(Default, Resource)]
pub struct MouseState {
    current_interactable: Option<(Entity, Vec3)>,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ui_3d::MouseState>()
            .add_event::<ui_3d::InteractableEventPacket>()
            .add_systems(Update, (UiElement::interaction, ui_3d::mouse_click));
    }
}