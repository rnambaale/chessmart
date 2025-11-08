--[[
  Update game state and sequence number, performing an atomic Check-And-Set operation.
  Only update game if provided sequence number is correct (=current seq + 1) to prevent multiple concurrent moves.

  Input:
    KEYS[1] game key

    ARGV[1] new game state representation
    ARGV[2] sequence number

  Output:
    0 - unexpected sequence number
    1 - game correctly updated
]]

local gameKey = KEYS[1]
local newGameRepr = ARGV[1]
local seq = ARGV[2]

local oldSeq = redis.call('HGET', gameKey, 'seq')

if tonumber(seq) ~= tonumber(oldSeq) + 1 then
  return 0
end

redis.call('HSET', gameKey, 'gameRepr', newGameRepr, 'seq', seq)
return 1
