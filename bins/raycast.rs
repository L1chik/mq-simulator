use macroquad::prelude::*;
use macroquad::math::*;

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;


struct camera {
    position: Vec3,
    up: Vec3,
    target: Vec3,
    dir: Vec3,
    local: Mat3,
}

#[macroquad::main("3D")]
async fn main() {
    let world_axes = Mat3::from_cols(
        vec3(1., 0., 0.),
        vec3(0., 1., 0.),
        vec3(0., 0., 1.)
    );
    let mut main_camera = camera{
        position: vec3(0., 0., -10.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        dir: vec3(1., 0., 0.),
        local: world_axes, 
    };

    let mut mouse_right_is_pressed = false;
    let mut mouse_middle_is_pressed = false;
    let mut control = true;
    let mut last_mouse_pos: Vec2 = mouse_position().into();
    let mut mouse_x = vec3(0., 0., 0.);
    let mut new_x = vec3(0., 0., 0.);
    let mut transition = Mat3::zero();
    let mut offset = vec3(0., 0., 0.);
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
                transition = world_axes.inverse().mul_mat3(&main_camera.local);
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

        if is_mouse_button_down(MouseButton::Right) {
            if mouse_right_is_pressed {
                //Hold
                let mouse_pos: Vec2  = mouse_position().into();
                let delta_mouse_pos = mouse_pos - last_mouse_pos;
                last_mouse_pos = mouse_pos;
                let angle = delta_mouse_pos * speed * delta;
                let rotate_mat = Mat3::from_axis_angle(main_camera.local.x_axis, angle.y * -1.);
                let rotate_mat2 = Mat3::from_axis_angle(main_camera.local.y_axis, angle.x);
                main_camera.local = rotate_mat.mul_mat3(&main_camera.local);
                main_camera.local = rotate_mat2.mul_mat3(&main_camera.local);
            }else {
                //Press
                mouse_right_is_pressed = true;
                last_mouse_pos = mouse_position().into();
            }
        } else {
            if mouse_right_is_pressed {
                //Release
                mouse_right_is_pressed = false;
            }else {
                //Not pressed
            }
        }

        if is_key_down(KeyCode::Up) {
            let angle = delta * -10. * speed;
            let rotate_mat = Mat3::from_axis_angle(main_camera.local.x_axis, angle);
            main_camera.local = rotate_mat.mul_mat3(&main_camera.local);
        }
        
        if is_key_down(KeyCode::Down) {
            let angle = delta * 10. * speed;
            let rotate_mat = Mat3::from_axis_angle(main_camera.local.x_axis, angle);
            main_camera.local = rotate_mat.mul_mat3(&main_camera.local);
        }
        if is_key_down(KeyCode::Right) {
            let angle = delta * -10. * speed;
            let rotate_mat = Mat3::from_axis_angle(main_camera.local.y_axis, angle);
            main_camera.local = rotate_mat.mul_mat3(&main_camera.local);
        }
        
        if is_key_down(KeyCode::Left) {
            let angle = delta * 10. * speed;
            let rotate_mat = Mat3::from_axis_angle(main_camera.local.y_axis, angle);
            main_camera.local = rotate_mat.mul_mat3(&main_camera.local);
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
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        draw_cube(vec3(0., 2.5, 0.), vec3(5., 5., 5.), None, GRAY);
        
        set_default_camera();
        draw_text(&*get_fps().to_string(), 10., 20., 32., BLACK);
        draw_text(&*format!("{}, {}, {}", world_axes.x_axis.x, world_axes.x_axis.y, world_axes.x_axis.z), 10., 50., 32., BLACK);
        draw_text(&*format!("{}, {}, {}", world_axes.y_axis.x, world_axes.y_axis.y, world_axes.y_axis.z), 10., 80., 32., BLACK);
        draw_text(&*format!("{}, {}, {}", world_axes.z_axis.x, world_axes.z_axis.y, world_axes.z_axis.z), 10., 110., 32., BLACK);
        draw_text(&*format!("{}, {}, {}", mouse_x.x, mouse_x.y, mouse_x.z), 10., 150., 32., BLACK);
        draw_text(&*format!("{}, {}, {}", main_camera.local.x_axis.x, main_camera.local.x_axis.y, main_camera.local.x_axis.z), 500., 50., 32., BLACK);
        draw_text(&*format!("{}, {}, {}", main_camera.local.y_axis.x, main_camera.local.y_axis.y, main_camera.local.y_axis.z), 500., 80., 32., BLACK);
        draw_text(&*format!("{}, {}, {}", main_camera.local.z_axis.x, main_camera.local.z_axis.y, main_camera.local.z_axis.z), 500., 110., 32., BLACK);
        draw_text(&*format!("{}, {}, {}", new_x.x, new_x.y, new_x.z), 500., 150., 32., BLACK);
        draw_text(&*format!("{}, {}, {}", transition.x_axis.x, transition.x_axis.y, transition.x_axis.z), 900., 50., 32., BLACK);
        draw_text(&*format!("{}, {}, {}", transition.y_axis.x, transition.y_axis.y, transition.y_axis.z), 900., 80., 32., BLACK);
        draw_text(&*format!("{}, {}, {}", transition.z_axis.x, transition.z_axis.y, transition.z_axis.z), 900., 110., 32., BLACK);

        next_frame().await
    }
}
