use three::{Object,Mesh};
use cgmath::{Point3,Quaternion};

#[derive(Clone,Debug)]
pub struct WrappedMesh {
    pub mesh: Mesh,

    position:    Point3<f32>,
    orientation: Quaternion<f32>,
    scale:       f32,
}

impl WrappedMesh {
    pub fn new(mesh: Mesh) -> WrappedMesh {
        let wm = WrappedMesh {
            mesh: mesh,
            position:    [0.0,0.0,0.0].into(),
            orientation: [1.0,0.0,0.0,0.0].into(),
            scale:       1.0,
        };
        wm.mesh.set_position(wm.position);
        wm.mesh.set_orientation(wm.orientation);
        wm.mesh.set_scale(wm.scale);
        wm
    }

    pub fn set_position<P>(&mut self, position: P) where P: Into<Point3<f32>>, P: Copy {
        self.position = position.into();
        self.mesh.set_position(position.into());
    }

    pub fn set_orientation(&mut self, orientation: Quaternion<f32>) {
        self.orientation = orientation;
        self.mesh.set_orientation(orientation);
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.mesh.set_scale(scale);
    }
}
