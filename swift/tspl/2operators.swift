var a = 3, b = 2
(a, b) = (6, 6)
assert(a == 6); assert(b == 6)

/// overflow operators allow value to be oveflowed
let c = a &+ b

let coalesce = Optional(3) ?? 0; assert(coalesce == 3)

let closed_range = 1 ... 5
var j = 1
for i in closed_range {
	assert(i == j)
	j += 1
}

let open_range = 1 ..< 5
