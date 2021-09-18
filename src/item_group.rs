// a group of items
// each one has a name, and a wrapped mesh
// when we add one, the meshes get repositioned
// removing, same diff
// ordered by name I guess

use crate::wrapped_mesh::WrappedMesh;

use std::cmp::Ordering;
use std::collections::BTreeMap;

const GAP: f32 = 1.6;

pub struct Item {
    name: String,
    wm: WrappedMesh,
}

impl Ord for Item {
    fn cmp(&self, other: &Item) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Item) -> bool {
        self.name == other.name
    }
}

impl Eq for Item { }

pub struct ItemGroup {
    geom: three::template::InstancedGeometry,

    items: BTreeMap<String,Item>,
}

impl ItemGroup {
    pub fn new(factory: &mut three::Factory) -> ItemGroup {
        let geom = factory.upload_geometry(three::Geometry::cuboid(1.0, 1.0, 1.0));

        ItemGroup {
            geom: geom,
            items: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, scene: &mut three::Scene, factory: &mut three::Factory, name: &str) {
        let mat = three::material::Lambert {
            color: {
                let c = random_color::RandomColor::new().to_rgb_array();
                c[0] << 16 | c[1] << 8 | c[2]
            },
            flat: false,
        };
        let wm = WrappedMesh::new(factory.create_instanced_mesh(&self.geom, mat));
        let item = Item {
            name: name.into(),
            wm: wm,
        };
        scene.add(&item.wm.mesh);
        self.items.insert(item.name.clone(), item);
        self.layout();
    }

    pub fn remove(&mut self, scene: &mut three::Scene, name: &str) {
        if let Some(item) = self.items.get(name) {
            scene.remove(&item.wm.mesh);
        }
        self.items.remove(name);
        self.layout();
    }

    fn layout(&mut self) {
        let num = self.items.len();
        if num == 0 { return }
        
        let positions = self.make_object_positions(num);

        self.items.values_mut().zip(positions.iter()).for_each(|(m, p)| m.wm.set_position(*p));
    }

    fn make_object_positions(&self, n: usize) -> Vec<[f32; 3]> {
        let w: u16 = f32::from(n as u16).sqrt().ceil() as u16;
        let off = GAP * ((w as f32) - 1.0) * 0.5;
        (0u16..n as u16).fold(vec!(), |mut vv, nn| {
            vv.push([f32::from(nn % w) * GAP - off, f32::from(nn / w) * GAP - off, 0.0]);
            vv
        })
    }
}
