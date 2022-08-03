local DCS = DCS
local GRPC = GRPC
local JSON = loadfile([[Scripts\JSON.lua]])()

GRPC.methods.exportEval = function(params)
    local fn, err = loadstring(params.lua)
    if not fn then
      return GRPC.error("Failed to load Lua code: "..err)
    end
  
    local ok, result = pcall(fn)
    if not ok then
      return GRPC.error("Failed to execute Lua code: "..result)
    end
  
    return GRPC.success(JSON:encode(result))
  end
