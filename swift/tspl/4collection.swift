// array

var ary_of_int: [Int] = []; ary_of_int.append(1)
assert(ary_of_int == [1]); assert(ary_of_int.count == 1)
ary_of_int += [3, 6, 9]
assert(ary_of_int == [1, 3, 6, 9])
let one = ary_of_int.remove(at: 0); assert(one == 1)

var letters: Set = ["a", "b", "os"]

// set operations

let odds: Set = [1, 3, 5, 7, 9], primes: Set = [2, 3, 5, 7, 11]
assert(odds.union(primes).sorted() == [1, 2, 3, 5, 7, 9, 11])
assert(odds.intersection(primes).sorted() == [3, 5, 7])
assert(odds.symmetricDifference(primes).sorted() == [1, 2, 9, 11])
assert(odds.subtracting(primes).sorted() == [1, 9])

// directory

var names_of_integers = [0: "zero"]
names_of_integers[2] = "two"
let old_zero = names_of_integers.updateValue("rei", forKey: 0)
