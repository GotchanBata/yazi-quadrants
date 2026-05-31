mod quadrants;
mod utils;

use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::path::Path;

/// end work not panic 
fn print_error_and_exit(msg: &str) -> ! {
    let mut stdout = io::stdout().lock();
    let _ = write!(stdout, "{}\n__FRAME_END__\n", msg);
    let _ = stdout.flush();
    std::process::exit(1);
}

fn main() -> io::Result<()> {
    let args: Vec<std::ffi::OsString> = std::env::args_os().collect();
    if args.len() < 4 {
        print_error_and_exit("Usage: yazi-quadrants <file_path> <pixel_w> <pixel_h>");
    }

    let file_path = &args[1];

    if !Path::new(file_path).exists() {
        print_error_and_exit("Error: Video file does not exist or is inaccessible.");
    }

    let pixel_w: u32 = args[2].to_string_lossy().parse().unwrap_or(80);
    let pixel_h: u32 = args[3].to_string_lossy().parse().unwrap_or(40);

    let mut ffmpeg = match Command::new("ffmpeg")
        .args(&[
            "-loglevel", "quiet",
            "-re",
            "-r", "10",
            "-i"
        ])
        .arg(file_path)
        .args(&[
            "-f", "rawvideo",
            "-pix_fmt", "rgb24",
            "-s", &format!("{}x{}", pixel_w, pixel_h),
            "-"
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn() 
    {
        Ok(child) => child,
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                print_error_and_exit(
                    "Error: 'ffmpeg' is not installed or not found in PATH.\n\
                     Please install FFmpeg using your package manager."
                );
            } else {
                let err_msg = format!("Error: Failed to spawn FFmpeg: {}", e);
                print_error_and_exit(&err_msg);
            }
        }
    };

    let mut stdin = ffmpeg.stdout.take().unwrap();
    let frame_size = (pixel_w * pixel_h * 3) as usize;
    let mut raw_buffer = vec![0u8; frame_size];
    let mut text_buffer = String::with_capacity(frame_size * 2);
    let mut stdout = io::stdout().lock();

    let mut decoded_frames_count = 0;

    while stdin.read_exact(&mut raw_buffer).is_ok() {
        decoded_frames_count += 1;
        text_buffer.clear();
        quadrants::render_quadrants(&mut text_buffer, &raw_buffer, pixel_w * 3, pixel_w, pixel_h);

        if write!(stdout, "{}", text_buffer).is_err() { break; }
        if write!(stdout, "__FRAME_END__\n").is_err() { break; }
        let _ = stdout.flush();
    }

    let _ = ffmpeg.kill();

    if decoded_frames_count == 0 {
        print_error_and_exit(
            "Error: Failed to decode video.\n\
             The file might be corrupted, or the codec is not supported."
        );
    }

    Ok(())
}
