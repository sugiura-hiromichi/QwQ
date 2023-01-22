#!swift
// # A Swift Tour

// ## d: Simple Values

var variable = 9
variable = 8
let constant = 6

let explicit_double: Double = 70

let lab = "the width is "
let width_label = lab + String(77)
assert(width_label == "the width is 77")

let kick = 55
let bass = 100
assert("\(kick + bass)" == "155")

let multiline = """
multiline strings are surrounded by three quotations
"""

var sight_array = ["kyoto", "sendai", "sapporo"]
sight_array[0] = "hirosima"
/// sight_array[3] = "Wazima" // this panic. If you want to add element to array, use `append`
sight_array.append("wazima")
let empty_array: [String] = []
let empty_dictionary: [Int: Float] = [:]

func take_ary(ary: [String]) {
	assert(ary == [])
}

take_ary(ary: [])

var sight_dict = ["Hirosima": "hirosima", "Miyagi": "sendai", "Hokkaido": "sapporo"]
sight_dict["Hirosima"] = "kure"
assert(sight_dict["Hirosima"] == "kure")

// ## d: Control Flow

let scores = [6, 66, 666, 6666]
var team = 0
/*
 lua vim.lsp.start{name='sourcekit-lsp',cmd={'sourcekit-lsp'},root_dir=vim.fs.dirname(vim.fs.find({'Package.swift'},{upward=true})[1])}
 */
for scr in scores {
	if scr != 666 { continue }
	team += scr
}

assert(team == 666)

/// optional!
let optional_str: String? = "Hell"
if let sky = optional_str { assert("Hell is \(sky)" == "Hell is Hell") }
else { assert(optional_str == nil) }

let friends: String? = nil
let imaginaly_one = "A B C"
let true_friends = "\(friends ?? imaginaly_one)"

/// if let shorthand
if let friends { assert(friends == "You") }
if let optional_str { assert(optional_str == "Hell") }

let you = "You"
switch you {
case let you where you.hasSuffix("u"):
	assert(you == "You")
default:
	assertionFailure()
}

let fav_langs = [
	"natural": ["japanese", "chinese", "english"],
	"virtual": ["nico"],
	"program": ["rust", "lua", "swift"],
]
for (k, v) in fav_langs {
	var i = 0
	for l in v {
		assert(fav_langs[k]?[i] == l)
		i += 1
	}
}

var n = 3
while n < 1000 { n *= n }
assert(n == 6561)

n = 2
repeat { n *= n } while n < 1000
assert(n == 65536)

// exclusive range
n = 0
for i in 0 ..< 4 { n += i }
assert(n == 6)
// inclusive range
for i in 0 ... 4 { n += i }
assert(n == 16)

// ## d: Functions and Closures

func awa(face: String?, _ emotion: Int) -> (face: String?, emo: Int, res: String) {
	var res = "\(emotion)"
	if let face {
		res += " with \(face)"
	}
	let ret = (face, emotion, res)
	return ret
}

assert(awa(face: nil, 3).2 == "3")
assert(awa(face: nil, 3).face == nil)
assert(awa(face: "†", 3).res == "3 with †")

func nested_triplet(_ a_value: Int) -> Int {
	var sum = a_value
	// nested function can access outer function's variables
	// same as rust's closure
	func add() { sum += a_value }
	add(); add()
	return sum
}

assert(nested_triplet(4) == 12)

func fn_generatior(_ f: () -> Int) -> ((Int) -> Int) {
	let f_rslt = f()
	func pow_2(_ i: Int) -> Int { f_rslt * i }
	return pow_2
}

func one() -> Int { 1 }
assert(fn_generatior(one)(5) == 5)

var nums = [20, 30, 50, 60].map { n in n / 10 }
assert(nums == [2, 3, 5, 6])

/// $1 points second argument. $0 points first argument
let sorted = nums.sorted { $1 < $0 }.map { $0 * 10 }
assert(sorted == [60, 50, 30, 20])

// MARK: - Favs
// ## d: Objects and Classes

class Favs {
	// MARK: Lifecycle
	init(add: [String]) {
		additional_langs = add
	}

	// MARK: Internal
	var fav_langs = 3
	let default_langs = ["swift", "rust", "lua", "markdown"]
	var additional_langs: [String]
	var most_fav = "rust"

	func dflts() -> [String] { default_langs }
}

var my_fav = Favs(add: [])
assert(my_fav.dflts() == ["swift", "rust", "lua", "markdown"])

// MARK: - Langs
class Langs: Favs {
	// MARK: Lifecycle
	init(add: [String], nl: Int) {
		natural_langs = nl
		super.init(add: add)
	}

	// MARK: Internal
	let dflt_langs = ["japanese", "english"]
	var natural_langs: Int

	var boil: Int { get { 1 + natural_langs } set(n) { natural_langs += n }}

	func dflts() -> Int {
		dflt_langs.count
	}
}

let nlang = Langs(add: [], nl: 0)
let ary: [String] = nlang.dflts()
let integer: Int = nlang.dflts()
assert(ary == ["swift", "rust", "lua", "markdown"])
assert(integer == 2)
assert(nlang.boil == 1)
nlang.boil = 8
assert(nlang.boil == 9)

// MARK: - Rank
// ## d: Enumerations and Structures

enum Rank: Int {
	case ace = 1
	case two, three, four, five, six, seven, eight, nine, ten
	case jack, queen, king

