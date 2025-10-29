--[[
  Add a player to matchmaking queue.

  Input:
    KEYS[1] queue key where player should be added
    KEYS[2] hash key where queue join times are stored
    KEYS[3] key that holds account status

    ARGV[1] new status of the player after joining the queue
    ARGV[2] account id of the player
    ARGV[3] ranking of the player
    ARGV[4] queue game type
    ARGV[5] queue ranked type (true/false)

  Output:
    string - current account status if account could not be added to the queue
    nil - account successfully added to the queue
]]

local queueKey = KEYS[1]
local timesKey = KEYS[2]
local accountStatusKey = KEYS[3]
local newStatus = ARGV[1]
local accountId = ARGV[2]
local mmr = ARGV[3]
local gameType = ARGV[4]
local ranked = ARGV[5]

-- If player is already in any state (playing, pending, searching, etc.) do not add him to the queue
local accountStatus = redis.call('HGET', accountStatusKey, 'status')
if accountStatus then
  return accountStatus
end

-- Calculate current time in milliseconds
local timeCommandResult = redis.call('TIME')
local nowMs = math.floor(timeCommandResult[1] * 1000 + timeCommandResult[2] / 1000)

-- Add player to queue and store join time
redis.call('ZADD', queueKey, mmr, accountId)
redis.call('HSET', timesKey, accountId, nowMs)

-- Set new account status, expiring in 1 day
redis.call('HSET', accountStatusKey, 'status', newStatus, 'gameType', gameType, 'ranked', ranked)
redis.call('EXPIRE', accountStatusKey, 86400)

return nil
