--[[
  Match players based on their mmr and time spent in the queue.

  Input:
    KEYS[1] queue key from which players should be matched
    KEYS[2] hash key where queue join times are stored

    ARGV[1] starting mmr search range
    ARGV[2] mmr search range increase per second
    ARGV[3] maximum mmr delta between players

  Output:
    table - flat array of length % 2 == 0, where each pair of elements represents matched players
]]

local queueKey = KEYS[1]
local timesKey = KEYS[2]
local mmrRange = ARGV[1]
local rangeIncreasePerSecond = ARGV[2]
local maxMmrDelta = ARGV[3] / 2

local result = {}

local playersAndScores = redis.call('ZRANGE', queueKey, 0, -1, 'WITHSCORES')
local queueJoinTimesMsArray = redis.call('HGETALL', timesKey)
local queueJoinTimesMs = {}
for i = 1, #queueJoinTimesMsArray, 2 do
  queueJoinTimesMs[queueJoinTimesMsArray[i]] = queueJoinTimesMsArray[i + 1]
end

-- Calculate mmr search range for a player, based on mmr and time spent in the queue
local function getPlayerRanges(id, mmr, currMs)
  if id == nil then
    return nil
  end
  -- Increase mmr search range based on time spent in the queue
  local mmrRangeBonus = math.floor((currMs - (queueJoinTimesMs[id] or 0)) / 1000) * rangeIncreasePerSecond
  return {
    id = id,
    lowerBound = math.max(mmr - mmrRange - mmrRangeBonus, mmr - maxMmrDelta),
    upperBound = math.min(mmr + mmrRange + mmrRangeBonus, mmr + maxMmrDelta)
  }
end

-- Calculate current time in milliseconds after all redis calls have been done
local seconds, microseconds = unpack(redis.call('TIME'))
local nowMs = math.floor(seconds * 1000 + microseconds / 1000)

local i = 1
while playersAndScores[i + 2] do
  local currPlayer = getPlayerRanges(playersAndScores[i], playersAndScores[i + 1], nowMs)
  local nextPlayer = getPlayerRanges(playersAndScores[i + 2], playersAndScores[i + 3], nowMs)
  
  -- Match players if their mmr search ranges overlap
  if nextPlayer ~= nil then
    if currPlayer.upperBound >= nextPlayer.lowerBound then
      table.insert(result, currPlayer.id)
      table.insert(result, nextPlayer.id)
      -- Skip next player (already matched with current one)
      i = i + 2
    end
  end
  -- Proceed with next player
  i = i + 2
end

return result
