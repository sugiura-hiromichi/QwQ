local i = string.find('ala ma kota', 'kota')
assert(i == 8, 'string.find failed')

os.execute('say "使用中のターミナルエミュレーター。"' .. os.getenv 'TERM_PROGRAM')
print '🫠'
