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

    (0u8..n).map(|_| {
        let mat = three::material::Lambert {
            color: {
                let c = random_color::RandomColor::new().to_rgb_array();
                c[0] << 16 | c[1] << 8 | c[2]
            },
            flat: false,
        };
        factory.create_instanced_mesh(&geom, mat)
    }).collect()
}

fn make_object_positions(n: u8) -> Vec<[f32; 3]> {
    let w: u8 = f32::from(n).sqrt().ceil() as u8;
    let off = GAP * ((w as f32) - 1.0) * 0.5;
    (0u8..n).fold(vec!(), |mut vv, nn| {
        vv.push([f32::from(nn % w) * GAP - off, f32::from(nn / w) * GAP - off, 0.0]);
        vv
    })
}
