use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use smallvec::SmallVec;

mod ui_3d;

pub use ui_3d::{Interaction3d, PluginConfig};

#[derive(Component, Default)]
pub struct Ui3dElement;

/// Add this bundle to 3d objects you create that you want to support interactions on
/// Alternatively, just add the components separately
#[derive(Bundle, Default)]
pub struct Ui3dElementBundle {
    pub ui_element: Ui3dElement,
    pub interaction: Interaction3d,
    pub collider: Collider,
}

#[derive(Default, Resource)]
pub(crate) struct UiState {
    /// Contains entities whose Interaction should be set to None
    ui_3d_entities_to_reset: SmallVec<[Entity; 1]>,
}

#[derive(Default)]
pub struct Ui3dPlugin {
    config: PluginConfig,
}

impl Plugin for Ui3dPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .insert_resource(self.config.clone());

        if !app.is_plugin_added::<RapierPhysicsPlugin>() {
            app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        }

        app.add_systems(
            PreUpdate,
            (absorb_bevy_ui_inputs).after(bevy::ui::ui_focus_system),
        );

        app.add_systems(Update, ui_3d::interaction_system);
    }
}

fn absorb_bevy_ui_inputs(
    mut mouse: ResMut<Input<MouseButton>>,
    interaction_query: Query<&Interaction, (With<Node>, Changed<Interaction>)>,
) {
    let event_absorbed_by_ui = interaction_query
        .iter()
        .any(|i| matches!(i, Interaction::Pressed | Interaction::Hovered));

    if event_absorbed_by_ui {
        mouse.reset_all();
    }
}
