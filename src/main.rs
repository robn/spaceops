extern crate three;
extern crate cgmath;
extern crate random_color;

use three::Object;
use cgmath::{Quaternion,Deg,Rotation3};
use std::f32;

const GAP: f32 = 1.6;
const NUM: u8 = 9;

fn main() {
    let mut win = three::Window::new("🚀 spaceops 🛰️");

    win.scene.background = three::Background::Color(0x000000);
    /*
    let backtex = win.factory.load_texture("/Users/robn/Downloads/22687644_702004036675566_4623325051301079301_n.jpg");
    win.scene.background = three::Background::Texture(backtex);
    */

    let camera = win.factory.perspective_camera(60.0, 1.0 .. 100.0);
    camera.look_at([0.0, 0.0, 5.0], [0.0, 0.0, 0.0], None);

    let meshes = make_objects(&mut win.factory, NUM);

    let group = win.factory.group();
    meshes.iter().for_each(|m| group.add(m));
    win.scene.add(&group);

    meshes.iter().zip(make_object_positions(NUM).iter()).for_each(|(m, p)| m.set_position(*p));

    let point_light = win.factory.point_light(0xffffff, 0.7);
    point_light.set_position([0.0, 0.0, 20.0]);
    win.scene.add(&point_light);

    let (mut rx, mut ry) = (0.0, 0.0);
    let (mut x, mut y) = (0.0, 0.0);

    while win.update() && !win.input.hit(three::KEY_ESCAPE) {

        if win.input.hit(three::Button::Mouse(three::MouseButton::Left)) {
            capture_mouse(&win);

            let (mx, my) = win.input.axes_movements().iter().fold((0.0,0.0), |a, &m| match m.0 {
                0 => (a.0 + m.1, a.1),
                1 => (a.0, a.1 + m.1),
                _ => unimplemented!(),
            });
            rx += mx;
            ry += my;
        }
        else {
            release_mouse(&win);
        }

        if win.input.hit(three::Key::A) {
            x += 0.1;
        }
        if win.input.hit(three::Key::D) {
            x -= 0.1;
        }
        if win.input.hit(three::Key::W) {
            y -= 0.1;
        }
        if win.input.hit(three::Key::S) {
            y += 0.1;
        }

        if win.input.hit(three::Key::R) {
            rx = 0.0;
            ry = 0.0;
            x = 0.0;
            y = 0.0;
        }

        let qx = Quaternion::from_angle_y(Deg(rx));
        let qy = Quaternion::from_angle_x(Deg(ry));
        group.set_orientation(qx*qy);

        group.set_position([x, y, 0.0]);

        win.render(&camera);
    }
}

fn capture_mouse(win: &three::Window) {
    win.glutin_window().grab_cursor(true).ok();
    win.glutin_window().hide_cursor(true);
}

fn release_mouse(win: &three::Window) {
    win.glutin_window().grab_cursor(false).ok();
    win.glutin_window().hide_cursor(false);
}

fn make_objects(factory: &mut three::Factory, n: u8) -> Vec<three::Mesh> {
    let geom = factory.upload_geometry(three::Geometry::cuboid(1.0, 1.0, 1.0));

    //println!("{:#?}", geom.base.vertices);
    /*
    geom.tex_coords = vec!(
        [0.0,0.0].into(),
        [0.0,1.0].into(),
        [1.0,1.0].into(),
        [1.0,0.0].into(),

        [0.0,0.0].into(),
        [0.0,1.0].into(),
        [1.0,1.0].into(),
        [1.0,0.0].into(),

        [0.0,0.0].into(),
        [0.0,1.0].into(),
        [1.0,1.0].into(),
        [1.0,0.0].into(),

        [0.0,0.0].into(),
        [0.0,1.0].into(),
        [1.0,1.0].into(),
        [1.0,0.0].into(),

        [0.0,0.0].into(),
        [0.0,1.0].into(),
        [1.0,1.0].into(),
        [1.0,0.0].into(),

        [0.0,0.0].into(),
        [0.0,1.0].into(),
        [1.0,1.0].into(),
        [1.0,0.0].into(),
    );
    */

    /*
    let mat = three::material::Lambert {
        color: 0x33aa33,
        flat: false,
    };
    */
    /*
    let mat = three::material::Phong {
        color: 0x33aa33,
        glossiness: 10.0,
    };
    */

    /*
    let map = factory.load_texture("/Users/robn/Downloads/roll_safe.jpg");
    let mat = three::material::Basic {
        color: 0xffffff,
        map: Some(map),
    };
    */

    (0u8..n).map(|_|
                 factory.create_instanced_mesh(&geom,
                                               three::material::Lambert {
                                                   color: {
                                                       let c = random_color::RandomColor::new().to_rgb_array();
                                                        c[0] << 16 | c[1] << 8 | c[2]
                                                   },
                                                   flat: false,
                                               })
                 ).collect()

    /*
    let group = factory.group();

    let off = 1.6 * ((n as f32) - 1.0) * 0.5;
    for x in 0u8..n {
        for y in 0u8..n {
            let m = factory.mesh_instance(&mesh);
            m.set_position([f32::from(x) * 1.6 - off, f32::from(y) * 1.6 - off, 0.0]);
            group.add(&m);
        }
    }

    group
    */
}

fn make_object_positions(n: u8) -> Vec<[f32; 3]> {
    let w: u8 = f32::from(n).sqrt().ceil() as u8;
    let off = GAP * ((w as f32) - 1.0) * 0.5;
    let v = (0u8..n).fold(vec!(), |mut vv, nn| {
        vv.push([f32::from(nn % w) * GAP - off, f32::from(nn / w) * GAP - off, 0.0]);
        vv
    });
    //let nx: u8 = 0;
    //let nx: u8 = n - ny * ny;
    //println!("{}: {} {}", n, nx, ny);
    println!("{:?}", v);
    v
}
