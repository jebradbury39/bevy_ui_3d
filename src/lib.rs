use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use smallvec::SmallVec;

pub mod ui_2d;
pub mod ui_3d;

#[derive(Component, Default)]
pub struct UiElement;

/// Add this bundle to Nodes you create (2d elements)
/// Alternatively, just add the components separately
#[derive(Bundle, Default)]
pub struct Ui2dElementBundle {
    pub ui_element: UiElement,
    pub interaction: Interaction,
}

/// Add this bundle to 3d objects you create that you want to support interactions on
/// Alternatively, just add the components separately
#[derive(Bundle, Default)]
pub struct Ui3dElementBundle {
    pub ui_element: UiElement,
    pub interaction: ui_3d::Interaction3d,
    pub collider: Collider,
}

pub mod prelude {
    use crate::ui_3d;

    pub use ui_3d::Interaction3d;
}

#[derive(Default, Resource)]
pub (crate) struct UiState {
    /// Contains entities whose Interaction should be set to None
    ui_3d_entities_to_reset: SmallVec<[Entity; 1]>,
    pub over_ui_2d_element: bool,
}

#[derive(Default)]
pub struct UiExtPlugin {
    ui_3d_config: ui_3d::PluginConfig,
}

impl Plugin for UiExtPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiState>()
            .insert_resource(self.ui_3d_config.clone());

        if !app.is_plugin_added::<RapierPhysicsPlugin>() {
            app
                .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        }

        app
            .add_systems(Update, (ui_2d::interaction_system, ui_3d::interaction_system).chain());
    }
}