extern crate sdl2; // Import the SDL2 library

// Import various SDL2 modules and standard library components
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use sdl2::video::FullscreenType;
use std::time::Duration;
use std::collections::HashSet;

// Constants for base dimensions and character dimensions
const BASE_WIDTH: u32 = 320;
const BASE_HEIGHT: u32 = 200;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 8;

fn main() {
    // Initialise SDL2 and its subsystems
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap(); // Initialise SDL2_ttf for font rendering

    // Get the current display mode to determine screen dimensions
    let display_mode = video_subsystem.current_display_mode(0).unwrap();
    let screen_width = display_mode.w as u32;
    let screen_height = display_mode.h as u32;

    // Calculate possible window sizes based on screen resolution and aspect ratio
    let window_sizes = calculate_window_sizes(screen_width, screen_height); //, 8, 5
    let mut current_size_index = 6; // Default to an intermediate size
    println!("Calculated window sizes: {:?}", window_sizes);
    println!("Initial size index: {}", current_size_index);

    let (window_width, window_height) = window_sizes[current_size_index];
    println!("main() 1 window width: {}, Window height: {}", window_width, window_height);


    // Create a centred SDL2 window
    let window = video_subsystem
        .window("The Fabricof", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();
    println!("main() 2 window width: {}, Window height: {}", window_width, window_height);

    let mut is_fullscreen = false; // Track whether the window is in fullscreen mode
    let mut canvas = window.into_canvas().build().unwrap(); // Create a rendering canvas
    
    // Load the font from the specified path
    let font_path = "src/PetMe64.ttf"; // Path to font file
    let font_size = 8; // Character size (8x8)
    let font: Font = ttf_context
        .load_font(font_path, font_size)
        .expect("Failed to load font");

    // Define the background picture (grid of characters)
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

    // Define the landscape picture (grid of ASCII art)
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
        " START         ▒    ▒▒▒▒      ╰│        ".to_string(),
        "   │           ▒    ▒▒▒▒       │        ".to_string(),
        "   │           ▒    ▒▒▒▒       │        ".to_string(),
        "   │           ▒    ▒▒▒▒       │        ".to_string(),
    ];

    // Define the character picture (grid of ASCII art representing the character)
    let character_picture = vec![
        " ╭#╮ ".to_string(),
        " ○ ○ ".to_string(),
        " │u│ ".to_string(),
        " ╰▅╯  ".to_string(),
        " ╭▒╮ ".to_string(),
        " ▗▒▖ ".to_string(),
        " @ @ ".to_string(),
    ];

    // Initial position of the character
    let mut character_x: i32 = 7;
    let character_y: i32 = 8;

    let mut event_pump = sdl_context.event_pump().unwrap(); // Event handler
    let mut is_running = true; // Main game loop flag
    let mut revealed_positions: HashSet<(usize, usize)> = HashSet::new(); // Track revealed positions

    // Main game loop
    while is_running {
        // Handle user inputs and events
        handle_events(
            &mut is_running,
            &mut is_fullscreen,
            &mut current_size_index,
            &window_sizes,
            &mut canvas,
            &mut character_x,
            &mut event_pump,
        );

        // Get current window size and recalculate scaling factors
        //let (window_width, window_height) = window_sizes[current_size_index];
        //println!("while 1 window width: {}, Window height: {}", window_width, window_height);
        let (window_width, window_height) = canvas.window().size();
        println!("while 2 window width: {}, Window height: {}", window_width, window_height);
        let scale_x = window_width as f32 / BASE_WIDTH as f32;
        let scale_y = window_height as f32 / BASE_HEIGHT as f32;
        println!("while 3 window width: {}, Window height: {}", window_width, window_height);


        // Clear the canvas with a black colour
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Render the background and landscape grids
        render_background_and_landscape(
            &background_picture,
            &landscape_picture,
            &font,
            &mut canvas,
            scale_x,
            scale_y,
            (character_x as usize, character_y as usize), // Pass character position
            &mut revealed_positions, // Pass revealed positions
        );

        // Render the character at its current position
        render_character(
            &character_picture,
            &font,
            &mut canvas,
            scale_x,
            scale_y,
            (character_x as isize, character_y as isize), // Pass character position
        );

        // Update the display
        canvas.present();

        // Pause for a short duration to limit frame rate (roughly 60 FPS)
        std::thread::sleep(Duration::from_millis(16));
    }
}

