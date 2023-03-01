import numpy
import matplotlib.pylab as plt
import sys, os
from templates.dataset.mnist import load_mnist
from PIL import Image
import pickle


def step_fn(x):
	return numpy.array(x > 0,
	                   dtype=int)  # `numpy.int` was depreated and use `int`


def sigmoid(x):
	"""here is sigmoid"""
	return 1 / (1 + numpy.exp(-x))


def relu(x):
	return numpy.maximum(0, x)


#x = numpy.arange(-5.0, 5.0, 0.1)
#y = sigmoid(x)
#plt.plot(x, y)
#plt.ylim(-0.1, 1.1)
#plt.show()

# d: multi dimentional array

a = numpy.array([[1, 2], [3, 4], [5, 6]])
assert a.ndim == 2
assert a.shape == (3, 2)

b = numpy.array([[1, 2, 3], [4, 5, 6]])
c = numpy.dot(a, b)
d = numpy.dot(b, a)

# d: dot of neural network

x = numpy.array([1.0, .5])
w1 = numpy.array([[.1, .3, .5], [.2, .4, .6]])
b1 = numpy.array([.1, .2, .3])
a1 = numpy.dot(x, w1) + b1
z1 = sigmoid(a1)

w2 = numpy.array([[.1, .4], [.2, .5], [.3, .6]])
b2 = numpy.array([.1, .2])
a2 = numpy.dot(z1, w2) + b2
z2 = sigmoid(a2)

w3 = numpy.array([[.1, .3], [.2, .4]])
b3 = numpy.array([.1, .2])
a3 = numpy.dot(z2, w3) + b3
#y=a3

# d: softmax function

a = numpy.array([.3, 2.9, 4])
exp_a = numpy.exp(a)
sum_exp_a = numpy.sum(exp_a)
y = exp_a / sum_exp_a
print(y)


def softmax(a):
	"""this is common implementation of `softmax`"""
	c = numpy.max(a)
	exp_a = numpy.exp(a - c)  # to avoid overflow
	sum_exp_a = numpy.sum(exp_a)
	y = exp_a / sum_exp_a
	return y


# d: MNIST

(x_train, t_train), (x_test, t_test) = load_mnist(flatten=True,
                                                  normalize=False)


def img_show(img):
	pil_img = Image.fromarray(numpy.uint8(img))
	pil_img.show()


img = x_train[0]
label = t_train[0]
assert label == 5
img = img.reshape(28, 28)


def get_data():
	(x_train, t_train), (x_test, t_test) = load_mnist(flatten=True,
	                                                  normalize=True,
	                                                  one_hot_label=False)
	return x_test, t_test


def init_network():
	with open('templates/ch03/sample_weight.pkl', 'rb') as f:
		network = pickle.load(f)
	return network


def predict(network, x):
	w1, w2, w3 = network['W1'], network['W2'], network['W3'],
	b1, b2, b3 = network['b1'], network['b2'], network['b3'],
	a1 = numpy.dot(x, w1) + b1
	z1 = sigmoid(a1)
	a2 = numpy.dot(z1, w2) + b2
	z2 = sigmoid(a2)
	a3 = numpy.dot(z2, w3) + b3
	y = softmax(a3)
	return y


x, t = get_data()
network = init_network()
accuracy_cnt = 0
for i in range(len(x)):
	y = predict(network, x[i])
	p = numpy.argmax(y)
	if p == t[i]:
		accuracy_cnt += 1

print('Accuracy|> ', str(float(accuracy_cnt) / len(x)))
