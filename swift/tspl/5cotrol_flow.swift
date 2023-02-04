let num: Int
switch 666 {
case 0 ..< 10:
	num = 0
case 10 ..< 100:
	num = 10
case 100 ..< 1000:
	num = 100
default:
	num = 1
}
assert(num == 100)

let yetAnotherPoint = (1, -1)
switch yetAnotherPoint {
case let (x, y) where x == y:
	print("(\(x), \(y)) is on the line x == y")
case let (x, y) where x == -y:
	print("(\(x), \(y)) is on the line x == -y")
	fallthrough
case let (x, y):
	print("(\(x), \(y)) is just some arbitrary point")
	fallthrough
default:
	break
}

@available(macOS 13, *)
struct ColorPreference {
	var bestColor = "blue"
}

func chooseBestColor() -> String {
	guard #available(macOS 10.12, *) else {
		return "gray"
	}
	let colors = ColorPreference()
	return colors.bestColor
}

assert(chooseBestColor() == "blue")
