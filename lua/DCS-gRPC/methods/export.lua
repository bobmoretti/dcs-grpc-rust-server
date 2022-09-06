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

GRPC.methods.listIndication = function(params)
  if not params.deviceId then
    return GRPC.error("Cannot find device id param")
  end
  local text = list_indication(params.deviceId)
  if not text then
    return GRPC.error("Device #" .. tostring(params.deviceId) .. "not found")
  end
  return GRPC.success({indication = text})
end

GRPC.methods.getArgumentValue = function(params)
  local device = GetDevice(params.deviceId)
  if not device then
    return GRPC.error("Device #" .. tostring(params.deviceId) .. " not found")
  end
  local value = device:get_argument_value(params.argument_id)
  return GRPC.success({value = value})
end

GRPC.methods.performClickableAction = function(params)
  local device = GetDevice(params.deviceId)
  if not device then
    return GRPC.error("Device #" .. tostring(params.deviceId) .. " not found")
  end
  device:performClickableAction(params.argument_id)
  return GRPC.success({})
end

GRPC.methods.getSelfData = function(params)
  local data = LoGetSelfData()

  if not data then
    return GRPC.error("Can't get player data")
  end
  local json = JSON:encode(data)
  if not json then
    return GRPC.error("Couldn't encode player data as json")
  end
  return GRPC.success(json)
end
