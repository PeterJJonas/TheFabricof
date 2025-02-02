extern crate sdl2; // Import the SDL2 library

use sdl2::event::Event; // Import SDL2 event handling
use sdl2::keyboard::Keycode; // Import SDL2 keycode handling
use sdl2::pixels::Color; // Import SDL2 color handling
use sdl2::rect::Rect; // Import SDL2 rectangle handling
use sdl2::ttf::Font; // Import SDL2 TTF font handling
use sdl2::video::FullscreenType; // Import SDL2 fullscreen handling
use std::time::Duration; // Import duration handling
use std::collections::HashSet; // Import HashSet collection

const BASE_WIDTH: u32 = 320; // Base width for window scaling
const BASE_HEIGHT: u32 = 200; // Base height for window scaling
const CHAR_WIDTH: u32 = 8; // Character width for rendering
const CHAR_HEIGHT: u32 = 8; // Character height for rendering

fn main() {
    // Initialize SDL2 context and subsystems
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    // Get the current display mode to determine screen dimensions
    let display_mode = video_subsystem.current_display_mode(0).unwrap();
    let screen_width = display_mode.w as u32;
    let screen_height = display_mode.h as u32;

    // Calculate possible window sizes based on screen dimensions
    let window_sizes = calculate_window_sizes(screen_width, screen_height);
    let mut current_size_index = if window_sizes.len() > 2 {
        window_sizes.len() - 2
    } else {
        1
    };
    //println!("Calculated window sizes: {:?}", window_sizes);
    //println!("Initial size index: {}", current_size_index);

    // Set initial window size
    let (window_width, window_height) = window_sizes[current_size_index];
    //println!("main() 1 window width: {}, Window height: {}", window_width, window_height);

    // Create the SDL2 window
    let window = video_subsystem
        .window("The Fabricof", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();
    //println!("main() 2 window width: {}, Window height: {}", window_width, window_height);

    // Initialize fullscreen state and canvas
    let mut is_fullscreen = false;
    let mut canvas = window.into_canvas().build().unwrap();
    
    // Load the font for rendering text
    let font_path = "src/PetMe64.ttf";
    let font_size = 8;
    let font: Font = ttf_context
        .load_font(font_path, font_size)
        .expect("Failed to load font");

    // Load pictures for background, landscape, and character
    let background_picture = get_background_picture();
    let landscape_picture = get_landscape_picture();
    let character_picture = get_character_picture();

    // Initialize character position
    let mut character_x: i32 = 7;
    let character_y: i32 = 8;

    // Initialize event pump and running state
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut is_running = true;
    let mut revealed_positions: HashSet<(usize, usize)> = HashSet::new();

    // Main game loop
    while is_running {
        // Handle user input and events
        handle_events(
            &mut is_running,
            &mut is_fullscreen,
            &mut current_size_index,
            &window_sizes,
            &mut canvas,
            &mut character_x,
            &mut event_pump,
        );

        // Get current window size and calculate scaling factors
        let (window_width, window_height) = canvas.window().size();
        //println!("while 2 window width: {}, Window height: {}", window_width, window_height);
        let scale_x = window_width as f32 / BASE_WIDTH as f32;
        let scale_y = window_height as f32 / BASE_HEIGHT as f32;
        //println!("while 3 window width: {}, Window height: {}", window_width, window_height);

        // Clear the canvas with a black background
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Render the background and landscape
        render_background_and_landscape(
            &background_picture,
            &landscape_picture,
            &font,
            &mut canvas,
            scale_x,
            scale_y,
            (character_x as usize, character_y as usize),
            &mut revealed_positions,
        );

        // Render the character
        render_character(
            &character_picture,
            &font,
            &mut canvas,
            scale_x,
            scale_y,
            (character_x as isize, character_y as isize),
        );

        // Present the updated canvas
        canvas.present();
        std::thread::sleep(Duration::from_millis(16)); // Sleep to control frame rate
    }
}

// Function to get the background picture as a vector of strings
fn get_background_picture() -> Vec<String> {
    vec![
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
    ]
}

// Function to get the landscape picture as a vector of strings
fn get_landscape_picture() -> Vec<String> {
    vec![
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
    ]
}

// Function to get the character picture as a vector of strings
fn get_character_picture() -> Vec<String> {
    vec![
        " ╭#╮ ".to_string(),
        " ○ ○ ".to_string(),
        " │u│ ".to_string(),
        " ╰▅╯ ".to_string(),
        " ╭▒╮ ".to_string(),
        " ▗▒▖ ".to_string(),
        " @ @ ".to_string(),
    ]
}

// Function to render the background and landscape
fn render_background_and_landscape(
    background_picture: &[String],
    landscape_picture: &[String],
    font: &Font,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    scale_x: f32,
    scale_y: f32,
    character_position: (usize, usize),
    revealed_positions: &mut HashSet<(usize, usize)>,
) {
    // Render the background grid
    render_grid(background_picture, font, canvas, scale_x, scale_y, Color::YELLOW);
    // Render the landscape
    render_landscape(
        landscape_picture,
        font,
        canvas,
        scale_x,
        scale_y,
        character_position,
        revealed_positions,
    );
}

// Function to render a grid of characters
fn render_grid(
    grid: &[String],
    font: &Font,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    scale_x: f32,
    scale_y: f32,
    color: Color,
) {
    for (row, line) in grid.iter().enumerate() { // Iterate over each row
        for (col, char_to_render) in line.chars().enumerate() { // Iterate over each character in the row
            if let Ok(rendered_char) = font.render_char(char_to_render).blended(color) { // Render the character
                let texture_creator = canvas.texture_creator(); // Create a texture creator
                let texture = texture_creator.create_texture_from_surface(&rendered_char).unwrap(); // Create a texture from the rendered character

                let dest_rect = Rect::new(
                    (col as f32 * CHAR_WIDTH as f32 * scale_x) as i32, // Calculate x position
                    (row as f32 * CHAR_HEIGHT as f32 * scale_y) as i32, // Calculate y position
                    (CHAR_WIDTH as f32 * scale_x) as u32, // Calculate width
                    (CHAR_HEIGHT as f32 * scale_y) as u32, // Calculate height
                );
                canvas.copy(&texture, None, dest_rect).unwrap(); // Copy the texture to the canvas
            }
        }
    }
}

// Function to render the landscape with revealed positions
fn render_landscape(
    landscape_picture: &[String],
    font: &Font,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    scale_x: f32,
    scale_y: f32,
    character_position: (usize, usize),
    revealed_positions: &mut HashSet<(usize, usize)>,
) {
    let (char_x, char_y) = character_position; // Get character position
    let reveal_radius = 6; // Set reveal radius

    for (row, line) in landscape_picture.iter().enumerate() { // Iterate over each row
        for (col, char_to_render) in line.chars().enumerate() { // Iterate over each character in the row
            if (row as isize - char_y as isize).abs() <= reveal_radius && (col as isize - char_x as isize).abs() <= reveal_radius {
                revealed_positions.insert((row, col)); // Reveal position if within radius
            }

            if revealed_positions.contains(&(row, col)) { // Check if position is revealed
                if let Ok(rendered_char) = font.render_char(char_to_render).blended(Color::GREEN) { // Render the character
                    let texture_creator = canvas.texture_creator(); // Create a texture creator
                    let texture = texture_creator.create_texture_from_surface(&rendered_char).unwrap(); // Create a texture from the rendered character

                    let dest_rect = Rect::new(
                        (col as f32 * CHAR_WIDTH as f32 * scale_x) as i32, // Calculate x position
                        (row as f32 * CHAR_HEIGHT as f32 * scale_y) as i32, // Calculate y position
                        (CHAR_WIDTH as f32 * scale_x) as u32, // Calculate width
                        (CHAR_HEIGHT as f32 * scale_y) as u32, // Calculate height
                    );
                    canvas.copy(&texture, None, dest_rect).unwrap(); // Copy the texture to the canvas
                }
            }
        }
    }
}

// Function to render the character
fn render_character(
    character_picture: &[String],
    font: &Font,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    scale_x: f32,
    scale_y: f32,
    character_position: (isize, isize),
) {
    let (char_x, char_y) = character_position; // Get character position

    for (row, line) in character_picture.iter().enumerate() { // Iterate over each row
        for (col, char_to_render) in line.chars().enumerate() { // Iterate over each character in the row
            if let Ok(rendered_char) = font.render_char(char_to_render).blended(Color::RED) { // Render the character
                let texture_creator = canvas.texture_creator(); // Create a texture creator
                let texture = texture_creator.create_texture_from_surface(&rendered_char).unwrap(); // Create a texture from the rendered character

                let dest_x = char_x.checked_add(col as isize).expect("Overflow occurred") as f32 * CHAR_WIDTH as f32 * scale_x; // Calculate x position
                let dest_y = char_y.checked_add(row as isize).expect("Overflow occurred") as f32 * CHAR_HEIGHT as f32 * scale_y; // Calculate y position

                let dest_rect = Rect::new(
                    dest_x as i32, // Set x position
                    dest_y as i32, // Set y position
                    (CHAR_WIDTH as f32 * scale_x) as u32, // Set width
                    (CHAR_HEIGHT as f32 * scale_y) as u32, // Set height
                );
                //println!("Character X position{}", dest_x);
                canvas.copy(&texture, None, dest_rect).unwrap(); // Copy the texture to the canvas
            }
        }
    }
}

// Function to handle user input and events
fn handle_events(
    is_running: &mut bool,
    is_fullscreen: &mut bool,
    current_size_index: &mut usize,
    window_sizes: &[(u32, u32)],
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    character_x: &mut i32,
    event_pump: &mut sdl2::EventPump,
) {
    for event in event_pump.poll_iter() { // Iterate over events
        match event {
            Event::Quit { .. } // Handle quit event
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                *is_running = false; // Set running to false to exit loop
            }
            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => {
                toggle_fullscreen(is_fullscreen, canvas); // Toggle fullscreen mode
            }
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => {
                resize_window(is_fullscreen, current_size_index, window_sizes, canvas); // Resize window
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

// Function to toggle fullscreen mode
fn toggle_fullscreen(is_fullscreen: &mut bool, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    *is_fullscreen = !*is_fullscreen; // Toggle fullscreen state
    if *is_fullscreen {
        canvas.window_mut().set_fullscreen(FullscreenType::True).unwrap(); // Set fullscreen
    } else {
        canvas.window_mut().set_fullscreen(FullscreenType::Off).unwrap(); // Exit fullscreen
    }
}

// Function to resize the window
fn resize_window(
    is_fullscreen: &bool,
    current_size_index: &mut usize,
    window_sizes: &[(u32, u32)],
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) {
    if !*is_fullscreen { // Only resize if not in fullscreen mode
        *current_size_index = (*current_size_index + 1) % window_sizes.len(); // Increment size index
        let (new_width, new_height) = window_sizes[*current_size_index]; // Get new size
        canvas.window_mut().set_size(new_width, new_height).unwrap(); // Set new window size
        println!("Current size index: {}, current window sizes {:?}", *current_size_index, window_sizes[*current_size_index]);
    }
}

// Function to calculate possible window sizes based on screen dimensions
fn calculate_window_sizes(screen_width: u32, screen_height: u32) -> Vec<(u32, u32)> {
    let mut sizes = Vec::new(); // Initialize sizes vector
    let mut width = BASE_WIDTH; // Start with base width
    let mut height = BASE_HEIGHT; // Start with base height

    while width <= screen_width && height <= screen_height { // Loop until width and height exceed screen dimensions
        sizes.push((width, height)); // Add size to vector
        width += BASE_WIDTH; // Increment width
        height += BASE_HEIGHT; // Increment height
    }

    if sizes.len() < 2 { // Ensure at least two sizes
        sizes.push((BASE_WIDTH, BASE_HEIGHT)); // Add base size if necessary
    }

    sizes
}