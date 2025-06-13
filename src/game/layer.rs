use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(transfer_layer_to_z);
}

#[derive(Component)]
#[require(Transform)]
pub struct Layer(pub f32);

impl Layer {
    pub const fn offset_by(&self, offset: i32) -> Self {
        Layer(self.0 + 0.1 * offset as f32)
    }
}

fn transfer_layer_to_z(
    trigger: Trigger<OnInsert, Layer>,
    query: Query<(&mut Transform, &Layer, Option<&Name>)>,
) -> Result {
    let (mut transform, layer, name) = query.get_inner(trigger.target())?;
    debug!("Applying Z of {} to {:?}", layer.0, name);

    transform.translation.z = layer.0;
    Ok(())
}
