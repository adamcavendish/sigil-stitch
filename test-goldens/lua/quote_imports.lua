local json = require("dkjson");
local inspect = require("inspect");

local data = json.decode(input)
inspect(data)
