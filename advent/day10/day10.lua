-- Helper class for printing
local function tprint(tbl, indent)
	if tbl == nil then
		return "nil"
	elseif type(tbl) ~= "table" then
		return tbl
	end
	if tbl.x ~= nil then
		local dirstring = ""
		if tbl.connection.west then
			dirstring = dirstring .. "⬅️"
		else
			dirstring = dirstring .. "❌"
		end
		if tbl.connection.north then
			dirstring = dirstring .. "⬆️"
		else
			dirstring = dirstring .. "❌"
		end
		if tbl.connection.east then
			dirstring = dirstring .. "➡️"
		else
			dirstring = dirstring .. "❌"
		end
		if tbl.connection.south then
			dirstring = dirstring .. "⬇️"
		else
			dirstring = dirstring .. "❌"
		end
		return "Node "
			.. tbl.direction
			.. "  "
			.. tbl.x
			.. "  "
			.. tbl.y
			.. "  "
			.. dirstring
			.. " #neighbors "
			.. #tbl.neighbors
			.. " min_distance "
			.. tprint(tbl.min_distance)
	end
	if not indent then
		indent = 0
	end
	local toprint = string.rep(" ", indent) .. "{\r\n"
	indent = indent + 2
	for k, v in pairs(tbl) do
		toprint = toprint .. string.rep(" ", indent)
		if type(k) == "number" then
			toprint = toprint .. "[" .. k .. "] = "
		elseif type(k) == "string" then
			toprint = toprint .. k .. "= "
		end
		if type(v) == "number" then
			toprint = toprint .. v .. ",\r\n"
		elseif type(v) == "string" then
			toprint = toprint .. '"' .. v .. '",\r\n'
		elseif type(v) == "table" then
			toprint = toprint .. tprint(v, indent + 2) .. ",\r\n"
		else
			toprint = toprint .. '"' .. tostring(v) .. '",\r\n'
		end
	end
	toprint = toprint .. string.rep(" ", indent - 2) .. "}"
	return toprint
end
local function D(tbl, prefix)
	if prefix == nil then
		prefix = ""
	end
	print(prefix, tprint(tbl, 0))
end

-- Node class
Node = {}
Node.__index = Node

function Node.new(x, y, direction)
	local self = setmetatable({}, Node)
	self.x = x
	self.y = y
	self.direction = direction
	self.connection = { north = false, south = false, west = false, east = false }
	self.min_distance = nil
	self.neighbors = {}
	self.is_loop_node = false
	self.inside_or_outside = nil
	self.up_or_down = nil
	return self
end

-- Path class
Path = {}
Path.__index = Path

function Path.new()
	local self = setmetatable({}, Path)
	self.nodes = {}
	return self
end

