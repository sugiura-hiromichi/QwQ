# d: loss function
import numpy


def mean_squared_error(y, t):
	return 0.5 * numpy.sum((y - t)**2)


t = numpy.array([0, 0, 1, 0, 0, 0, 0, 0, 0, 0])
y = numpy.array([.1, .05, .6, 0, .05, .1, 0, .1, 0, 0])
y_wrong = numpy.array([1, 0, 0, 0, 0, 0, 0, 0, 0, 0])
assert mean_squared_error(y, t) == 0.09750000000000003
assert mean_squared_error(y_wrong, t) >= 0.09750000000000003


def cross_entropy_error(y, t):
	delta = 1e-7
	return -numpy.sum(t * numpy.log(y + delta))


assert cross_entropy_error(y, t) == 0.510825457099338
assert cross_entropy_error(y_wrong, t) >= 0.510825457099338

# d: mini batch learning
from templates.dataset.mnist import load_mnist
(x_train, t_train), (x_test, t_test) = load_mnist(normalize=True,
                                                  one_hot_label=True)
train_size = x_train.shape[0]
batch_size = 10
batch_mask = numpy.random.choice(train_size, batch_size)
x_batch = x_train[batch_mask]
t_batch = t_train[batch_mask]


def cross_entropy_error_batch(y, t):
	if y.ndim == 1:
		t = t.reshape(1, t.size)
		y = y.reshape(1, y.size)
	batch_size = y.shape[0]
	return -numpy.sum(t * numpy.log(y)) / batch_size


# d: diff
def numerical_diff(f, x):
	epsilon = 1e-4
	return (f(x + epsilon) - f(x - epsilon)) / 2 * epsilon


def _numerical_gradient_no_batch(f, x):
	h = 1e-4  # 0.0001
	grad = numpy.zeros_like(x)

	for idx in range(x.size):
		tmp_val = x[idx]
		x[idx] = float(tmp_val) + h
		fxh1 = f(x)  # f(x+h)

		x[idx] = tmp_val - h
		fxh2 = f(x)  # f(x-h)
		grad[idx] = (fxh1 - fxh2) / (2 * h)

		x[idx] = tmp_val  # 値を元に戻す

	return grad


def numerical_gradient(f, X):
	if X.ndim == 1:
		return _numerical_gradient_no_batch(f, X)
	else:
		grad = numpy.zeros_like(X)

		for idx, x in enumerate(X):
			grad[idx] = _numerical_gradient_no_batch(f, x)

		return grad


def gradient_descent(f, init_x, learn_rate=0.01, step_n=100):
	x = init_x
	for i in range(step_n):
		grad = numerical_gradient(f, x)
		x -= learn_rate * grad
	return x


def fun_2(x):
	return x[0]**2 + x[1]**2


def softmax(a):
	"""this is common implementation of `softmax`"""
	c = numpy.max(a)
	exp_a = numpy.exp(a - c)  # to avoid overflow
	sum_exp_a = numpy.sum(exp_a)
	y = exp_a / sum_exp_a
	return y


def sigmoid(x):
	"""here is sigmoid"""
	return 1 / (1 + numpy.exp(-x))


class SimpleNet:

	def __init__(self):
		self.W = numpy.random.randn(2, 3)

	def predict(self, x):
		return numpy.dot(x, self.W)

	def loss(self, x, t):
		z = self.predict(x)
		y = softmax(z)
		loss = cross_entropy_error_batch(y, t)
		return loss


net = SimpleNet()
x = numpy.array([.6, .9])
p = net.predict(x)
t = numpy.array([0, 0, 1])


def f(_):
	return net.loss(x, t)


g = f

dW = numerical_gradient(g, net.W)


class TwoLayerNet:

	def __init__(self,
	             input_size,
	             hidden_size,
	             output_size,
	             weight_init_std=0.01):
		self.params = {}
		self.params['W1'] = weight_init_std * numpy.random.randn(
		    input_size, hidden_size)
		self.params['b1'] = numpy.zeros(hidden_size)
		self.params['W2'] = weight_init_std * numpy.random.randn(
		    hidden_size, output_size)
		self.params['b2'] = numpy.zeros(output_size)

	def predict(self, x):
		W1, W2 = self.params['W1'], self.params['W2']
		b1, b2 = self.params['b1'], self.params['b2']

		a1 = numpy.dot(x, W1) + b1
		z1 = sigmoid(a1)
		a2 = numpy.dot(z1, W2) + b2
		y = softmax(a2)

		return y

	def loss(self, x, t):
		y = self.predict(x)
		return cross_entropy_error_batch(y, t)

	def accuracy(self, x, t):
		y = self.predict(x)
		y = numpy.argmax(y, axis=1)
		t = numpy.argmax(t, axis=1)

		accuracy = numpy.sum(y == t) / float(x.shape[0])
		return accuracy

	def numerical_gradient(self, x, t):
		loss_W = lambda W: self.loss(x, t)

		grads = {}
		grads['W1'] = numerical_gradient(loss_W, self.params['W1'])
		grads['b1'] = numerical_gradient(loss_W, self.params['b1'])
		grads['W2'] = numerical_gradient(loss_W, self.params['W2'])
		grads['b2'] = numerical_gradient(loss_W, self.params['b2'])

		return grads


from templates.dataset.mnist import load_mnist
(x_train, t_train), (x_test, t_test) = load_mnist(normalize=True,
                                                  one_hot_label=True)
#train_size = x_train.shape[0]
train_loss_list = []
train_acc_list = []
test_acc_list = []
iter_per_epoch = max(train_size / batch_size, 1)

# meta
iters_num = 10_000
batch_size = 100
learning_rate = .1

network = TwoLayerNet(input_size=784, hidden_size=50, output_size=10)
for i in range(iters_num):
	#get mini_batch
	batch_mask = numpy.random.choice(train_size, batch_size)
	x_batch = x_train[batch_mask]
	t_batch = t_train[batch_mask]
	#calc gradient
	grad = network.numerical_gradient(x_batch, t_batch)
	#update params
	for key in ('W1', 'b1', 'W2', 'b2'):
		network.params[key] -= learning_rate * grad[key]
	#log learning
	train_loss_list.append(network.loss(x_batch, t_batch))
	#calc accuracy of each epoch
	if i % iter_per_epoch == 0:
		train_acc = network.accuracy(x_train, t_train)
		test_acc = network.accuracy(x_test, t_test)
		train_acc_list.append(train_acc)
		test_acc_list.append(test_acc)
		print('train_acc, test_acc |>' + str(train_acc) + ' ' + str(test_acc))

print('🫠🫠 🫠🫠')
