// string
assert("\u{24}" == "$")
assert("".isEmpty)

/// `char` is `Character` in swift
let characters: [Character] = ["c", "h", "a", "r"]
let char = String(characters); assert(char == "char")

/// string cannot be indexed by integer
let greet = "what?"
assert(greet[greet.startIndex] == "w")
assert(greet[greet.index(before: greet.endIndex)] == "?")
assert(greet[greet.index(after: greet.startIndex)] == "h")
let idx = greet.index(greet.startIndex, offsetBy: 2); assert(greet[idx] == "a")

/// insert and remove
var encounter = "oh my god"
encounter.insert("!", at: encounter.endIndex)
encounter.insert(
	contentsOf: " what a hell",
	at: encounter.index(before: encounter.endIndex)
)
assert(encounter == "oh my god what a hell!")

encounter.remove(at: encounter.index(before: encounter.endIndex))
let erange = encounter.index(encounter.endIndex, offsetBy: -6) ..< encounter.endIndex
encounter.removeSubrange(erange)
assert(encounter == "oh my god what ")

let e_idx = encounter.firstIndex(of: " ") ?? encounter.endIndex
let splitted_with_ws = encounter[..<e_idx]
assert(splitted_with_ws == "oh")
