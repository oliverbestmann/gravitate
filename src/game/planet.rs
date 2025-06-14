use crate::common::stopwatch::Stopwatch;
use crate::game::assets::PlanetAssets;
use crate::game::attraction::Attractor;
use crate::game::cv::LAYER_PLANETS;
use crate::game::wiggle::Wiggle;
use avian2d::prelude::{Collider, ColliderDensity, RigidBody};
use bevy::asset::RenderAssetUsages;
use bevy::ecs::spawn::SpawnIter;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Primitive};
use std::collections::HashMap;
use std::ops::Deref;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CropCache>();
}

pub struct Planet;

pub fn bundle(
    assets: &PlanetAssets,
    crop_cache: &mut CropCache,
    images: &mut Assets<Image>,
    radius: f32,
) -> impl Bundle {
    let mut layer_bundle = |idx: usize, image: &Handle<Image>| {
        // crop empty regions from image
        let cropped = image_crop(crop_cache, images, image).unwrap();

        // position & size must be scaled by this much
        let scale = Vec2::splat(2.0 * radius) / cropped.full_size.as_vec2();

        // the size of the cropped image
        let size = cropped.rect.size().as_vec2() * scale;

        // calculate the offset in position
        let offset = cropped.rect.min.as_vec2() * vec2(1.0, -1.0) / scale + vec2(-radius, radius);

        (
            LAYER_PLANETS.offset_by(idx as i32),
            Wiggle::with_offset(offset),
            Sprite {
                image: cropped.handle.clone(),
                custom_size: Some(size),
                anchor: Anchor::TopLeft,
                ..default()
            },
        )
    };

    let children: Vec<_> = assets
        .images
        .iter()
        .enumerate()
        .map(|(idx, image)| layer_bundle(idx, image))
        .collect();

    (
        RigidBody::Static,
        Collider::circle(radius),
        ColliderDensity(100000.0),
        Attractor,
        Visibility::Inherited,
        Children::spawn(SpawnIter(children.into_iter())),
    )
}

#[derive(Default, Resource, Reflect, Deref, DerefMut)]
pub struct CropCache(HashMap<Handle<Image>, Cropped>);

#[derive(Clone, Reflect)]
pub struct Cropped {
    pub handle: Handle<Image>,
    pub rect: URect,
    pub full_size: UVec2,
}

fn image_crop(
    cache: &mut CropCache,
    images: &mut Assets<Image>,
    image: &Handle<Image>,
) -> Result<Cropped> {
    if let Some(cached) = cache.get(image) {
        return Ok(cached.clone());
    }

    let input = images.get(image).ok_or("failed to find image")?;

    let _watch = Stopwatch::new(format!("Crop image of size {}", input.size()));

    // find first row that is non empty
    let full_image = input.clone().try_into_dynamic()?;

    let crop_rect = match &full_image {
        DynamicImage::ImageLuma8(image) => find_crop_rect(image),
        DynamicImage::ImageLumaA8(image) => find_crop_rect(image),
        DynamicImage::ImageRgb8(image) => find_crop_rect(image),
        DynamicImage::ImageRgba8(image) => find_crop_rect(image),
        DynamicImage::ImageLuma16(image) => find_crop_rect(image),
        DynamicImage::ImageLumaA16(image) => find_crop_rect(image),
        DynamicImage::ImageRgb16(image) => find_crop_rect(image),
        DynamicImage::ImageRgba16(image) => find_crop_rect(image),
        DynamicImage::ImageRgb32F(image) => find_crop_rect(image),
        DynamicImage::ImageRgba32F(image) => find_crop_rect(image),
        _ => Err("unknown image type")?,
    };

    let full_size = UVec2::new(full_image.width(), full_image.height());

    let cropped = if crop_rect.is_empty() || crop_rect.size() == full_size {
        // we keep the original image
        Cropped {
            handle: image.clone(),
            rect: URect::from_corners(UVec2::ZERO, full_size),
            full_size,
        }
    } else {
        // take a sub image
        let crop: DynamicImage = full_image
            .view(
                crop_rect.min.x,
                crop_rect.min.y,
                crop_rect.width(),
                crop_rect.height(),
            )
            .to_image()
            .into();

        // convert to bevy image
        let crop = Image::from_dynamic(
            crop,
            input.texture_descriptor.format.is_srgb(),
            RenderAssetUsages::all(),
        );

        Cropped {
            handle: images.add(crop),
            rect: crop_rect,
            full_size,
        }
    };

    // also put that into the cache
    cache.insert(image.clone(), cropped.clone());

    Ok(cropped)
}

fn find_crop_rect<P, Container>(image: &ImageBuffer<P, Container>) -> URect
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]>,
{
    // get the first row that has a non-zero alpha component
    let min_y = image
        .rows()
        .enumerate()
        .find_map(|(y, mut pixels)| pixels.any(alpha_is_non_zero).then_some(y as u32))
        .unwrap_or_default();

    // get the last row that has a non-zero alpha component
    let max_y = image
        .rows()
        .enumerate()
        .rev()
        .find_map(|(y, mut pixels)| pixels.any(alpha_is_non_zero).then_some(y as u32))
        .unwrap_or_default();

    // get the first column that has a non zero alpha component
    let min_x = columns(image)
        .enumerate()
        .find_map(|(x, mut pixels)| pixels.any(alpha_is_non_zero).then_some(x as u32))
        .unwrap_or_default();

    // get the last column that has a non zero alpha component
    let max_x = columns(image)
        .enumerate()
        .rev()
        .find_map(|(x, mut pixels)| pixels.any(alpha_is_non_zero).then_some(x as u32))
        .unwrap_or_default();

    URect::new(min_x, min_y, max_x, max_y)
}

#[inline]
fn alpha_is_non_zero<P>(pixel: &P) -> bool
where
    P: Pixel,
{
    // assume full alpha by default
    let mut is_non_zero: bool = false;

    pixel.map_with_alpha(
        |value| value,
        |a| {
            is_non_zero = a != P::Subpixel::DEFAULT_MIN_VALUE;
            a
        },
    );

    is_non_zero
}

fn columns<P, Container>(
    image: &ImageBuffer<P, Container>,
) -> impl DoubleEndedIterator<Item = impl Iterator<Item = &P>> + ExactSizeIterator
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]>,
{
    let range = 0..image.width();

    range.map(move |x| {
        image
            .pixels()
            .skip(x as usize)
            .step_by(image.width() as usize)
    })
}
