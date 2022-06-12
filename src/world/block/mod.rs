pub mod block_data;
pub mod block_handlers;

pub type BlockId = u32;

#[derive(Clone, Copy)]
pub struct Block {
    pub value: f32,
    pub id: BlockId,
}
