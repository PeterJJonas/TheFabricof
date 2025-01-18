extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use sdl2::video::FullscreenType;
use std::time::Duration;

const BASE_WIDTH: u32 = 320;
const BASE_HEIGHT: u32 = 200;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 8;
const COLS: u32 = BASE_WIDTH / CHAR_WIDTH; // 40 columns
const ROWS: u32 = BASE_HEIGHT / CHAR_HEIGHT; // 25 rows;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    // Get screen resolution
    let display_mode = video_subsystem.current_display_mode(0).unwrap();
    let screen_width = display_mode.w as u32;
    let screen_height = display_mode.h as u32;

    // Predefined window sizes based on 8:5 ratio
    let window_sizes = calculate_window_sizes(screen_width, screen_height, 8, 5);
    let mut current_size_index = 2;

    // Start with the smallest window size
    let (window_width, window_height) = window_sizes[current_size_index];

    let window = video_subsystem
        .window("The Fabricof", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut is_fullscreen = false;
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let font_path = "src/PetMe64.ttf"; // Path to your font file
    let font_size = 8; // Fixed 8x8 characters
    let font: Font = ttf_context
        .load_font(font_path, font_size)
        .expect("Failed to load font");

    // Define the custom grids
    let background_picture = vec![
        "########################################".to_string(),
        "#                                      #".to_string(),
        "#                                      #".to_string(),
        "#                                      #".to_string(),
        "#                                      #".to_string(),
        "#                                      #".to_string(),
        "#                                      #".to_string(),
        "                                        ".to_string(),
        "                                        ".to_string(),
        "                                        ".to_string(),
        "                                        ".to_string(),
        "                                        ".to_string(),
        "                                        ".to_string(),
        "                                        ".to_string(),
        "                                        ".to_string(),
        "########################################".to_string(),
        "#                                      #".to_string(),
        "#       Welcome to The Fabricof        #".to_string(),
        "#                                      #".to_string(),
        "#             ♦ Play                   #".to_string(),
        "#               Settings               #".to_string(),
        "#               Blablabla              #".to_string(),
        "#                                      #".to_string(),
        "#                                      #".to_string(),
        "########################################".to_string(),
    ];

    let landscape_picture = vec![
        "                                        ".to_string(),
        "                ▒▒                      ".to_string(),
        "                ▒▒▒▒          ♠♠♠       ".to_string(),
        "               ▒▒▒▒▒▒▒▒▒     ♠♠♠♠♠      ".to_string(),
        "               ▒ Sheep ▒     ♠tree♠ ♠   ".to_string(),
        "               ▒ Shop  ▒      ♠♠♠ ♠♠♠♠  ".to_string(),
        "               ▒▒▒▒▒▒▒▒▒       │ ♠╱♠♠♠  ".to_string(),
        "               ▒▒▒▒▒▒▒▒▒       │ ╱ ♠♠   ".to_string(),
        "               ▒    ▒▒▒▒     ♠♠│╱       ".to_string(),
        "               ▒    ▒  ▒     ♠Y│        ".to_string(),
        "               ▒    ▒  ▒      ││        ".to_string(),
        "               ▒    ▒▒▒▒      ╰│        ".to_string(),
        "               ▒    ▒▒▒▒       │        ".to_string(),
        "               ▒    ▒▒▒▒       │        ".to_string(),
        "               ▒    ▒▒▒▒       │        ".to_string(),
    ];

    let character_picture = vec![
        " ╭#╮ ".to_string(),
        " ○ ○ ".to_string(),
        " │u│ ".to_string(),
        " ╰▅╯  ".to_string(),
        " ╭▒╮ ".to_string(),
        " ▗▒▖ ".to_string(),
        " @ @ ".to_string(),
    ];
    let character_width = character_picture.iter().map(|line| line.len()).max().unwrap_or(0) as u32;
    let character_height = character_picture.len() as u32; 
    
    let mut character_x: i32 = 10; // Can move outside the screen
    let mut character_y: i32 = 8;  // Can move outside the screen

    let background_colour = Color::YELLOW;
    let landscape_colour = Color::GREEN;
    let character_colour = Color::RED;

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut is_running = true;

    while is_running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    is_running = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    is_fullscreen = !is_fullscreen;
                    if is_fullscreen {
                        canvas
                            .window_mut()
                            .set_fullscreen(FullscreenType::Desktop)
                            .unwrap();
                    } else {
                        canvas
                            .window_mut()
                            .set_fullscreen(FullscreenType::Off)
                            .unwrap();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    current_size_index = (current_size_index + 1) % window_sizes.len();
                    let (new_width, new_height) = window_sizes[current_size_index];
                    canvas.window_mut().set_size(new_width, new_height).unwrap();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => character_x = (character_x - 1).max(-(character_width as i32)),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => character_x = (character_x + 1).min(COLS as i32),
                 Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => (), //character_y = (character_y - 1).max(-(character_height as i32)),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => (), //character_y = (character_y + 1).min(ROWS as i32),
                
                _ => {}
            }
        }

        // Get current screen size
        let (current_width, current_height) = canvas.output_size().unwrap();

        // Calculate scale and paddings
        let scale_x = (current_width / BASE_WIDTH) as f32;
        let scale_y = (current_height / BASE_HEIGHT) as f32;
        let scale = scale_x.min(scale_y);

        let render_width = (BASE_WIDTH as f32 * scale) as u32;
        let render_height = (BASE_HEIGHT as f32 * scale) as u32;

        let padding_x = (current_width - render_width) / 2;
        let padding_y = (current_height - render_height) / 2;

        // Clear the screen
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Render the background and landscape grids
        for row in 0..ROWS as usize {
            for col in 0..COLS as usize {
                let mut char_to_render;
                let mut char_colour;
        
                // First, render the background
                if row < background_picture.len() {
                    if let Some(background_char) = background_picture[row].chars().nth(col) {
                        char_to_render = background_char;
                        char_colour = background_colour;
        
                        if let Ok(rendered_char) = font.render_char(char_to_render).blended(char_colour) {
                            let texture_creator = canvas.texture_creator();
                            let texture = texture_creator
                                .create_texture_from_surface(&rendered_char)
                                .unwrap();
        
                            let dest_rect = Rect::new(
                                padding_x as i32 + (col as u32 * CHAR_WIDTH) as i32 * scale as i32,
                                padding_y as i32 + (row as u32 * CHAR_HEIGHT) as i32 * scale as i32,
                                (CHAR_WIDTH as f32 * scale) as u32,
                                (CHAR_HEIGHT as f32 * scale) as u32,
                            );
                            canvas.copy(&texture, None, dest_rect).unwrap();
                        }
                    }
                }
        
                // Then, render the landscape if present
                if row < landscape_picture.len() {
                    if let Some(landscape_char) = landscape_picture[row].chars().nth(col) {
                        if landscape_char != ' ' {
                            char_to_render = landscape_char;
                            char_colour = landscape_colour;
        
                            if let Ok(rendered_char) = font.render_char(char_to_render).blended(char_colour) {
                                let texture_creator = canvas.texture_creator();
                                let texture = texture_creator
                                    .create_texture_from_surface(&rendered_char)
                                    .unwrap();
        
                                let dest_rect = Rect::new(
                                    padding_x as i32 + (col as u32 * CHAR_WIDTH) as i32 * scale as i32,
                                    padding_y as i32 + (row as u32 * CHAR_HEIGHT) as i32 * scale as i32,
                                    (CHAR_WIDTH as f32 * scale) as u32,
                                    (CHAR_HEIGHT as f32 * scale) as u32,
                                );
                                canvas.copy(&texture, None, dest_rect).unwrap();
                            }
                        }
                    }
                }
            }
        }
        

        // Render the character
        for (row, line) in character_picture.iter().enumerate() {
            for (col, char_to_render) in line.chars().enumerate() {
                let render_x = character_x + col as i32;
                let render_y = character_y + row as i32;

                // Skip rendering if the character is outside the visible grid
                if render_x < 0
                    || render_y < 0
                    || render_x >= COLS as i32
                    || render_y >= ROWS as i32
                {
                    continue;
                }

                if let Ok(rendered_char) = font.render_char(char_to_render).blended(character_colour) {
                    let texture_creator = canvas.texture_creator();
                    let texture = texture_creator
                        .create_texture_from_surface(&rendered_char)
                        .unwrap();

                    let dest_rect = Rect::new(
                        padding_x as i32
                            + (render_x * CHAR_WIDTH as i32) * scale as i32,
                        padding_y as i32
                            + (render_y * CHAR_HEIGHT as i32) * scale as i32,
                        (CHAR_WIDTH as f32 * scale) as u32,
                        (CHAR_HEIGHT as f32 * scale) as u32,
                    );
                    canvas.copy(&texture, None, dest_rect).unwrap();
                }
            }
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}

fn calculate_window_sizes(
    screen_width: u32,
    screen_height: u32,
    aspect_width: u32,
    aspect_height: u32,
) -> Vec<(u32, u32)> {
    let mut sizes = Vec::new();
    let mut width = 320;
    let mut height = width * aspect_height / aspect_width;

    while width <= screen_width && height <= screen_height {
        sizes.push((width, height));
        width += 320;
        height = width * aspect_height / aspect_width;
    }

    sizes
}
