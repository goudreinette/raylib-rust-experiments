use raylib::prelude::*;



fn main() {
    let (mut rl, thread) = raylib::init()
        .title("Hello, World")
        .msaa_4x()
        .size(800, 800)
        .build();

    let my_str = include_str!("./resources/shaders/wave.frag");
    let mut s =  rl.load_shader_code(&thread, None, Some(my_str));
    let time_loc = s.get_shader_location("uTime");
    s.set_shader_value(time_loc, 15.0);

    let img = Image::gen_image_color(rl.get_screen_width(),rl.get_screen_height(),Color::WHITE);
    let tex = rl.load_texture_from_image(&thread, &img).unwrap();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let red = ((((d.get_time()).sin() + 1.0) / 2.0) * 255.0) as u8;

        s.set_shader_value(time_loc, d.get_time() as f32);

        let st = d.get_fps().to_string();
        let ss: &str = &st;
        d.clear_background(Color::new(red, 0,100,255));
        d.draw_text(ss, 12, 12, 20, Color::WHITE);
        d.draw_circle(d.get_mouse_x()*2, d.get_mouse_y()*2, 50.0, Color::WHITE);


        let mut t = d.begin_shader_mode(&s);
        t.draw_texture(&tex,0,0, Color::WHITE);

        // println!("{}", red);
    }
}
