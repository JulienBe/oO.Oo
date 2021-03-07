use image::{Pixel, Rgba};
use bevy::{app::{App, Plugin, AppBuilder}, render::camera::OrthographicProjection};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin};
use rand::Rng;
extern crate image;

use image::GenericImageView;
use core::fmt;
use std::fmt::Formatter;

struct Pos {
    x: u16,
    y: u16,
}

const snow: u8 = 0b0001;
const rock: u8 = 0b0010;

const map_width: u16 = 160;
const map_height: u16 = 144;
const rocks_layer: &str = "assets/layers/rocks_normal_map.jpg";

struct Rock {
    x_slope: f32,
    y_slope: f32,
    pos: Pos,
}
impl fmt::Display for Rock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({} {}) ({})", self.x_slope, self.y_slope, self.pos)
    }
}
impl fmt::Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }

}
struct PixCell {
    components: u8,
}
struct Map {
    layers: [u8; (map_width * map_height) as usize],
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(map_builder.system())
        .add_system(animate_sprite_system.system())
        .run();
}

fn map_builder(commands: &mut Commands) {
    let img = image::open(rocks_layer).unwrap();
    for pixel in img.pixels() {
        process_pixel(pixel);
    }
    img.pixels()
        .map(|p| process_pixel(p))
        .filter(|op| op.is_some())
        .map(|op| op.unwrap())
        .for_each(|p| println!("Rock {}", p))
}

fn process_pixel(pixel: (u32, u32, Rgba<u8>)) -> Option<Rock> {
    let color = Pixel::channels4(&pixel.2);
    if color.2 < 125 {
        None
    } else {
        Some(Rock {
            x_slope: color.0 as f32 / 255.0,
            y_slope: color.1 as f32 / 255.0,
            pos: Pos { x: pixel.0 as u16, y: pixel.1 as u16 }
        })
    }
}

fn animate_sprite_system(time: Res<Time>, texture_atlases: Res<Assets<TextureAtlas>>, mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let texture_handle = asset_server.load("berserker/running/run1.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true));
}