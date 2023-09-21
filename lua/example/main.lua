local i = string.find('ala ma kota', 'kota')
assert(i == 8, 'string.find failed')

os.execute('say "使用中のターミナルエミュレーター。"' .. os.getenv 'TERM_PROGRAM')

local f = assert(io.open('io_open.txt', 'w+'), 'could not open io_open.txt')
f:write 'fst'
f:close()

local f2 = assert(io.open('io_open.txt', 'w+'), 'could not open io_open.txt')
f2:write 'snd'
f2:close()

local f3 = assert(io.open('io_open.txt', 'r'), 'could not open io_open.txt')
local s = f3:read '*a'
assert(s == 'snd', 'io.open failed')
print '🫠'
