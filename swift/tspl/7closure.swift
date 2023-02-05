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
