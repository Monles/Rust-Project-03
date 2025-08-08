use macroquad::prelude::*;

struct Player {
    x: f32,
    y: f32,
    vel_y: f32,
    on_ground: bool,
}

#[macroquad::main("Simple Platformer")]
async fn main() {
    let mut player = Player {
        x: 100.0,
        y: 300.0,
        vel_y: 0.0,
        on_ground: false,
    };

    // Simple platforms (x, y, width, height)
    let platforms = vec![
        (0.0, 550.0, 800.0, 50.0),      // ground
        (200.0, 450.0, 100.0, 20.0),   // platform 1
        (400.0, 350.0, 100.0, 20.0),   // platform 2
        (600.0, 250.0, 100.0, 20.0),   // platform 3
    ];

    loop {
        let dt = get_frame_time();

        // Handle input
        if is_key_down(KeyCode::A) {
            player.x -= 200.0 * dt;
        }
        if is_key_down(KeyCode::D) {
            player.x += 200.0 * dt;
        }
        if is_key_pressed(KeyCode::W) && player.on_ground {
            player.vel_y = -400.0;
            player.on_ground = false;
        }

        // Apply gravity
        player.vel_y += 800.0 * dt;
        player.y += player.vel_y * dt;

        // Reset on_ground
        player.on_ground = false;

        // Check collisions with platforms
        for (px, py, pw, ph) in &platforms {
            // If player is touching platform from above
            if player.x + 30.0 > *px && player.x < px + pw &&
               player.y + 30.0 > *py && player.y + 30.0 < py + ph + 10.0 &&
               player.vel_y > 0.0 {
                player.y = py - 30.0;
                player.vel_y = 0.0;
                player.on_ground = true;
            }
        }

        // Keep player on screen
        if player.x < 0.0 { player.x = 0.0; }
        if player.x > 770.0 { player.x = 770.0; }

        // Reset if falling off
        if player.y > 700.0 {
            player.x = 100.0;
            player.y = 300.0;
            player.vel_y = 0.0;
        }

        // Draw everything
        clear_background(SKYBLUE);

        // Draw platforms
        for (px, py, pw, ph) in &platforms {
            let color = if *py > 500.0 { GREEN } else { BROWN };
            draw_rectangle(*px, *py, *pw, *ph, color);
        }

        // Draw player
        let player_color = if player.on_ground { BLUE } else { RED };
        draw_rectangle(player.x, player.y, 30.0, 30.0, player_color);

        // Draw instructions
        draw_text("A/D to move, W to jump", 10.0, 30.0, 30.0, BLACK);

        next_frame().await;
    }
}