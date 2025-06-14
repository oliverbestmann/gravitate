use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use image::{GrayAlphaImage, LumaA, Rgba, RgbaImage, imageops};
use std::collections::HashMap;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ShadowCache>();
    app.add_observer(add_shadow_to_entity);
    app.add_observer(remove_shadow_from_entry);
}

#[derive(Component, Reflect)]
pub struct Shadow {
    pub offset_z: f32,
    pub sigma: f32,
}

#[derive(Component, Reflect)]
#[relationship(relationship_target = Shadows)]
struct ShadowOf(Entity);

#[derive(Component, Reflect)]
#[relationship_target(relationship = ShadowOf)]
struct Shadows(Vec<Entity>);

#[derive(Resource, Default)]
struct ShadowCache {
    images: HashMap<Handle<Image>, Handle<Image>>,
}

impl Default for Shadow {
    fn default() -> Self {
        Shadow {
            offset_z: -0.01,
            sigma: 3.0,
        }
    }
}

fn remove_shadow_from_entry(
    trigger: Trigger<OnRemove, Shadow>,
    mut commands: Commands,
    shadows: Query<&Shadows>,
) {
    let Ok(shadows) = shadows.get(trigger.target()) else {
        return;
    };

    for shadow in shadows.iter() {
        // try to remove the shadow if it still exists
        commands.entity(shadow).try_despawn();
    }
}

fn add_shadow_to_entity(
    trigger: Trigger<OnAdd, Shadow>,
    query_sprite: Query<(&Sprite, &Shadow)>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut cache: ResMut<ShadowCache>,
) -> Result {
    let (sprite, shadow_desc) = query_sprite.get(trigger.target())?;

    if sprite.anchor != Anchor::Center {
        return Err("anchor must be center for Shadow".into());
    }

    let shadow = match cache
        .images
        .get(&sprite.image)
        .and_then(|handle| images.get_strong_handle(handle.into()))
    {
        Some(handle) => handle,
        None => {
            #[cfg(not(target_family = "wasm"))]
            let start = std::time::Instant::now();

            // we could not get a strong handle to the cached shadow.
            // need to create a new shadow from the image
            let image = images.get(&sprite.image).ok_or("image not found")?;

            // convert & create shadow
            let image = image.clone().try_into_dynamic()?;
            let shadow = generate_shadow_from_alpha(image.into_rgba8(), shadow_desc.sigma)?;

            // create a bevy image from the shadow
            let shadow = Image::from_dynamic(shadow.into(), true, RenderAssetUsages::RENDER_WORLD);

            #[cfg(not(target_family = "wasm"))]
            {
                let duration = std::time::Instant::now().duration_since(start);
                info!(
                    "Creating shadow of size {}x{} took {:?}",
                    shadow.width(),
                    shadow.height(),
                    duration
                );
            }

            let handle = images.add(shadow);

            // cache a weak clone of the handle
            cache
                .images
                .insert(sprite.image.clone_weak(), handle.clone_weak());

            handle
        }
    };

    // spawn a new sprite as child
    commands.entity(trigger.target()).with_child((
        Name::new("Shadow"),
        ShadowOf(trigger.target()),
        Sprite {
            image: shadow,
            custom_size: sprite.custom_size.map(|size| 2.0 * size),
            anchor: Anchor::Center,
            ..default()
        },
        Transform::from_xyz(0., 0., shadow_desc.offset_z),
    ));

    Ok(())
}

fn generate_shadow_from_alpha(image: RgbaImage, sigma: f32) -> Result<GrayAlphaImage> {
    // start with a transparent black image
    let mut mask = GrayAlphaImage::new(image.width() * 2, image.height() * 2);

    let offset_x = image.width() / 2;
    let offset_y = image.height() / 2;

    // copy alpha channel into the center of the mask image
    for (src, dst) in image.rows().zip(mask.rows_mut().skip(offset_y as usize)) {
        for (Rgba(sp), LumaA(dp)) in src.zip(dst.skip(offset_x as usize)) {
            dp[1] = sp[3];
        }
    }

    // increase the size of the mask by first applying a small blur
    let mut mask = imageops::fast_blur(&mask, 1.0);

    // and then using a threshold
    mask.pixels_mut()
        .for_each(|LumaA([_, px])| *px = px.saturating_mul(5));

    // now blur the mask to create the base shadow
    let mut shadow = GrayAlphaImage::new(mask.width(), mask.height());

    add(&mut shadow, &imageops::fast_blur(&mask, sigma), 2);

    // reduce influence of the mask for pixels with wider shadow blur
    add(&mut shadow, &imageops::fast_blur(&mask, sigma * 2.0), 3);

    // reduce influence of the mask for pixels with wider shadow blur
    add(&mut shadow, &imageops::fast_blur(&mask, sigma * 4.0), 4);

    Ok(shadow)
}

#[inline]
fn add(target: &mut GrayAlphaImage, source: &GrayAlphaImage, divider: u8) {
    for (LumaA([_, tp]), LumaA([_, sp])) in target.pixels_mut().zip(source.pixels()) {
        *tp = tp.saturating_add(*sp / divider);
    }
}
