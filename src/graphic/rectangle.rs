use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 3],
    tex_coords: [f32; 2],
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    vertices: [Vertex; 4],
}

impl Rectangle {
    pub const NUM_INDICES: u32 = Self::INDICES.len() as u32;

    #[rustfmt::skip]
    const INDICES: &[u16] = &[
        0, 1, 2,
        2, 3, 0,
    ];

    const RIGHT_TOP_CORNER: Vertex = Vertex {
        pos: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    };

    const LEFT_TOP_CORNER: Vertex = Vertex {
        pos: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    };

    const LEFT_BOTTOM_CORNER: Vertex = Vertex {
        pos: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    };

    const RIGHT_BOTTOM_CORNER: Vertex = Vertex {
        pos: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    };

    pub fn new() -> Self {
        let vertices = [
            Self::RIGHT_TOP_CORNER,
            Self::LEFT_TOP_CORNER,
            Self::LEFT_BOTTOM_CORNER,
            Self::RIGHT_BOTTOM_CORNER,
        ];

        Self { vertices }
    }

    pub fn get_vbuffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    pub fn get_ibuffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&Self::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        })
    }
}
