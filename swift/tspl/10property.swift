// MARK: - Point
struct Point {
	var x = 0.0, y = 0.0
}

// MARK: - Size
struct Size {
	var width = 0.0, height = 0.0
}

// MARK: - Rect
struct Rect {
	var origin = Point()
	var size = Size()

	var center: Point {
		get {
			Point(x: origin.x + size.width / 2, y: origin.y + size.height / 2)
		}
		set {
			origin.x = newValue.x - size.width / 2
			origin.y = newValue.y - size.height / 2
		}
	}
}

var square = Rect(origin: Point(x: 0.0, y: 0.0), size: Size(width: 10, height: 10))
assert(square.center.x == 5); assert(square.center.y == 5)
square.center = Point(x: 15, y: 15)
assert(square.origin.x == 10); assert(square.origin.y == 10)

// MARK: - Cuboid
struct Cuboid {
	var width = 0.0, height = 0.0, depth = 0.0

	var volume: Double {
		width * height * depth
	}
}

let cube = Cuboid(width: 4, height: 4, depth: 4); assert(cube.volume == 64)

// MARK: - TwelveOrLess
@propertyWrapper
struct TwelveOrLess {
	// MARK: Lifecycle
	init(wrappedValue: Int) {
		self.wrappedValue = wrappedValue
	}

	// MARK: Internal
	var wrappedValue: Int {
		get { num }
		set { num = min(newValue, 12) }
	}

	// MARK: Private
	private var num = 0
}

// MARK: - SmallRect
struct SmallRect {
	@TwelveOrLess(wrappedValue: 4) var height: Int
	@TwelveOrLess(wrappedValue: 7) var width: Int
}

var small_rect = SmallRect(); small_rect.height = 20; assert(small_rect.height == 1)

print("🫠")
