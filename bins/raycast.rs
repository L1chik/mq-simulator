use macroquad::prelude::*;

use crate::raycast::*;

struct MyCamera {
    position: Vec3,
    local: Mat3,
}


#[macroquad::main("3D")]
async fn main() {
    let world_axes = Mat3::from_cols(
        vec3(1., 0., 0.),
        vec3(0., 1., 0.),
        vec3(0., 0., 1.)
    );
    let mut main_camera = MyCamera{
        position: vec3(0., 0., -10.),
        local: world_axes, 
    };
    let mut mouse_right_is_pressed = false;
    let mut mouse_middle_is_pressed = false;
    let mut mouse_left_is_pressed = false;
    let mut last_mouse_pos: Vec2 = mouse_position().into();
    let mut mouse_x = vec3(0., 0., 0.);
    let mut new_x = vec3(0., 0., 0.);
    let mut mp = vec2(0., 0.);
    loop {
        let delta = get_frame_time();
        let speed = 0.2; 
        //Controls
        
        if is_mouse_button_down(MouseButton::Middle) {
            if mouse_middle_is_pressed {
                //Hold
                let mouse_pos: Vec2  = mouse_position().into();
                let delta_mouse_pos = mouse_pos - last_mouse_pos;
                last_mouse_pos = mouse_pos;
                mouse_x = vec3(delta_mouse_pos.x, delta_mouse_pos.y, 0.);
                let transition = world_axes.inverse().mul_mat3(&main_camera.local);
                new_x = transition.mul_vec3(mouse_x);
                main_camera.position += new_x * delta;
            }else {
                //Press
                mouse_middle_is_pressed = true;
                last_mouse_pos = mouse_position().into();
            }
        } else {
            if mouse_middle_is_pressed {
                //Release
                mouse_middle_is_pressed = false;
            }else {
                //Not pressed
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            if mouse_left_is_pressed {
                //Hold
            }else {
                //Press
                mouse_left_is_pressed = true;
                let mouse_pos: Vec2 = mouse_position().into();
                mp = vec2(mouse_pos.x - (screen_width() / 2.), (screen_height() / 2.) - mouse_pos.y);
                cast();
            }
        } else {
            if mouse_left_is_pressed {
                //Release
                mouse_left_is_pressed = false;
            }else {
                //Not pressed
            }
        }

        if is_mouse_button_down(MouseButton::Right) {
            if mouse_right_is_pressed {
                //Hold
                let mouse_pos: Vec2  = mouse_position().into();
                let delta_mouse_pos = last_mouse_pos - mouse_pos;
                last_mouse_pos = mouse_pos;
                let angle = delta_mouse_pos * speed * delta;
                let rotate_mat = Mat3::from_axis_angle(main_camera.local.x_axis, angle.y * -1.);
                let rotate_mat2 = Mat3::from_axis_angle(world_axes.y_axis, angle.x);
                let sum_rotate = rotate_mat.mul_mat3(&rotate_mat2);
                //let rot = Quat::from_rotation_ypr(angle.x * -1., angle.y, 0.);
                
                main_camera.local.z_axis = sum_rotate.mul_vec3(main_camera.local.z_axis).normalize();
                main_camera.local.x_axis = world_axes.y_axis.cross(main_camera.local.z_axis).normalize();
                main_camera.local.y_axis = main_camera.local.z_axis.cross(main_camera.local.x_axis).normalize();
                //main_camera.local = sum_rotate.mul_mat3(&main_camera.local); 
                //main_camera.local.y_axis = rotate_mat.mul_vec3(main_camera.local.y_axis).normalize();
                //main_camera.local.z_axis = rotate_mat.mul_vec3(main_camera.local.z_axis).normalize();
                //main_camera.local.x_axis = rotate_mat2.mul_vec3(main_camera.local.x_axis).normalize();
                //main_camera.local.z_axis = rotate_mat2.mul_vec3(main_camera.local.z_axis).normalize();
            }else {
                //Press
                mouse_right_is_pressed = true;
                last_mouse_pos = mouse_position().into();
                //tmp_axes = main_camera.local;
            }
        } else {
            if mouse_right_is_pressed {
                //Release
                mouse_right_is_pressed = false;
                //main_camera.local = tmp_axes;
            }else {
                //Not pressed
            }
        }
 
        //Change


        //draw
        clear_background(LIGHTGRAY);
        
        //zoom
        main_camera.position += main_camera.local.z_axis* mouse_wheel().1;
        //rotate

        //look_at

        set_camera(&Camera3D {
            position: main_camera.position,
            up: main_camera.local.y_axis,
            target: main_camera.position + main_camera.local.z_axis,
            //aspect: Some(screen_width() / screen_height()),
            //viewport: Some((0, 0, screen_width() as i32 - 200, screen_height() as i32 - 100)),
            ..Default::default()
        });

        draw_grid(50, 10., BLACK, GRAY);

        draw_cube(vec3(0., 2.5, 0.), vec3(5., 5., 5.), None, GRAY);
        draw_cube(vec3(0., 0., -5.), vec3(5., 5., 5.), None, GRAY);
        
        set_default_camera();
        draw_text(&*get_fps().to_string(), 10., 20., 32., BLACK);
        draw_text(&*format!("{}, {}, {}", world_axes.x_axis.x, world_axes.x_axis.y, world_axes.x_axis.z), 10., 50., 16., BLACK);
        draw_text(&*format!("{}, {}, {}", world_axes.y_axis.x, world_axes.y_axis.y, world_axes.y_axis.z), 10., 80., 16., BLACK);
        draw_text(&*format!("{}, {}, {}", world_axes.z_axis.x, world_axes.z_axis.y, world_axes.z_axis.z), 10., 110., 16., BLACK);
        draw_text(&*format!("{}, {}, {}", mouse_x.x, mouse_x.y, mouse_x.z), 10., 150., 16., BLACK);
        draw_text(&*format!("{}, {}, {}", main_camera.local.x_axis.x, main_camera.local.x_axis.y, main_camera.local.x_axis.z), 200., 50., 16., BLACK);
        draw_text(&*format!("{}, {}, {}", main_camera.local.y_axis.x, main_camera.local.y_axis.y, main_camera.local.y_axis.z), 200., 80., 16., BLACK);
        draw_text(&*format!("{}, {}, {}", main_camera.local.z_axis.x, main_camera.local.z_axis.y, main_camera.local.z_axis.z), 200., 110., 16., BLACK);
        draw_text(&*format!("{}, {}, {}", new_x.x, new_x.y, new_x.z), 500., 150., 16., BLACK);
        draw_text(&*format!("|x| = {}, |y| = {}, |z| = {}", main_camera.local.x_axis.length(), main_camera.local.y_axis.length(), main_camera.local.z_axis.length()), 800., 50., 16., BLACK);
        draw_text(&*format!("x*y={}, x*z={},", main_camera.local.x_axis.dot(main_camera.local.y_axis), main_camera.local.x_axis.dot(main_camera.local.z_axis)), 800., 80., 16., BLACK);
        draw_text(&*format!("y*x={}, y*z={},", main_camera.local.y_axis.dot(main_camera.local.x_axis), main_camera.local.y_axis.dot(main_camera.local.z_axis)), 800., 110., 16., BLACK);
        draw_text(&*format!("z*x={}, z*y={},", main_camera.local.z_axis.dot(main_camera.local.x_axis), main_camera.local.z_axis.dot(main_camera.local.y_axis)), 800., 140., 16., BLACK);
        draw_text(&*format!("{}", mp), 10., 200., 16., BLACK);
        draw_text(&*format!("{:?}", Camera3D {
            position: main_camera.position,
            up: main_camera.local.y_axis,
            target: main_camera.position + main_camera.local.z_axis,
            //aspect: Some(screen_width() / screen_height()),
            //viewport: Some((0, 0, screen_width() as i32 - 200, screen_height() as i32 - 100)),
            ..Default::default()}), 10., 400., 16., BLACK);

        next_frame().await
    }
}
