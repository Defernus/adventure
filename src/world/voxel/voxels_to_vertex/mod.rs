use crate::{utils::position::Position, vec::Vec3, vertex::Vertex, world::chunk::Chunk};

use self::triangulation_table::{get_index_by_voxels, TABLE};

use super::Voxel;

mod triangulation_table;

#[derive(Clone, Copy)]
struct VertexNode {
    index: usize,
    pos: Vec3<f32>,
}

const DN: VertexNode = VertexNode {
    index: 0,
    pos: Vec3::new(0.5, 0.0, 1.0),
};
const DE: VertexNode = VertexNode {
    index: 1,
    pos: Vec3::new(1.0, 0.0, 0.5),
};
const DS: VertexNode = VertexNode {
    index: 2,
    pos: Vec3::new(0.5, 0.0, 0.0),
};
const DW: VertexNode = VertexNode {
    index: 3,
    pos: Vec3::new(0.0, 0.0, 0.5),
};

const UN: VertexNode = VertexNode {
    index: 4,
    pos: Vec3::new(0.5, 1.0, 1.0),
};
const UE: VertexNode = VertexNode {
    index: 5,
    pos: Vec3::new(1.0, 1.0, 0.5),
};
const US: VertexNode = VertexNode {
    index: 6,
    pos: Vec3::new(0.5, 1.0, 0.0),
};
const UW: VertexNode = VertexNode {
    index: 7,
    pos: Vec3::new(0.0, 1.0, 0.5),
};

const NW: VertexNode = VertexNode {
    index: 8,
    pos: Vec3::new(0.0, 0.5, 1.0),
};
const NE: VertexNode = VertexNode {
    index: 9,
    pos: Vec3::new(1.0, 0.5, 1.0),
};
const SE: VertexNode = VertexNode {
    index: 10,
    pos: Vec3::new(1.0, 0.5, 0.0),
};
const SW: VertexNode = VertexNode {
    index: 11,
    pos: Vec3::new(0.0, 0.5, 0.0),
};

const NODES_POS_COUNT: usize = 12;
const NODES: [VertexNode; NODES_POS_COUNT] = [DN, DE, DS, DW, UN, UE, US, UW, NW, NE, SE, SW];

type Nodes = [Voxel; NODES_POS_COUNT];
type VoxelsBlock = [[[Voxel; 2]; 2]; 2];

fn get_voxel(chunk: &Chunk, pos: Position) -> Voxel {
    match chunk.get_voxel(pos) {
        Some(voxel) => voxel,
        _ => Voxel { id: 0, value: 0. },
    }
}

fn get_voxels_for_vertex(chunk: &Chunk, base_pos: Position) -> VoxelsBlock {
    let voxels: [[[Voxel; 2]; 2]; 2] = [
        [
            [
                get_voxel(chunk, base_pos + Position::new(0, 0, 0)),
                get_voxel(chunk, base_pos + Position::new(0, 0, 1)),
            ],
            [
                get_voxel(chunk, base_pos + Position::new(0, 1, 0)),
                get_voxel(chunk, base_pos + Position::new(0, 1, 1)),
            ],
        ],
        [
            [
                get_voxel(chunk, base_pos + Position::new(1, 0, 0)),
                get_voxel(chunk, base_pos + Position::new(1, 0, 1)),
            ],
            [
                get_voxel(chunk, base_pos + Position::new(1, 1, 0)),
                get_voxel(chunk, base_pos + Position::new(1, 1, 1)),
            ],
        ],
    ];
    return voxels;
}

fn chose_voxel_for_node(a: Voxel, b: Voxel) -> Voxel {
    if a.id == 0 {
        return Voxel {
            id: b.id,
            value: (-a.value) / (b.value - a.value),
        };
    }
    if b.id == 0 {
        return Voxel {
            id: a.id,
            value: 1.0 - (-b.value) / (a.value - b.value),
        };
    }
    return Voxel { id: 0, value: 0. };
}

