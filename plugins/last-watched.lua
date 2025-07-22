local VIDEO_EXTENSIONS = { "mp4", "mkv", "avi", "webm", "flv", "mov", "wmv" }

local function is_video_file(file)
    local extention = file:match("^.+%.(.+)$")

    for _, ext in ipairs(VIDEO_EXTENSIONS) do
        if ext == extention then
            return true
        end
    end

    return false
end

local function on_file_loaded(event)
    -- Get the extention and folder of the file
    local file = mp.get_property("filename")

    local extention = file:match("^.+%.(.+)$")
    if extention == nil or file == nil or not is_video_file(file) then
        return
    end

    -- Load the sidecar file
    local success, lines = pcall(io.lines, '.watched')

    -- Check for the current file in the sidecar file, returning if it is already there
    if success then
        for line in lines do
            if line == file then
                mp.osd_message("Already watched")
                return
            end
        end
    end

    mp.osd_message("Marking as watched")
    local sidecar = assert(io.open('.watched', "a+"))
    sidecar:write(file .. "\n")
    sidecar:close()
end

mp.register_event("file-loaded", on_file_loaded)
