--[[
  Remove player from queue.

  Input:
    KEYS[1] queue key from which players should be removed
    KEYS[2] hash key where queue join times are stored
    KEYS[3] key that holds account status

    ARGV[1] account id of the player to remove
    ARGV[2] game type of the queue the player is in
    ARGV[3] whether the queue is ranked or not

  Output:
    1 - player successfully removed from the queue
    0 - number not removed from the queue (player wasn't in the queue)
]]

local queueKey = KEYS[1]
local timesKey = KEYS[2]
local accountStatusKey = KEYS[3]
local accountId = ARGV[1]
local gameType = ARGV[2]
local ranked = ARGV[3]

local currStatus, currGameType, currRanked = unpack(redis.call('HMGET', accountStatusKey, 'status', 'gameType', 'ranked'))
if currStatus == 'searching' and currGameType == gameType and currRanked == ranked then
  redis.call('DEL', accountStatusKey)
end

local result = redis.call('ZREM', queueKey, accountId)
redis.call('HDEL', timesKey, accountId)

return result
