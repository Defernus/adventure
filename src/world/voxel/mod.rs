pub mod voxel_data;
pub mod voxels_to_vertex;

pub type VoxelId = u32;

#[derive(Clone, Copy)]
pub struct Voxel {
    pub value: f32,
    pub id: VoxelId,
}
