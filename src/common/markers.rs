use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_markers);
}

pub enum MarkerTarget {
    Static { position: Vec2 },
    Dynamic { target: Entity, offset: Vec2 },
}

#[derive(Component)]
pub struct Marker {
    pub follow: Entity,
    pub target: MarkerTarget,
    pub offset: f32,
    pub offset_z: f32,
}

fn update_markers(
    mut markers: Query<(&mut Transform, &Marker)>,
    transforms: Query<&Transform, Without<Marker>>,
) {
    for (mut marker_transform, marker) in markers.iter_mut() {
        let target = match &marker.target {
            MarkerTarget::Static { position } => *position,
            MarkerTarget::Dynamic { target, offset } => {
                let Ok(transform) = transforms.get(*target) else {
                    continue;
                };

                transform.translation.xy() + *offset
            }
        };

        let direction = (target - marker_transform.translation.xy()).normalize();
        let offset = (direction * marker.offset).extend(marker.offset_z);

        marker_transform.translation = (marker_transform.translation + offset);
        marker_transform.rotation = Quat::from_rotation_z(direction.to_angle());
    }
}
