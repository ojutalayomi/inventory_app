use iced::window::icon;

pub fn load_icon() -> Option<icon::Icon> {
    // Create a 128x128 RGBA icon with an inventory/box theme
    let size = 128u32;
    let mut rgba = vec![0u8; (size * size * 4) as usize];

    // Background color (dark blue)
    let bg_color = [0x1a, 0x1a, 0x2e, 0xff];

    // Fill background
    for y in 0..size {
        for x in 0..size {
            let idx = ((y * size + x) * 4) as usize;
            rgba[idx..idx + 4].copy_from_slice(&bg_color);
        }
    }

    // Draw a simple box/package icon (orange/amber color)
    let box_color = [0xff, 0xa5, 0x00, 0xff]; // Orange
    let box_outline = [0x8b, 0x45, 0x13, 0xff]; // Dark brown

    // Main box body (centered square)
    let box_x = 32;
    let box_y = 48;
    let box_width = 64;
    let box_height = 56;

    // Fill box body
    for y in box_y..box_y + box_height {
        for x in box_x..box_x + box_width {
            if x < size && y < size {
                let idx = ((y * size + x) * 4) as usize;
                rgba[idx..idx + 4].copy_from_slice(&box_color);
            }
        }
    }

    // Draw box outline
    for y in box_y..box_y + box_height {
        for x in box_x..box_x + box_width {
            if x < size && y < size {
                // Top and bottom borders
                if y == box_y || y == box_y + box_height - 1 ||
                   // Left and right borders
                   x == box_x || x == box_x + box_width - 1 {
                    let idx = ((y * size + x) * 4) as usize;
                    rgba[idx..idx + 4].copy_from_slice(&box_outline);
                }
            }
        }
    }

    // Draw tape/cross on box (lighter color)
    let tape_color = [0xd2, 0x8b, 0x40, 0xff]; // Light brown
    let tape_width = 6;

    // Horizontal tape
    for y in (box_y + box_height / 2 - tape_width / 2)..(box_y + box_height / 2 + tape_width / 2) {
        for x in box_x..box_x + box_width {
            if x < size && y < size {
                let idx = ((y * size + x) * 4) as usize;
                rgba[idx..idx + 4].copy_from_slice(&tape_color);
            }
        }
    }

    // Vertical tape
    for y in box_y..box_y + box_height {
        for x in (box_x + box_width / 2 - tape_width / 2)..(box_x + box_width / 2 + tape_width / 2) {
            if x < size && y < size {
                let idx = ((y * size + x) * 4) as usize;
                rgba[idx..idx + 4].copy_from_slice(&tape_color);
            }
        }
    }

    // Draw small highlight on top-left to give 3D effect
    let highlight_color = [0xff, 0xc4, 0x4d, 0xff]; // Lighter orange
    for y in box_y + 4..box_y + 12 {
        for x in box_x + 4..box_x + 20 {
            if x < size && y < size {
                let idx = ((y * size + x) * 4) as usize;
                rgba[idx..idx + 4].copy_from_slice(&highlight_color);
            }
        }
    }

    icon::from_rgba(rgba, size, size).ok()
}

