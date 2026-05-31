local M = {}

function M:peek(job)
    local target_w = job.area.w
    local target_h = job.area.h

    --fullscreen video 
    local pixel_w = target_w * 2
    local pixel_h = target_h * 2

    --OS find 
    local binary_path
    if ya.target_family() == "windows" then
        local appdata = os.getenv("APPDATA") or (os.getenv("USERPROFILE") .. "\\AppData\\Roaming")
        binary_path = appdata .. "\\yazi\\config\\plugins\\video-quadrants.yazi\\yazi-quadrants.exe"
    else
        local home = os.getenv("HOME") or ("/home/" .. (os.getenv("USER") or "user"))
        binary_path = home .. "/.local/bin/yazi-quadrants"
    end

    local opts = {
        area = job.area,
        file = job.file,
        mime = "video/*",
        skip = job.skip,
    }

    local child, err = Command(binary_path)
        :arg(tostring(job.file.url))
        :arg(tostring(pixel_w))
        :arg(tostring(pixel_h))
        :stdout(Command.PIPED)
        :spawn()

    -- error try catch
    if not child then
        local err_msg = "Error: 'yazi-quadrants' binary not found.\n"
        if ya.target_family() == "windows" then
            err_msg = err_msg .. "Please make sure 'yazi-quadrants.exe' is compiled and placed in:\n%APPDATA%\\yazi\\config\\plugins\\video-quadrants.yazi\\"
        else
            err_msg = err_msg .. "Please make sure the binary is installed in ~/.local/bin/ and added to PATH."
        end
        ya.preview_widget(opts, ui.Text.parse(err_msg):area(job.area))
        return
    end

    local frame_lines = {}
    
    while true do
        local line, event = child:read_line()
        if not line then break end

        -- Windows-moument (i dont test it on windows, say me if it broken)
        line = line:gsub("\r$", "")

        if line == "__FRAME_END__\n" then
            local frame_data = table.concat(frame_lines)
            ya.preview_widget(opts, ui.Text.parse(frame_data):area(job.area))
            frame_lines = {}
        else
            table.insert(frame_lines, line)
        end
    end
end

function M:seek(job)
end

return M
