import numpy


def AND(x1, x2):
	x = numpy.array([x1, x2])
	w = numpy.array([.5, .5])
	b = -.7
	if numpy.sum(w * x) + b <= 0:
		return 0
	else:
		return 1


assert AND(0, 0) == 0
assert AND(0, 1) == 0
assert AND(1, 0) == 0
assert AND(1, 1) == 1


def NAND(x1, x2):
	x = numpy.array([x1, x2])
	w = numpy.array([-.5, -.5])
	b = .7
	if numpy.sum(w * x) + b <= 0:
		return 0
	else:
		return 1


assert NAND(0, 0) == 1
assert NAND(0, 1) == 1
assert NAND(1, 0) == 1
assert NAND(1, 1) == 0


def OR(x1, x2):
	x = numpy.array([x1, x2])
	w = numpy.array([.5, .5])
	b = -.4
	if numpy.sum(w * x) + b <= 0:
		return 0
	else:
		return 1


assert OR(0, 0) == 0
assert OR(0, 1) == 1
assert OR(1, 0) == 1
assert OR(1, 1) == 1

# d: multi layered perceptron


def XOR(x1, x2):
	return AND(NAND(x1, x2), OR(x1, x2))


assert XOR(0, 0) == 0
assert XOR(0, 1) == 1
assert XOR(1, 0) == 1
assert XOR(1, 1) == 0

print('🫠🫠🫠🫠🫠')
