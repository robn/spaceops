extern crate three;
extern crate cgmath;

use three::Object;
use cgmath::{Quaternion,Deg,Rotation3};
use std::f32;

fn main() {
    let mut win = three::Window::new("🚀 spaceops 🛰️");

    win.scene.background = three::Background::Color(0x000000);
    /*
    let backtex = win.factory.load_texture("/Users/robn/Downloads/22687644_702004036675566_4623325051301079301_n.jpg");
    win.scene.background = three::Background::Texture(backtex);
    */

    let camera = win.factory.perspective_camera(60.0, 1.0 .. 100.0);
    camera.look_at([0.0, 0.0, 5.0], [0.0, 0.0, 0.0], None);

    let group = make_cube_grid(&mut win.factory, 3);
    win.scene.add(&group);

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

fn make_cube_grid(factory: &mut three::Factory, n: u8) -> three::Group {
    let mut geom = three::Geometry::cuboid(1.0, 1.0, 1.0);
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

    let mat = three::material::Lambert {
        color: 0x33aa33,
        flat: false,
    };
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

    let mesh = factory.mesh(geom, mat);

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
}
