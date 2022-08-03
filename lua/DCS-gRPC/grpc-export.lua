if not GRPC then
    GRPC = {}
end

local function writeLog(level, message)
    log.write("[GRPC-export]", level, message)
end

writeLog(log.INFO, "TETSTAST")

-- load settings from `Saved Games/DCS/Config/dcs-grpc.lua`
-- do
--   writeLog(log.INFO,"Checking optional config at `Config/dcs-grpc.lua` ...")
--   local file, err = io.open(lfs.writedir() .. [[Config\dcs-grpc.lua]], "r")
--   if file then
--     local f = assert(loadstring(file:read("*all")))
--     setfenv(f, GRPC)
--     f()
--     writeLog(log.INFO,"`Config/dcs-grpc.lua` successfully read")
--   else
--     writeLog(log.INFO,"`Config/dcs-grpc.lua` not found (" .. tostring(err) .. ")")
--   end
-- end

-- Set default settings.
if not GRPC.luaPath then
    GRPC.luaPath = lfs.writedir() .. [[Scripts\DCS-gRPC\]]
end
if not GRPC.dllPath then
    GRPC.dllPath = lfs.writedir() .. [[Mods\tech\DCS-gRPC\]]
end
if GRPC.throughputLimit == nil or GRPC.throughputLimit == 0 or not type(GRPC.throughputLimit) == "number" then
    GRPC.throughputLimit = 600
end

-- Let DCS know where to find the DLLs
if not string.find(package.cpath, GRPC.dllPath) then
    package.cpath = package.cpath .. [[;]] .. GRPC.dllPath .. [[?.dll;]]
end

-- Load DLL before `require` gets sanitized.
local ok, grpc = pcall(require, "dcs_grpc_hot_reload")
-- local grpc = require("dcs_grpc_hot_reload")
if grpc then
    writeLog(log.INFO, "loaded hot reload DLL " .. tostring(grpc))
else
    grpc = require("dcs_grpc")
    writeLog(log.INFO, "loaded regular DLL")
end

local loaded = false
function GRPC.load()
    if loaded then
        writeLog(log.INFO, "already loaded")
        return
    end

    local env = setmetatable({
        grpc = grpc,
        lfs = lfs
    }, {
        __index = _G
    })
    local f = setfenv(assert(loadfile(GRPC.luaPath .. [[grpc.lua]])), env)
    f()

    loaded = true
end

do
    local function writeLog(level, msg)
        log.write("[gRPC-Export]", level, msg)
    end

    writeLog(log.INFO, "doing export")

    -- (Hook) Called once right before mission start.
    do
        local PrevLuaExportStart = LuaExportStart
        LuaExportStart = function()
            writeLog(log.INFO, "On LuaExportStart")
            if PrevLuaExportStart then
                PrevLuaExportStart()
            end
        end
    end

    -- (Hook) Called right after every simulation frame.
    do
        local PrevLuaExportAfterNextFrame = LuaExportAfterNextFrame
        LuaExportAfterNextFrame = function()


            if GRPC.onSimulationFrameExport then
                GRPC.onSimulationFrameExport()
            end
            if PrevLuaExportAfterNextFrame then
                PrevLuaExportAfterNextFrame()
            end
        end
    end

    -- (Hook) Called right after mission end.
    do
        local PrevLuaExportStop = LuaExportStop

        LuaExportStop = function()

            if PrevLuaExportStop then
                PrevLuaExportStop()
            end
        end
    end

end

if GRPC.autostart == true then
    writeLog(log.INFO, "auto starting")
    GRPC.load()
end
