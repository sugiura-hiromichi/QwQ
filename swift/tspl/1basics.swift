let a = 1, b = "he", c = (3, 3)
let 🫠 = "melt"
print(🫠, terminator: "")
print(🫠, separator: " + ")

// I like this shell script style semicolons

let end = 9; assert(end == 9)

let min_uint8 = UInt8.min; assert(min_uint8 == 0)
let max_uint8 = UInt8.max; assert(max_uint8 == 255)
let this_is_double = 3 + 0.14
let dec = 666
let bin_dec = 0b1010
let oct_dec = 0o32
let hex_dec = 0xA02
let sci = 2e3; assert(sci == 2000)

typealias i32 = Int32
let i32_max = i32.max

/// this is exciting
let named_element_of_tuple = (fst: 3, snd: "3")

/// this value is optionals
var converted = i32("12"); assert(converted! == 12)
/// if declare as constant, compiler emit error
var implicit_nil: Int?; assert(implicit_nil == nil)

if let converted {
	assert(converted == 12)
}

if var converted {
	converted = 13
	assert(converted == 13)
}

assert(converted! == 12)

/// implicitly unwrapped optional
let assumed_string: String! = "implicitly unwrapped"
let implicit_string: String = assumed_string

func throw_error() throws {}

// assertion is ignored on release build, but precondition is not even in release build
