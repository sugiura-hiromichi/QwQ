// MARK: - Celsius
struct Celsius {
	// MARK: Lifecycle
	init(fahrenheit: Double) {
		temperature = (fahrenheit - 32.0) / 1.8
	}

	init(kelvin: Double) {
		temperature = kelvin - 273.15
	}

	init(_ celsius: Double) {
		temperature = celsius
	}

	// MARK: Internal
	var temperature: Double
}

// MARK: - Sub
class Sub: Celsius {
	override init() {
		super.init()
		temperature = 3
	}
}
