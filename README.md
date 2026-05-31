# yazi-quadrants

A high-performance, lightweight, and cross-platform video previewer plugin for the **Yazi** file manager. It renders video frames in real-time inside your terminal preview pane using **ANSI quadrants** (Unicode 2x2 block characters).

It relies on a custom highly optimized **Rust** engine for fast Block Truncation Coding (BTC) with Run-Length Encoding (RLE) to deliver rich, high-resolution text-based previews without hogging your CPU.

## Features

- **Ultra High Quality:** Renders 2x2 sub-pixels inside a single terminal character cell (effectively doubling your terminal resolution).
- **GPU & CPU Friendly:** Built-in RLE compression reduces ANSI color escape sequence bandwidth by up to 70-80%, preventing terminal lag.
- **Zero Orphans:** The Rust engine automatically spawned by Yazi directly manages the lifecycle of `ffmpeg`, ensuring no background zombie processes are left behind on exit.
- **Pure Native UI Integration:** No direct-to-TTY hacks. It pipes frames directly into Yazi's virtual layout, letting Yazi handle screen updates, overlays, and perfect cleanup.
- **Cross-Platform:** Works on Linux, macOS, and Windows.

## Requirements

- **Rust** (cargo) to compile the renderer.
- **FFmpeg** installed and available in your system `PATH`.

## Installation

Run the one-liner command below corresponding to your operating system to automatically clone, compile, and place files in their correct folders.

### Linux / macOS
curl -sSfL https://raw.githubusercontent.com/GotchanBata/yazi-quadrants/main/install.sh | bash

Windows (PowerShell)
code Powershell

irm https://raw.githubusercontent.com/GotchanBata/yazi-quadrants/main/install.ps1 | iex

Configuration

Add the following lines to your Yazi configuration file (~/.config/yazi/yazi.toml or %APPDATA%\yazi\config\yazi.toml):
code Toml

[plugin]
prepend_previewers = [
    { mime = "video/*", run = "video-quadrants" }
]

How It Works Under the Hood

    Yazi spawns the compiled yazi-quadrants binary.

    The binary runs ffmpeg internally to decode the video stream at 10 FPS in rgb24 raw format.

    The Rust engine processes the 2x2 sub-pixel blocks using ITU-R BT.601 luma formulas, clusters colors into optimal foreground/background groups, and formats them into ANSI text.

    The output is streamed back to Yazi's Lua coroutine via standard pipe, which leverages native Yazi ui.Text.parse() to display it safely in the UI.
