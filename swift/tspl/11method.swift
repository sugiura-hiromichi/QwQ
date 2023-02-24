// MARK: - Counter
class Counter {
	var count = 0

	func increment() {
		count += 1
	}

	func increment(by amount: Int) {
		count += amount
	}

	func reset() {
		count = 0
	}
}

var c = Counter(); c.increment(by: 5); assert(c.count == 5)

// MARK: - Point
struct Point {
	var x = 0.0, y = 0.0

	func is_right(of: Double) -> Bool {
		x > of
	}

	mutating func move_by(delx: Double, dely: Double) {
		x += delx
		y += dely
	}
}

var somePoint = Point(x: 6, y: 0)
assert(somePoint.is_right(of: 2))
somePoint.move_by(delx: 5, dely: 11); assert(somePoint.is_right(of: 10))

print("⚰️")