function Path:is_last(node)
	if node == self.nodes[#self.nodes - 1] then
		return true
	else
		return false
	end
end

function Path:pair_exists(firstnode, nextnode)
	for i, n in ipairs(self.nodes) do
		if n == nextnode and self.nodes[i - 1] == firstnode then
			return true
		end
	end
	return false
end

function Path:walked_node(node)
	for _, n in ipairs(self.nodes) do
		if n == node then
			return true
		end
	end
	return false
end

function Path:distance()
	return #self.nodes
end

-- Maze class
Maze = {}
Maze.__index = Maze

function Maze.new()
	local self = setmetatable({}, Maze)
	self.width = nil
	self.height = nil
	self.nodes = {}
	self.loop_nodes = {}
	self.start_node = nil
	self.curr_node = nil
	self.curr_path = Path.new()
	return self
end

function Maze:readFromFile(filename)
	print("read from file " .. filename)
	local file = io.open(filename, "r")
	if not file then
		return nil
	end
	local content = file:read("*all")
	file:close()

	local y = 1
	for line in content:gmatch("[^\n]+") do
		local x = 1
		for char in line:gmatch(".") do
			local is_start_node = false
			local node = Node.new(x, y, char)
			if char == "7" then
				node.connection.west = true
				node.connection.south = true
			elseif char == "L" then
				node.connection.east = true
				node.connection.north = true
			elseif char == "-" then
				node.connection.west = true
				node.connection.east = true
			elseif char == "|" then
				node.connection.north = true
				node.connection.south = true
			elseif char == "F" then
				node.connection.east = true
				node.connection.south = true
			elseif char == "J" then
				node.connection.north = true
				node.connection.west = true
			elseif char == "S" then
				node.connection.north = true
				node.connection.west = true
				node.connection.east = true
				node.connection.south = true
				is_start_node = true
			end
			table.insert(self.nodes, node)
			if is_start_node then
				self.start_node = node
			end
			x = x + 1
		end
		if self.width == nil then
			self.width = x - 1
		end
		y = y + 1
	end
	self.height = y - 1
	-- table.insert(self.curr_path.nodes, self.start_node)
end

function Maze:get_node(x, y)
	for _, n in ipairs(self.nodes) do
		if n.x == x and n.y == y then
			return n
		end
	end
end

function Maze:set_neighbors()
	print("set neighbors")
	local neigbor = nil
	for _, n in ipairs(self.nodes) do
		if n.direction ~= "." then
			if n.connection.north then
				neigbor = self:get_node(n.x, n.y - 1)
				if neigbor ~= nil and neigbor.connection.south then
					table.insert(n.neighbors, neigbor)
				end
			end
			if n.connection.south then
				neigbor = self:get_node(n.x, n.y + 1)
				if neigbor ~= nil and neigbor.connection.north then
					table.insert(n.neighbors, neigbor)
				end
			end
			if n.connection.west then
				neigbor = self:get_node(n.x - 1, n.y)
				if neigbor ~= nil and neigbor.connection.east then
					table.insert(n.neighbors, neigbor)
				end
			end
			if n.connection.east then
				neigbor = self:get_node(n.x + 1, n.y)
				if neigbor ~= nil and neigbor.connection.west then
					table.insert(n.neighbors, neigbor)
				end
			end
		end
	end
end

function Maze:print(typ)
	for y = 1, self.height do
		for x = 1, self.width do
			local node = self:get_node(x, y)
			if typ == "distance" and node.min_distance ~= nil then
				io.write(node.min_distance)
			elseif typ == "loop" and node ~= nil and node.is_loop_node then
				io.write("*")
			elseif typ == "up_or_down" and node ~= nil and node.up_or_down ~= nil then
				io.write(node.up_or_down)
			elseif typ == "inside" and node ~= nil and node.inside_or_outside ~= nil then
				io.write(node.inside_or_outside)
			elseif node ~= nil then
				io.write(node.direction)
			end
		end
		print("")
	end
end

function Maze:walk(curr_node, last_node)
	if curr_node == nil then
		curr_node = self.start_node
	end
	local viable_neighbors = {}
	for _, n in ipairs(curr_node.neighbors) do
		if n ~= last_node then
			table.insert(viable_neighbors, n)
		end
	end
	-- D(viable_neighbors, "viable_neighbors ")
	if #viable_neighbors > 1 then
		D(curr_node, "more than one")
	end
	for _, n in ipairs(viable_neighbors) do
		-- D(self.curr_path, "curr path")
		-- print(" ")
		-- D(curr_node, "curr_node")
		-- D(last_node, "last_node")
		-- D(n, "next_node")
		if #viable_neighbors > 1 then
			local new_path = Path.new()
			for _, x in ipairs(self.curr_path.nodes) do
				-- D(x, "copy path")
				if x == curr_node then
					break
				end
				table.insert(new_path.nodes, x)
			end
			self.curr_path = new_path
		end

		-- Move forward
		-- D(self.curr_path, "curr path")
		if curr_node.min_distance == nil or self.curr_path:distance() < curr_node.min_distance then
			curr_node.min_distance = self.curr_path:distance()
		end
		table.insert(self.curr_path.nodes, curr_node)
		if n ~= self.start_node then
			self:walk(n, curr_node)
		end
	end
end

function Maze:max_dist()
	local max = 0
	for _, n in ipairs(self.nodes) do
		if n.min_distance ~= nil and n.min_distance > max then
			max = n.min_distance
		end
	end
	return max
end

function Maze:get_loop_nodes()
	for _, n in ipairs(self.nodes) do
		if n.min_distance ~= nil then
			table.insert(self.loop_nodes, n)
			n.is_loop_node = true
		end
	end
end

function Maze:get_inside_nodes()
	for i, n in ipairs(self.curr_path.nodes) do
		-- D(n, "path_node")
		local last_node = self.curr_path.nodes[i - 1]
		local next_node = self.curr_path.nodes[i + 1]
		if last_node ~= nil then
			if n.y > last_node.y then
				n.up_or_down = "D"
			elseif n.y < last_node.y then
				n.up_or_down = "U"
			end
		end
		if next_node ~= nil then
			if next_node.y > n.y then
				n.up_or_down = "D"
			elseif next_node.y < n.y then
				n.up_or_down = "U"
			end
		end
	end
	local node = nil
	local cnt = 0
	for y = 1, self.height do
		local inside = false
		for x = 1, self.width do
			node = self:get_node(x, y)
			if node.up_or_down == "U" then
				inside = true
			end
			if node.up_or_down == "D" then
				inside = false
			end
			if not node.is_loop_node and inside then
				cnt = cnt + 1
				node.inside_or_outside = "I"
			elseif not node.is_loop_node then
				node.inside_or_outside = "O"
			else
				node.inside_or_outside = nil
			end
		end
	end
	return cnt
end

local input = arg[1]
if input == nil then
	input = "text_simple.txt"
end
MM = Maze.new()
MM:readFromFile(input)
MM:set_neighbors()
MM:walk()
MM:get_loop_nodes()
local cnt = MM:get_inside_nodes()
if input:sub(1, 4) == "text" then
	D(MM.curr_path, "path")
	print(" ")
	MM:print("loop")
	print(" ")
	MM:print("distance")
	print(" ")
	MM:print("up_or_down")
	print(" ")
	MM:print("inside")
	print(" ")
end
print("part 1=", MM:max_dist())
print("part 2=", cnt)
