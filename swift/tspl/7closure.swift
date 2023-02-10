let names = ["Chris", "Alex", "Ewa", "Barry", "Daniella"]

/// this shorthand is so amazing. treat operator as function
let reversed = names.sorted { $0 > $1 }
assert(reversed == ["Ewa", "Daniella", "Chris", "Barry", "Alex"])

// trailing closure

func some_f(cl: () -> Void) {
	cl()
}

some_f {
	assert(true)
}

let digitNames = ["Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"]
let numbers = [16, 58, 510]

let strs = numbers.map { n -> String in
	var n = n
	var rslt = ""
	repeat {
		rslt = digitNames[n % 10] + rslt
		n /= 10
	} while n != 0
	return rslt
}

assert(strs == ["OneSix", "FiveEight", "FiveOneZero"])

func inc(by: Int) -> () -> Int {
	var inc_val = 0
	func increnemter() -> Int {
		inc_val += by
		return inc_val
	}
	return increnemter
}

let inc_by_10 = inc(by: 10)
_ = inc_by_10(); _ = inc_by_10(); _ = inc_by_10(); assert(inc_by_10() == 40)
let inc_by_6 = inc(by: 6)
_ = inc_by_6(); _ = inc_by_6(); _ = inc_by_6(); assert(inc_by_6() == 24)

// closures are ref type (not value type)

let also_inc_10 = inc_by_10; assert(also_inc_10() == 50); assert(inc_by_10() == 60)

// escaping closure

var cmp_handlers: [() -> String] = []
func escaping_cl(_ cmp_handler: @escaping () -> String) {
	cmp_handlers.append(cmp_handler)
}

escaping_cl { "hello from func in array" }; assert(cmp_handlers.count == 1)
assert(cmp_handlers[0]() == "hello from func in array")

// more escaping

func someFunctionWithNonescapingClosure(closure: () -> Void) {
	closure()
}

// MARK: - SomeClass
class SomeClass {
	var x = 10

	func doSomething() {
		escaping_cl { self.x = 100; return "doSomething" }
		someFunctionWithNonescapingClosure { x = 200 }
	}
}

let instance = SomeClass(); instance.doSomething(); assert(instance.x == 200)
assert(cmp_handlers[1]() == "doSomething"); assert(instance.x == 100)

// MARK: - SomeOtherClass
class SomeOtherClass {
	var x = 10

	func doSomething() {
		escaping_cl { [self] in x = 100; return "do other thing" }
		someFunctionWithNonescapingClosure { x = 200 }
	}
}

// autoclosure

var customers = ["chris", "leon", "eida", "ethan"]; assert(customers.count == 4)
let custom_remove = { customers.remove(at: 0) }; assert(customers.count == 4)
assert(custom_remove() == "chris"); assert(customers.count == 3)

func serve(customer cstmr_prvdr: () -> String) {
	assert(cstmr_prvdr().count == 4)
}

serve(customer: custom_remove)

func other_serve(cstm: @autoclosure () -> String) {
	assert(cstm().count == 4)
}

other_serve(cstm: custom_remove())
other_serve(cstm: "lgbt")

print("|>reached eof")
