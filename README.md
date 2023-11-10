# Bevy UI 3D

This library provides a simple way to tag game objects as UI elements.
You can then write queries on UI interactions for these elements.

# Bevy compatibility

| Bevy Version | bevy_ui_3d version |
|--------------|--------------------|
| ^0.11.3      | ^0.1.0             |
| ^0.12.0      | ^0.2.0             | 

# Examples

You can look at the examples folder for practical uses of this crate, but to get started,
you can simply do the following when setting up your app and scene:

```rust
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_ui_3d::{Interaction3d, Ui3dElementBundle, Ui3dPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Ui3dPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // ... other setup code

    // add a 3d gameobject with UI interactions
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Ui3dElementBundle {
            // the collider is needed, since the interaction is detected via raycasting
            collider: Collider::cuboid(0.5, 0.5, 0.5),
            ..default()
        },
    ));
}
```

# Configuring the Plugin

The plugin comes with some configuration options you can set on startup, since you may
not be interested in all of the interaction types, and too many interactions can
be noisy.

```rust
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_ui_3d::{Interaction3d, Ui3dElementBundle, Ui3dPlugin, PluginConfig};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Ui3dPlugin {
            config: PluginConfig {
                hover_enabled: false,
                hover_point_enabled: false,
                press_enabled: true,
                press_point_enabled: false,
            }
        })
        .add_startup_system(setup)
        .run();
}
```
