# Marked Commands

Generates an extension trait that implements the methods 
`marked` and `marked_bundle` (and some other ones) equivalent to
`spawn` and `spawn_bundle` except the new entities will also contain
the marker components passed to the macro.
The marker components must implement `Default`.

## usage example

```rust
marked_commands!((UIElementMarker,));

pub fn spawn_text_box(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
 
    commands.marked_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect { left: Val::Px(100.0), bottom: Val::Px(100.0), ..Default::default() },
            padding: Rect::all(Val::Px(2.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|builder| {
        builder.marked_bundle(NodeBundle {
                color: UiColor (Color::DARK_GRAY),
                style: Style {
                    padding: Rect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            }
        )
        .with_children(|builder| {
            builder.marked_bundle(TextBundle {
                text: Text::with_section(
                    "Hello, world!",
                    TextStyle {
                        font: asset_server.load("FiraMono-Regular.ttf"),
                        font_size: 16.0,
                        color: Color::ANTIQUE_WHITE,
                    },
                    TextAlignment::default()
                ),
                 ..Default::default()
            });
        });
    });
}

pub fn despawn_text_box(
    mut commands: Commands,
    query: Query<Entity, With<UIElementMarker>>
) {
    query.for_each(|entity|
        commands.entity(entity).despawn();
    );
}
```
## Info

* Takes a bundle, so you can add as many marker components as you like! 100, why not.
* I wouldn't use this (because it's a macro).
* The example is bad. You only need to mark the root `NodeBundle` and then call `despawn_recursive`. 
But then do you really need the macro if you are only marking the root.