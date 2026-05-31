use std::fmt::Write as _;
use crate::utils::{Rgb, boost_color};

pub fn render_quadrants(
    buffer: &mut String, 
    data: &[u8], 
    stride: u32, 
    width: u32, 
    height: u32
) {
    let mut last_fg = None;
    let mut last_bg = None;
    let quadrants = [' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█'];

    for y in (0..height.saturating_sub(1)).step_by(2) {
        for x in (0..width.saturating_sub(1)).step_by(2) {
            let mut pxs = [Rgb { r: 0, g: 0, b: 0 }; 4];
            let mut total_luma = 0.0;
            
            for dy in 0..2 {
                for dx in 0..2 {
                    let idx = (y as usize + dy) * stride as usize + (x as usize + dx) * 3;
                    if idx + 2 >= data.len() { continue; }
                    let p = boost_color(data[idx], data[idx + 1], data[idx + 2]);
                    pxs[dy * 2 + dx] = p;
                    total_luma += (p.r as f32 * 0.299 + p.g as f32 * 0.587 + p.b as f32 * 0.114) / 255.0;
                }
            }
            
            let avg_luma = total_luma / 4.0;
            let (mut mask, mut fg_c, mut bg_c) = (0usize, (0u32, 0u32, 0u32, 0u32), (0u32, 0u32, 0u32, 0u32));

            for (i, p) in pxs.iter().enumerate() {
                let l = (p.r as f32 * 0.299 + p.g as f32 * 0.587 + p.b as f32 * 0.114) / 255.0;
                if l >= avg_luma {
                    mask |= 1 << i;
                    fg_c.0 += p.r as u32; fg_c.1 += p.g as u32; fg_c.2 += p.b as u32; fg_c.3 += 1;
                } else {
                    bg_c.0 += p.r as u32; bg_c.1 += p.g as u32; bg_c.2 += p.b as u32; bg_c.3 += 1;
                }
            }
            
            let fg = if fg_c.3 > 0 { 
                Rgb { r: (fg_c.0 / fg_c.3) as u8, g: (fg_c.1 / fg_c.3) as u8, b: (fg_c.2 / fg_c.3) as u8 } 
            } else { 
                pxs[0] 
            };
            
            let bg = if bg_c.3 > 0 { 
                Rgb { r: (bg_c.0 / bg_c.3) as u8, g: (bg_c.1 / bg_c.3) as u8, b: (bg_c.2 / bg_c.3) as u8 } 
            } else { 
                fg 
            };

            if Some(fg) != last_fg || Some(bg) != last_bg {
                let _ = write!(buffer, "\x1b[38;2;{};{};{};48;2;{};{};{}m", fg.r, fg.g, fg.b, bg.r, bg.g, bg.b);
                last_fg = Some(fg); 
                last_bg = Some(bg);
            }
            buffer.push(quadrants[mask]);
        }
        buffer.push_str("\x1b[0m\n");
        last_fg = None; 
        last_bg = None;
    }
}
