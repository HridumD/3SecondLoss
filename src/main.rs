use bevy::prelude::*;
use bevy_ecs::bundle::DynamicBundle;

#[derive(Component, Copy, Clone)]
enum Colors {
    BLACK,
    WHITE
}

impl Colors {
    fn rgb(&self) -> Color {
        match *self {
            Colors::BLACK => Color::rgb(0.00, 0.00, 0.00),
            Colors::WHITE => Color::rgb(255.00, 255.00, 255.00),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct File(char);

#[derive(Component)]
struct Rank(f32);

#[derive(Bundle)]
struct Square {
    color: Colors,
    file: File,
    rank: Rank,

    sprite_bundle: SpriteBundle
}

#[derive(Bundle)]
struct Piece {
    square: Square,
    color: Colors,

    sprite_bundle: SpriteBundle
}

/* TOOD
    
    Match files with numbers
    Use file and rank numbers to position squares

*/

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let mut s_color: Colors;
    let mut s_rank: f32 = 1.;

    for x in 0..64 {
        if x % 2 == 0 {
            s_color = Colors::BLACK;
        }else {
            s_color = Colors::WHITE;
        }

        commands.spawn(Square {
            color: s_color,
            file: File('a'),
            rank: Rank(s_rank),

            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: s_color.rgb(),
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..default()
            }
        });
    }
}