	// MARK: Internal
	func simple_desc() -> String {
		switch self {
		case .ace: // NOTE: this abbreviation is able anytime the type is already known
			return "ace"
		case .jack:
			return "jack"
		case .queen:
			return "queen"
		case .king:
			return "king"
		default:
			return "\(rawValue)"
		}
	}
}

assert(Rank.two.simple_desc() == "2")
assert(Rank.jack.rawValue == 11)

/// `init?(rawValue:)`
if let as_enum = Rank(rawValue: 3) {
	assert(as_enum.simple_desc() == "3")
}

assert(Rank(rawValue: 0) == nil)

let ace = Rank.ace
let two = Rank.two

// MARK: - ServerResponse
// this prints `ace` print("\(ace)")

enum ServerResponse {
	case rslt(String, String)
	case fail(String)
}

let suc = ServerResponse.rslt("130", "666")
let err = ServerResponse.fail("Error")

switch suc {
case let .rslt(min, max):
	assert((min, max) == ("130", "666"))
case let .fail(emsg):
	print(emsg)
}

// MARK: - Card
/// a: structs are always passed by copy
///    classes are always passed by reference
struct Card {
	var rank: Rank
	var serv: ServerResponse
}

let card = Card(rank: .jack, serv: .fail("error")) // abbreviated

// ## d: Concurrency

/// label can be up to two words. last word is variable name inside function, first word is label of
/// this function
func fetch_usrid(from server: String) async -> Int {
	if server == "primary" {
		return 97
	}
	return 501
}

func fetch_usrnam(from server: String) async -> String {
	let usrid = await fetch_usrid(from: server)
	if usrid == 501 {
		return "Buddha"
	}
	return "Muhammad"
}

func cnct_usr(to server: String) async -> String {
	async let usrid = fetch_usrid(from: server)
	async let usrnm = fetch_usrnam(from: server)
	return await "usrid \(usrid) is named \(usrnm)"
}

let asy_response = await cnct_usr(to: "primary")
assert(asy_response == "usrid 97 is named Muhammad")

// functions called from inside of `Task` block returns without having us waiting
Task {
	print(await fetch_usrnam(from: "secondary"))
}

// MARK: - ExpProt
// ## d: Protocols and Extensions

protocol ExpProt {
	var desc_me: String { get }
	mutating func adjust()
}

// MARK: - SimpleCls
/// adoptation of protocols
class SimpleCls: ExpProt {
	var desc_me = "fiercely simple class😍"
	let ano_prop = 69105

	let class_original = "I want to be struct"

	/// method of `class` can always modify the class
	func adjust() {
		desc_me += " since it is adjustable"
	}
}

var a = SimpleCls()
a.adjust()
assert(a.desc_me == "fiercely simple class😍 since it is adjustable")

// MARK: - SmplStrc
struct SmplStrc: ExpProt {
	var desc_me = "here is a struct"
	let struct_original = "Hating the word 'struct' is .. ironic"

	/// method of `struct` required tobe marked as mutating if it modifies the struct
	mutating func adjust() {
		desc_me = " which is known as its simplicity"
	}
}

// MARK: - Int + ExpProt
/// extension is equal to rust's `impl <struct name> for <trait name>`
extension Int: ExpProt {
	var desc_me: String { "The number \(self)" }
	mutating func adjust() {
		self += 42
	}
}

var seven_and_seven = 7
seven_and_seven.adjust()
assert(seven_and_seven == 49)

func take_proto(prt: ExpProt) -> String {
	if prt.desc_me.contains("here is a struct") {
		return "struct"
	}
	return "class"
}

var class_or_struct: ExpProt = a
// a: this cause error because of class_or_struct is treated as type of ExpProt
// Thus even runtime type is SimpleCls, it cannot access method or member outside of ExpProt
// class_or_struct.class_original
assert(take_proto(prt: class_or_struct) == "class")
class_or_struct = SmplStrc()
// a: this cause error as mentioned above: class_or_struct.struct_original
assert(take_proto(prt: class_or_struct) == "struct")

// MARK: - HumanError
// ## d: Error Handling

/// we can represent error to adopt `Error` protocol
enum HumanError: Error {
	case oversight
	case doze
	case neglect
	case inexperienced
	case others
}

// MARK: - would_throw
func would_throw(job: Int, toPrinter printer_name: String) throws -> String {
	if printer_name == "Never Has Toner" {
		throw HumanError.doze
	}
	return "job sent"
}

do {
	_ = try would_throw(job: 404, toPrinter: "zNever Has Toner")
	throw HumanError.inexperienced
} catch HumanError.doze {
	print("here we stay...")
} catch let he as HumanError {
	print("oops! \(he)")
} catch {
	print(error)
}

let succeed = try? would_throw(job: 808, toPrinter: "")
let failed = try? would_throw(job: 404, toPrinter: "Never Has Toner")
assert(succeed == "job sent")
assert(failed == nil)

var is_liquid = true
let liquids = ["water", "earth", "you", "tear", "glass"]
func liq_judge(_ liq: String) -> Bool {
	is_liquid = true
	defer {
		is_liquid = false
	}
	let rslt = liquids.contains(liq)
	return rslt
}

assert(!liq_judge("my spirit"))
assert(is_liquid == false)

// ## d:Generics

func make_ary<Item>(repeating item: Item, times: Int) -> [Item] {
	var rslt: [Item] = []
	for _ in 0 ..< times { rslt.append(item) }
	return rslt
}

assert(make_ary(repeating: 666, times: 6) == [666, 666, 666, 666, 666, 666])
