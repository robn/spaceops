use hecs::*;
use three::Object;
use cgmath::{Quaternion,Deg,Rotation3};

mod component;
mod animator;
mod wrapped_mesh;
mod item_group;

use crate::component::*;
use crate::animator::PositionAnimator;
use crate::wrapped_mesh::WrappedMesh;
use crate::item_group::ItemGroup;

fn main() {
    let mut world = World::new();

    let mut win = three::Window::new("🚀 spaceops 🛰️");

    win.scene.background = three::Background::Color(0x000000);

    let camera = win.factory.perspective_camera(60.0, 1.0 .. 100.0);

    let mut item_group = ItemGroup::new(&mut win.factory);

    let point_light = win.factory.point_light(0xffffff, 0.7);
    point_light.set_position([0.0, 0.0, 20.0]);
    win.scene.add(&point_light);

    let (mut rx, mut ry) = (0.0, 0.0);
    let (mut x, mut y) = (0.0, 0.0);

    //let mut anim = PositionAnimator::new([1.0,1.0,1.0].into(), [3.0,-3.0,2.0].into(), 3.0);
    let mut entities = vec!();

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

        if win.input.hit(three::Key::Z) {
            let e = world.spawn((Item,));
            entities.push(e);
            item_group.add(&mut win.scene, &mut win.factory, &e.id().to_string());
        }
        if win.input.hit(three::Key::X) {
            if let Some(e) = entities.pop() {
                item_group.remove(&mut win.scene, &e.id().to_string());
                world.despawn(e);
            }
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
        //item_group.group.set_orientation(qx*qy);
        camera.set_orientation(qx*qy);

        //item_group.group.set_position([x, y, 0.0]);
        camera.set_position([x, y, 5.0]);

        //meshes[0].set_position(anim.next_position());

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
