use crate::{
    utils::{
        direction::Direction,
        position::Position,
    },
    vertex::Vertex,
};

pub fn append_face(vertex: &mut Vec<Vertex>, pos: Position, color: [f32; 3], direction: Direction) {
    let x = pos.x as f32;
    let y = pos.y as f32;
    let z = pos.z as f32;

    match direction {
        Direction::North => {
            vertex.push(Vertex {
                position: [x - 0.5, y - 0.5, z + 0.5],
                normal: [0.0, 0.0, 1.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y - 0.5, z + 0.5],
                normal: [0.0, 0.0, 1.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y + 0.5, z + 0.5],
                normal: [0.0, 0.0, 1.0],
                color,
            });

            vertex.push(Vertex {
                position: [x - 0.5, y - 0.5, z + 0.5],
                normal: [0.0, 0.0, 1.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y + 0.5, z + 0.5],
                normal: [0.0, 0.0, 1.0],
                color,
            });
            vertex.push(Vertex {
                position: [x - 0.5, y + 0.5, z + 0.5],
                normal: [0.0, 0.0, 1.0],
                color,
            });
        }
        Direction::South => {
            vertex.push(Vertex {
                position: [x - 0.5, y - 0.5, z - 0.5],
                normal: [0.0, 0.0, -1.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y + 0.5, z - 0.5],
                normal: [0.0, 0.0, -1.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y - 0.5, z - 0.5],
                normal: [0.0, 0.0, -1.0],
                color,
            });

            vertex.push(Vertex {
                position: [x - 0.5, y - 0.5, z - 0.5],
                normal: [0.0, 0.0, -1.0],
                color,
            });
            vertex.push(Vertex {
                position: [x - 0.5, y + 0.5, z - 0.5],
                normal: [0.0, 0.0, -1.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y + 0.5, z - 0.5],
                normal: [0.0, 0.0, -1.0],
                color,
            });
        }

        Direction::East => {
            vertex.push(Vertex {
                position: [x + 0.5, y - 0.5, z - 0.5],
                normal: [1.0, 0.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y + 0.5, z - 0.5],
                normal: [1.0, 0.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y + 0.5, z + 0.5],
                normal: [1.0, 0.0, 0.0],
                color,
            });

            vertex.push(Vertex {
                position: [x + 0.5, y - 0.5, z - 0.5],
                normal: [1.0, 0.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y + 0.5, z + 0.5],
                normal: [1.0, 0.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y - 0.5, z + 0.5],
                normal: [1.0, 0.0, 0.0],
                color,
            });
        }
        Direction::West => {
            vertex.push(Vertex {
                position: [x - 0.5, y - 0.5, z - 0.5],
                normal: [-1.0, 0.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x - 0.5, y + 0.5, z + 0.5],
                normal: [-1.0, 0.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x - 0.5, y + 0.5, z - 0.5],
                normal: [-1.0, 0.0, 0.0],
                color,
            });

            vertex.push(Vertex {
                position: [x - 0.5, y - 0.5, z - 0.5],
                normal: [-1.0, 0.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x - 0.5, y - 0.5, z + 0.5],
                normal: [-1.0, 0.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x - 0.5, y + 0.5, z + 0.5],
                normal: [-1.0, 0.0, 0.0],
                color,
            });
        }

        Direction::Up => {
            vertex.push(Vertex {
                position: [x - 0.5, y + 0.5, z - 0.5],
                normal: [0.0, 1.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y + 0.5, z + 0.5],
                normal: [0.0, 1.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y + 0.5, z - 0.5],
                normal: [0.0, 1.0, 0.0],
                color,
            });

            vertex.push(Vertex {
                position: [x - 0.5, y + 0.5, z - 0.5],
                normal: [0.0, 1.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x - 0.5, y + 0.5, z + 0.5],
                normal: [0.0, 1.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y + 0.5, z + 0.5],
                normal: [0.0, 1.0, 0.0],
                color,
            });
        }
        Direction::Down => {
            vertex.push(Vertex {
                position: [x - 0.5, y - 0.5, z - 0.5],
                normal: [0.0, -1.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y - 0.5, z - 0.5],
                normal: [0.0, -1.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y - 0.5, z + 0.5],
                normal: [0.0, -1.0, 0.0],
                color,
            });

            vertex.push(Vertex {
                position: [x - 0.5, y - 0.5, z - 0.5],
                normal: [0.0, -1.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x + 0.5, y - 0.5, z + 0.5],
                normal: [0.0, -1.0, 0.0],
                color,
            });
            vertex.push(Vertex {
                position: [x - 0.5, y - 0.5, z + 0.5],
                normal: [0.0, -1.0, 0.0],
                color,
            });
        }
        _ => {}
    }
}
