// MARK: - Vehicle
class Vehicle {
	var speed = 0.0

	var desc: String {
		"\(speed) km/h"
	}

	func make_noise() -> String { "sounds like" }
}

let some_vehicle = Vehicle()

// MARK: - Bicycle
class Bicycle: Vehicle {
	var has_basket = false
}

let bicycle = Bicycle()

// MARK: - Tandem
class Tandem: Bicycle {
	var cur_passengers = 0

	override func make_noise() -> String {
		super.make_noise() + " gowu gowu"
	}
}

let tandem = Tandem(); assert(tandem.make_noise() == "sounds like gowu gowu")
tandem.has_basket = true
tandem.cur_passengers = 1
tandem.speed = 100

print("|> 🫠")