// Render the background and landscape grids
fn render_background_and_landscape(
    background_picture: &[String],
    landscape_picture: &[String],
    font: &Font,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    scale_x: f32,
    scale_y: f32,
    character_position: (usize, usize), // Add character position parameter
    revealed_positions: &mut HashSet<(usize, usize)>, // Add revealed positions parameter
) {
    // Render the background grid
    for (row, line) in background_picture.iter().enumerate() {
        for (col, char_to_render) in line.chars().enumerate() {
            if let Ok(rendered_char) = font.render_char(char_to_render).blended(Color::YELLOW) {
                let texture_creator = canvas.texture_creator();
                let texture = texture_creator.create_texture_from_surface(&rendered_char).unwrap();

                let dest_rect = Rect::new(
                    (col as f32 * CHAR_WIDTH as f32 * scale_x) as i32,
                    (row as f32 * CHAR_HEIGHT as f32 * scale_y) as i32,
                    (CHAR_WIDTH as f32 * scale_x) as u32,
                    (CHAR_HEIGHT as f32 * scale_y) as u32,
                );
                canvas.copy(&texture, None, dest_rect).unwrap();
            }
        }
    }

    // Render the landscape grid
    let (char_x, char_y) = character_position;
    let reveal_radius = 6; // Define the radius around the character to reveal

    for (row, line) in landscape_picture.iter().enumerate() {
        for (col, char_to_render) in line.chars().enumerate() {
            // Only render characters within the reveal radius or already revealed
            if (row as isize - char_y as isize).abs() <= reveal_radius && (col as isize - char_x as isize).abs() <= reveal_radius {
                revealed_positions.insert((row, col));
            }

            if revealed_positions.contains(&(row, col)) {
                if let Ok(rendered_char) = font.render_char(char_to_render).blended(Color::GREEN) {
                    let texture_creator = canvas.texture_creator();
                    let texture = texture_creator.create_texture_from_surface(&rendered_char).unwrap();

                    let dest_rect = Rect::new(
                        (col as f32 * CHAR_WIDTH as f32 * scale_x) as i32,
                        (row as f32 * CHAR_HEIGHT as f32 * scale_y) as i32,
                        (CHAR_WIDTH as f32 * scale_x) as u32,
                        (CHAR_HEIGHT as f32 * scale_y) as u32,
                    );
                    canvas.copy(&texture, None, dest_rect).unwrap();
                }
            }
        }
    }
}

// Render the character at its current position
fn render_character(
    character_picture: &[String],
    font: &Font,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    scale_x: f32,
    scale_y: f32,
    character_position: (isize, isize), // Change to isize
) {
    let (char_x, char_y) = character_position;

    for (row, line) in character_picture.iter().enumerate() {
        for (col, char_to_render) in line.chars().enumerate() {
            if let Ok(rendered_char) = font.render_char(char_to_render).blended(Color::RED) {
                let texture_creator = canvas.texture_creator();
                let texture = texture_creator.create_texture_from_surface(&rendered_char).unwrap();

                // Use checked_add to prevent overflow
                let dest_x = char_x.checked_add(col as isize).expect("Overflow occurred") as f32 * CHAR_WIDTH as f32 * scale_x;
                let dest_y = char_y.checked_add(row as isize).expect("Overflow occurred") as f32 * CHAR_HEIGHT as f32 * scale_y;

                let dest_rect = Rect::new(
                    dest_x as i32,
                    dest_y as i32,
                    (CHAR_WIDTH as f32 * scale_x) as u32,
                    (CHAR_HEIGHT as f32 * scale_y) as u32,
                );
                println!("{}", dest_x);
                canvas.copy(&texture, None, dest_rect).unwrap();
            }
        }
    }
}

// Handle user inputs and events
fn handle_events(
    is_running: &mut bool,
    is_fullscreen: &mut bool,
    current_size_index: &mut usize,
    window_sizes: &[(u32, u32)],
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    character_x: &mut i32,
    event_pump: &mut sdl2::EventPump,
) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } // Quit the game
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                *is_running = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => {
                // Toggle fullscreen mode
                *is_fullscreen = !*is_fullscreen;
                if *is_fullscreen {
                    canvas
                        .window_mut()
                        .set_fullscreen(FullscreenType::True)
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
                if !*is_fullscreen {
                    // Resize the window
                    *current_size_index = (*current_size_index + 1) % window_sizes.len();
                    let (new_width, new_height) = window_sizes[*current_size_index];
                    canvas.window_mut().set_size(new_width, new_height).unwrap();
                    println!("Current size index: {}, current window sizes {:?}", *current_size_index, window_sizes[*current_size_index]);
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => *character_x -= 1, // Move character left
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => *character_x += 1, // Move character right
            _ => {}
        }
    }
}

// Calculate possible window sizes based on base resolution and screen size
fn calculate_window_sizes(
    screen_width: u32,
    screen_height: u32,
//    aspect_width: u32,
//    aspect_height: u32,
) -> Vec<(u32, u32)> {
    let mut sizes = Vec::new();
    let mut width = BASE_WIDTH;
    let mut height = BASE_HEIGHT;

    while width <= screen_width && height <= screen_height {
        sizes.push((width, height));
        width += BASE_WIDTH;
        height += BASE_HEIGHT;
//        height = (width * aspect_height) / aspect_width;
    }
    sizes
}