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
    pub group: three::Group,

    items: BTreeMap<String,Item>,

    gap: f32,
}

impl ItemGroup {
    pub fn new(factory: &mut three::Factory) -> ItemGroup {
        let geom = factory.upload_geometry(three::Geometry::cuboid(1.0, 1.0, 1.0));
        let group = factory.group();

        ItemGroup {
            geom: geom,
            group: group,
            items: BTreeMap::new(),
            gap: GAP,
        }
    }

    pub fn add(&mut self, factory: &mut three::Factory, name: &str) {
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
        self.group.add(&item.wm.mesh);
        self.items.insert(item.name.clone(), item);
        self.layout();
    }

    pub fn remove(&mut self, name: &str) {
        if let Some(item) = self.items.get(name) {
            self.group.remove(&item.wm.mesh);
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
        let off = self.gap * ((w as f32) - 1.0) * 0.5;
        (0u16..n as u16).fold(vec!(), |mut vv, nn| {
            vv.push([f32::from(nn % w) * self.gap - off, f32::from(nn / w) * self.gap - off, 0.0]);
            vv
        })
    }
}