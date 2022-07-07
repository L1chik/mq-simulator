use macroquad::prelude::*;

use macroquad::models::{draw_mesh, Vertex};

use macroquad::color::Color;

use raycast::ray::Ray;
use raycast::camera::{MainCamera};
use raycast::objects::{GameObject, Primitivies, Transform, Render};
use raycast::world_axes;

struct MyCamera {
    position: Vec3,
    local: Mat3,
}

/*fn draw_selected(selected: Option<&GameObject>, color: Color) {
    match selected {
        Some(x) => {
            let mesh = x.mesh.as_ref().unwrap();
            for i in 0..mesh.indices.len() / 3 {
                let mut A = mesh.vertices[mesh.indices[3*i + 0] as usize].position;
                let mut B = mesh.vertices[mesh.indices[3*i + 1] as usize].position;
                let mut C = mesh.vertices[mesh.indices[3*i + 2] as usize].position;
                let A_dir = A - x.transform.position;
                let B_dir = B - x.transform.position;
                let C_dir = C - x.transform.position;
                draw_line_3d(A, B, color);
                draw_line_3d(B, C, color);
                draw_line_3d(A, C, color);
            }
        },
        _ => ()
    }
}*/

#[macroquad::main("3D")]
async fn main() {
    let mut mouse_right_is_pressed = false;
    let mut mouse_middle_is_pressed = false;
    let mut mouse_left_is_pressed = false;
    let mut last_mouse_pos: Vec2 = mouse_position().into();
    let mut mouse_x = vec3(0., 0., 0.);
    let mut new_x = vec3(0., 0., 0.);
    let mut mp = vec2(0., 0.);
    let mut main_camera = MainCamera::default();
    let mut ray = Ray{
        origin: vec3(0., 0., 0.),
        direction: vec3(0., 0., 1.),
    };
    let mut selected: Option<&GameObject> = None;
    let rust_logo = load_texture("rust.png").await.unwrap();
    let cube = GameObject::new(Some(Transform::default()), Some(Primitivies::cube(Some(rust_logo))));
    let mut scene = vec![cube];
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
                let transition = world_axes.inverse().mul_mat3(&main_camera.local_axes);
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

        /*if is_mouse_button_down(MouseButton::Left) {
            if mouse_left_is_pressed {
                //Hold
            }else {
                //Press
                mouse_left_is_pressed = true;
                let mouse_pos: Vec2 = mouse_position().into();
                ray = main_camera.screen_point_to_ray(mouse_pos.extend(1.));
                selected = ray.cast(&scene);
                println!("{:?}", selected);
                //let t = -ray.origin.y / ray.direction.y;
                //if t >= 0. {
                //    let new_cube: GameObject = Primitivies::cube_with_pos(ray.origin + ray.direction * t);
                //    scene.push(new_cube);
                //}
            }
        } else {
            if mouse_left_is_pressed {
                //Release
                mouse_left_is_pressed = false;
            }else {
                //Not pressed
            }
        }*/

        if is_mouse_button_down(MouseButton::Right) {
            if mouse_right_is_pressed {
                //Hold
                let mouse_pos: Vec2  = mouse_position().into();
                let delta_mouse_pos = last_mouse_pos - mouse_pos;
                last_mouse_pos = mouse_pos;
                let angle = delta_mouse_pos * speed * delta;
                let rotate_mat = Mat3::from_axis_angle(main_camera.local_axes.x_axis, angle.y * -1.);
                let rotate_mat2 = Mat3::from_axis_angle(world_axes.y_axis, angle.x);
                let sum_rotate = rotate_mat.mul_mat3(&rotate_mat2);
                //let rot = Quat::from_rotation_ypr(angle.x * -1., angle.y, 0.);
                
                main_camera.local_axes.z_axis = sum_rotate.mul_vec3(main_camera.local_axes.z_axis).normalize();
                main_camera.local_axes.x_axis = world_axes.y_axis.cross(main_camera.local_axes.z_axis).normalize();
                main_camera.local_axes.y_axis = main_camera.local_axes.z_axis.cross(main_camera.local_axes.x_axis).normalize();
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

        if is_key_down(KeyCode::Right) {
            let angle = 5. * delta;
            //my_cube.rotate_z(angle);
        }   
 
        if is_key_down(KeyCode::Left) {
            let angle = -5. * delta;
            //my_cube.rotate_z(angle);
        }   

        if is_key_down(KeyCode::Up) {
            let angle = 5. * delta;
            //my_cube.rotate_x(angle);
        }   
 
        if is_key_down(KeyCode::Down) {
            let angle = -5. * delta;
            //my_cube.rotate_x(angle);
        }   
        //Change


        //draw
        clear_background(LIGHTGRAY);
        
        //zoom
        main_camera.position += main_camera.local_axes.z_axis* mouse_wheel().1;
        //rotate

        //look_at
        let cam = main_camera.get_view();

        set_camera(&cam);

        draw_grid(50, 10., BLACK, GRAY);
        
        for obj in scene.iter() {
            obj.render();
        }
    
       // draw_selected(selected, YELLOW);

        set_default_camera();
        draw_text(&*get_fps().to_string(), 10., 20., 32., BLACK);
        /*draw_text(&*format!("{}, {}, {}", world_axes.x_axis.x, world_axes.x_axis.y, world_axes.x_axis.z), 10., 50., 16., BLACK);
        draw_text(&*format!("{}, {}, {}", world_axes.y_axis.x, world_axes.y_axis.y, world_axes.y_axis.z), 10., 80., 16., BLACK);
        draw_text(&*format!("{}, {}, {}", world_axes.z_axis.x, world_axes.z_axis.y, world_axes.z_axis.z), 10., 110., 16., BLACK);
        draw_text(&*format!("{}, {}, {}", mouse_x.x, mouse_x.y, mouse_x.z), 10., 150., 16., BLACK);
        draw_text(&*format!("{}, {}, {}", main_camera.local_axes.x_axis.x, main_camera.local_axes.x_axis.y, main_camera.local_axes.x_axis.z), 200., 50., 16., BLACK);
        draw_text(&*format!("{}, {}, {}", main_camera.local_axes.y_axis.x, main_camera.local_axes.y_axis.y, main_camera.local_axes.y_axis.z), 200., 80., 16., BLACK);
        draw_text(&*format!("{}, {}, {}", main_camera.local_axes.z_axis.x, main_camera.local_axes.z_axis.y, main_camera.local_axes.z_axis.z), 200., 110., 16., BLACK);
        draw_text(&*format!("{}, {}, {}", new_x.x, new_x.y, new_x.z), 500., 150., 16., BLACK);
        draw_text(&*format!("|x| = {}, |y| = {}, |z| = {}", main_camera.local_axes.x_axis.length(), main_camera.local_axes.y_axis.length(), main_camera.local_axes.z_axis.length()), 800., 50., 16., BLACK);
        draw_text(&*format!("x*y={}, x*z={},", main_camera.local_axes.x_axis.dot(main_camera.local_axes.y_axis), main_camera.local_axes.x_axis.dot(main_camera.local_axes.z_axis)), 800., 80., 16., BLACK);
        draw_text(&*format!("y*x={}, y*z={},", main_camera.local_axes.y_axis.dot(main_camera.local_axes.x_axis), main_camera.local_axes.y_axis.dot(main_camera.local_axes.z_axis)), 800., 110., 16., BLACK);
        draw_text(&*format!("z*x={}, z*y={},", main_camera.local_axes.z_axis.dot(main_camera.local_axes.x_axis), main_camera.local_axes.z_axis.dot(main_camera.local_axes.y_axis)), 800., 140., 16., BLACK);
        draw_text(&*format!("ray: {:?}, {:?}", ray.origin, ray.direction), 10., 200., 16., BLACK);
        draw_text(&*format!("scene: {:?}", scene), 10., 350., 16., BLACK);
        draw_text(&*format!("{:?}", Camera3D {
            position: main_camera.position,
            up: main_camera.local_axes.y_axis,
            target: main_camera.position + main_camera.local_axes.z_axis,
            //aspect: Some(screen_width() / screen_height()),
            //viewport: Some((0, 0, screen_width() as i32 - 200, screen_height() as i32 - 100)),
            ..Default::default()}), 10., 400., 16., BLACK);
        draw_text(&*format!("{:?}", Camera3D {
            position: main_camera.position,
            up: main_camera.local_axes.y_axis,
            target: main_camera.position + main_camera.local_axes.z_axis,
            //aspect: Some(screen_width() / screen_height()),
            //viewport: Some((0, 0, screen_width() as i32 - 200, screen_height() as i32 - 100)),
            ..Default::default()}.matrix()), 10., 600., 16., BLACK);
        draw_text(&*format!("{:?}", Mat4::perspective_rh_gl(cam.fovy, cam.aspect.unwrap_or(screen_width() / screen_height()), 0.01, 10000.0)), 10., 700., 16., BLACK);
        draw_text(&*format!("{:?}", Mat4::look_at_rh(cam.position, cam.target, cam.up)), 10., 800., 16., BLACK);
        draw_text(&*format!("world to screen{:?}", main_camera.world_point_to_screen(Vec3::ZERO)), 10., 250., 16., BLACK);
        draw_text(&*format!("screen to world{:?}", main_camera.screen_point_to_world_point(vec4(mouse_position().0, mouse_position().1, 0., 1.))), 10., 275., 16., BLACK);
        draw_text(&*format!("{}, {}", mouse_position().0 / (screen_width() * 0.5) - 1., mouse_position().1 / -(screen_height() * 0.5) + 1.), 10., 300., 16., BLACK);
*/
        next_frame().await
    }
}
