// MARK: - CompassPoint
enum CompassPoint {
	case north, south, east, west
}

var direction = CompassPoint.south
direction = .west
switch direction {
case .west:
	assert(true)
default:
	assertionFailure()
}

// MARK: - Beverage
enum Beverage: CaseIterable {
	case coffee, tea, juice
}

assert(Beverage.allCases.count == 3)

// MARK: - Barcode
enum Barcode {
	case upc(Int, Int, Int, Int)
	case qr_code(String)
}

// MARK: - ASCIISpecialCharacter
enum ASCIISpecialCharacter: Character {
	case tab = "\t"
	case line_feed = "\n"
	case carriage_return = "\r"
}

assert(ASCIISpecialCharacter.tab.rawValue == "\t")
assert(ASCIISpecialCharacter(rawValue: "\t")?.rawValue == "\t")

// MARK: - RawString
enum RawString: String {
	case text, char, str
}

assert(RawString.text.rawValue == "text")

print("|> eof")
