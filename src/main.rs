use std::{
    ffi::{c_void, CStr},
    ptr, thread,
    time::Duration,
};

use camera::Camera;
use eyre::Result;
use glam::Vec3;
use gui::Gui;
use model::Model;
use renderer::Renderer;
use sdl2::{keyboard::Scancode, EventPump};

use window::MyWindow;

mod camera;
mod gui;
mod model;
mod renderer;
mod shader;
mod window;

fn main() -> Result<()> {
    let mut window = MyWindow::new("PGRF2 Projekt - Skeletální Animace - Tomáš Král")?;

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageCallback(Some(gl_debug_callback), ptr::null());
        gl::DebugMessageControl(
            gl::DONT_CARE,
            gl::DONT_CARE,
            gl::DONT_CARE,
            0,
            ptr::null(),
            gl::TRUE,
        );
    };

    let mut scene = setup_scene()?;

    let mut camera = Camera::new(
        Vec3::new(0., 0., 0.),
        0.3,
        0.05,
        window.width,
        window.height,
    );
    let mut renderer = Renderer::new()?;

    let mut gui = Gui::new();

    'render_loop: loop {
        handle_inputs(&mut window.event_pump, &mut camera);

        window.begin_frame();

        unsafe {
            //gl::Viewport(0, 0, width as i32, height as i32);
            gl::Enable(gl::DEPTH_TEST);
            //TODO: test culling
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::FrontFace(gl::CCW);

            if gui.wireframe {
                gl::PolygonMode(gl::FRONT, gl::LINE);
            } else {
                gl::PolygonMode(gl::FRONT, gl::FILL);
            }

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        renderer.render(&mut scene, &mut camera, &window, &gui);
        gui.render(&mut scene, &mut camera, &mut window.egui_ctx);

        unsafe {
            // Disable backface culling and depth test, otherwise egui doesn't render correctly
            gl::Disable(gl::DEPTH_TEST);
            gl::Disable(gl::CULL_FACE);
        }

        let should_quit = window.end_frame();
        if should_quit {
            break 'render_loop;
        }

        thread::sleep(Duration::from_millis(3));
    }

    Ok(())
}

fn setup_scene() -> Result<Vec<Model>> {
    let mut scene = Vec::new();

    let mut add = |path: &str| -> Result<()> {
        let start = std::time::Instant::now();

        let model = Model::from_gltf(path)?;

        let time = std::time::Instant::now().duration_since(start);
        println!("Loading '{path}' took '{time:?}'");

        scene.push(model);
        Ok(())
    };

    add("resources/s9_mini_drone/drone.gltf")?;
    add("resources/animated_goblin_vs._vampire_spell_casting_loop/scene.gltf")?;
    add("resources/dancing_stormtrooper/Stormtrooper.gltf")?;
    add("resources/animated_humanoid_robot/scene.gltf")?;
    add("resources/reap_the_whirlwind/Whirlwind.gltf")?;
    add("resources/toon_cat_free/Cat.gltf")?;
    add("resources/pakistan_girl_-_animated/Girl.gltf")?;
    add("resources/elephant_animation_idle/Elephant.gltf")?;
    add("resources/CesiumMan.glb")?;
    //add("resources/RiggedSimple.gltf")?;
    //add("resources/infantry/Infantry.gltf")?;

    Ok(scene)
}

fn handle_inputs(event_pump: &mut EventPump, camera: &mut Camera) {
    let k = event_pump.keyboard_state();

    if k.is_scancode_pressed(Scancode::W) {
        camera.move_forward(1.0);
    }

    if k.is_scancode_pressed(Scancode::S) {
        camera.move_backward(1.0);
    }

    if k.is_scancode_pressed(Scancode::A) {
        camera.strafe_left(1.0);
    }

    if k.is_scancode_pressed(Scancode::D) {
        camera.strafe_right(1.0);
    }

    let mouse_state = event_pump.mouse_state();
    let mouse_x = mouse_state.x() as f32;
    let mouse_y = mouse_state.y() as f32;

    if mouse_state.right() {
        camera.adjust_look(mouse_x, mouse_y);
    } else {
        camera.set_x_y(mouse_x, mouse_y)
    }
}

extern "system" fn gl_debug_callback(
    _src: u32,
    _typ: u32,
    id: u32,
    severity: u32,
    _len: i32,
    msg: *const i8,
    _user_param: *mut c_void,
) {
    // Buffer creation on NVidia cards
    if id == 131185 {
        return;
    }

    match severity {
        gl::DEBUG_SEVERITY_NOTIFICATION => print!("OpenGL - notification: "),
        gl::DEBUG_SEVERITY_LOW => print!("OpenGL - low: "),
        gl::DEBUG_SEVERITY_MEDIUM => print!("OpenGL - medium: "),
        gl::DEBUG_SEVERITY_HIGH => print!("OpenGL - high: "),
        _ => unreachable!("Unknown severity in glDebugCallback: '{}'", severity),
    }

    // TODO: check if the message is guaranteed to be ASCII
    let msg = unsafe { CStr::from_ptr(msg) };
    println!("OpenGL debug message: '{}'", msg.to_string_lossy())
}
