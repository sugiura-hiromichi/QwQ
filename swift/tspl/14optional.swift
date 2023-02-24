// MARK: - Person
class Person {
	var residence: Residence?
}

// MARK: - Residence
class Residence {
	var number_or_rooms = 1
}

let john = Person()
if let room_count = john.residence?.number_or_rooms {
	assertionFailure("this should be fail |> room_count is \(room_count)")
}

john.residence = Residence(); assert(john.residence?.number_or_rooms == 1)

print("|> 🫠")
