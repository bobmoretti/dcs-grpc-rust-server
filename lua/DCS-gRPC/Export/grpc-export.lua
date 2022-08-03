local function init()
    log.write("[GRPC-export]", log.INFO, "Initializing ...")
  
    if not GRPC then
      _G.GRPC = {}
    end
  
    -- load settings from `Saved Games/DCS/Config/dcs-grpc.lua`
    do
      log.write("[GRPC-export]", log.INFO, "Checking optional config at `Config/dcs-grpc.lua` ...")
      local file, err = io.open(lfs.writedir() .. [[Config\dcs-grpc.lua]], "r")
      if file then
        local f = assert(loadstring(file:read("*all")))
        setfenv(f, GRPC)
        f()
        log.write("[GRPC-export]", log.INFO, "`Config/dcs-grpc.lua` successfully read")
      else
        log.write("[GRPC-export]", log.INFO, "`Config/dcs-grpc.lua` not found (" .. tostring(err) .. ")")
      end
    end
  
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
  
    dofile(GRPC.luaPath .. [[grpc-export.lua]])
  
    log.write("[GRPC-export]", log.INFO, "Initialized...")
  end
  
  local ok, err = pcall(init)
  if not ok then
    log.write("[GRPC-export]", log.ERROR, "Failed to Initialize: "..tostring(err))
  end
  
  