fn get_vertex_nodes(voxels: VoxelsBlock) -> Nodes {
    let mut result: Nodes = [Voxel { id: 0, value: 0. }; NODES_POS_COUNT];

    result[DS.index] = chose_voxel_for_node(voxels[0][0][0], voxels[1][0][0]);
    result[DE.index] = chose_voxel_for_node(voxels[1][0][0], voxels[1][0][1]);
    result[DN.index] = chose_voxel_for_node(voxels[0][0][1], voxels[1][0][1]);
    result[DW.index] = chose_voxel_for_node(voxels[0][0][0], voxels[0][0][1]);

    result[NE.index] = chose_voxel_for_node(voxels[1][0][1], voxels[1][1][1]);
    result[NW.index] = chose_voxel_for_node(voxels[0][0][1], voxels[0][1][1]);
    result[SE.index] = chose_voxel_for_node(voxels[1][0][0], voxels[1][1][0]);
    result[SW.index] = chose_voxel_for_node(voxels[0][0][0], voxels[0][1][0]);

    result[US.index] = chose_voxel_for_node(voxels[0][1][0], voxels[1][1][0]);
    result[UE.index] = chose_voxel_for_node(voxels[1][1][0], voxels[1][1][1]);
    result[UN.index] = chose_voxel_for_node(voxels[0][1][1], voxels[1][1][1]);
    result[UW.index] = chose_voxel_for_node(voxels[0][1][0], voxels[0][1][1]);

    return result;
}

fn shift_node_pos(pos: Vec3<f32>, value: f32) -> Vec3<f32> {
    if pos.x == 0.5 {
        return Vec3::new(value, pos.y, pos.z);
    }
    if pos.y == 0.5 {
        return Vec3::new(pos.x, value, pos.z);
    }
    if pos.z == 0.5 {
        return Vec3::new(pos.x, pos.y, value);
    }

    panic!("failed to process pos {:?}", pos);
}

fn append_triangle(
    pos: Position,
    vertex: &mut Vec<Vertex>,
    nodes: Nodes,
    a: VertexNode,
    b: VertexNode,
    c: VertexNode,
) {
    let a_v = nodes[a.index].value;
    let b_v = nodes[b.index].value;
    let c_v = nodes[c.index].value;

    if a_v < 0. || a_v < 0. || c_v < 0. {
        return;
    }

    let pos_vec = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32);

    let a_pos = shift_node_pos(a.pos, a_v) + pos_vec;
    let b_pos = shift_node_pos(b.pos, b_v) + pos_vec;
    let c_pos = shift_node_pos(c.pos, c_v) + pos_vec;

    let a_color: [f32; 3] = [0.9; 3];
    let b_color: [f32; 3] = [0.9; 3];
    let c_color: [f32; 3] = [0.9; 3];

    let normal_vec = (c_pos - a_pos).cross(b_pos - a_pos).normalize();
    let normal: [f32; 3] = [normal_vec.x, normal_vec.y, normal_vec.z];

    vertex.push(Vertex {
        color: a_color,
        normal,
        position: c_pos.to_arr(),
    });
    vertex.push(Vertex {
        color: b_color,
        normal,
        position: b_pos.to_arr(),
    });
    vertex.push(Vertex {
        color: c_color,
        normal,
        position: a_pos.to_arr(),
    });
}

pub fn append_vertex(pos: Position, chunk: &Chunk, vertex: &mut Vec<Vertex>) {
    let voxels = get_voxels_for_vertex(chunk, pos);
    let nodes = get_vertex_nodes(voxels);

    let triangle_points = TABLE[get_index_by_voxels(voxels)];

    let mut triangle_offset = 0;

    while triangle_points[triangle_offset] != -1 {
        let a = NODES[triangle_points[triangle_offset] as usize];
        let b = NODES[triangle_points[triangle_offset + 1] as usize];
        let c = NODES[triangle_points[triangle_offset + 2] as usize];

        append_triangle(pos, vertex, nodes, a, b, c);

        triangle_offset += 3;
    }
}
