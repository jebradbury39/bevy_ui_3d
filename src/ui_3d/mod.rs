use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::window::PrimaryWindow;

use bevy_rapier3d::prelude::*;

use crate::{UiElement, UiState};

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum Interaction3d {
    #[default]
    None,
    Pressed(Vec3),
    Hovered(Vec3),
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct NodeQuery {
    entity: Entity,
    node: &'static Node,
    ui_element: &'static UiElement,
    interaction: &'static mut Interaction3d,
    collider: &'static Collider,
    computed_visibility: Option<&'static ComputedVisibility>,
}

pub fn interaction_system(mut ui_state: ResMut<UiState>,
                          mouse_button_input: Res<Input<MouseButton>>,
                          touches_input: Res<Touches>,
                          window_query: Query<&Window, With<PrimaryWindow>>,
                          rapier_context: Res<RapierContext>,
                          cameras: Query<(&Camera, &GlobalTransform)>,
                          mut ui_events: EventWriter<UiEventPacket>,
                          mut node_query: Query<NodeQuery>) {

    let primary_window = primary_window.iter().next();

    // reset entities that were both clicked and released in the last frame
    for entity in ui_state.ui_3d_entities_to_reset.drain(..) {
        if let Ok(mut interaction) = node_query.get_component_mut::<Interaction3d>(entity) {
            *interaction = Interaction3d::None;
        }
    }

    let mouse_released =
        mouse_button_input.just_released(MouseButton::Left) || touches_input.any_just_released();
    if mouse_released {
        for node in node_query.iter_mut() {
            if *node.interaction == Interaction::Pressed {
                *node.interaction = Interaction::None;
            }
        }
    }

    let mouse_clicked =
        mouse_button_input.just_pressed(MouseButton::Left) || touches_input.any_just_pressed();

    let mut hovered_nodes: Vec<Entity> = Vec::new();
    for node in node_query.iter_mut() {
        if mouse_clicked || ui_state.over_ui_2d_element {
            // not hovering any 3d interactions since ui event consumed already
            if *node.interaction == Interaction3d::Hovered {
                *node.interaction = Interaction3d::None;
            }
        } else {
            hovered_nodes.push(node.entity);
        }
    }

    if ui_state.over_ui_2d_element {
        return;
    }

    let mut clicked_obj: bool = false;
    let mut next_event_idx: usize = 0;
    let mut event_packet = UiEventPacket::default();

    for (camera, camera_transform) in cameras.iter() {
        // compute ray from the mouse position
        let ray = if let Some(ray) = ray_from_mouse_position(primary_window, camera, camera_transform) {
            ray
        } else {
            continue;
        };

        let query_filter = QueryFilter::new().predicate(|entity| node_query.get(entity).is_ok());

        // cast the ray
        let hit: Option<Ray> = rapier_context.cast_ray_and_get_normal(
            ray.origin,
            ray.direction,
            f32::MAX,
            true,
            query_filter,
        );

        if let Some((entity, ray_intersection)) = hit {
            let hit_point = ray_intersection.point;

            let mut node = node_query.get_mut(entity).unwrap();
            if mouse_clicked {
                node.interaction.set_if_neq(Interaction3d::Pressed(hit_point));
                if mouse_released {
                    state.ui_3d_entities_to_reset.push(node.entity);
                }
            } else {
                if matches!(*node.interaction, Interaction3d::None | Interaction3d::Hovered(_)) {
                    node.interaction.set_if_neq(Interaction3d::Hovered(hit_point));

                    let mut hovered_node_idx: Option<usize> = None;
                    for (idx, hovered_node) in hovered_nodes.iter().enumerate() {
                        if *hovered_node == node.entity {
                            hovered_node_idx = Some(idx);
                            break;
                        }
                    }
                    if let Some(hovered_node_idx) = hovered_node_idx {
                        hovered_nodes.remove(hovered_node_idx);
                    }
                }
            }
        }
    }

    let mut iter = node_query.iter_many_mut(hovered_nodes);
    while let Some(node) = iter.fetch_next() {
        // don't reset pressed nodes because they're handled separately
        if *node.interaction != Interaction::Pressed {
            node.interaction.set_if_neq(Interaction::None);
        }
    }
}

fn ray_from_mouse_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Ray> {
    let mouse_position = window.cursor_position().unwrap_or(Vec2::new(0.0, 0.0));
    return camera.viewport_to_world(camera_transform, mouse_position);
}