# d: several loss functions
class Momentum:

	def __init__(self, lr=.01, momentum=.9):
		self.lr = lr
		self.momentum = momentum
		self.v = None

	def update(self, params, grads):
		if self.v is None:
			self.v = {}
			for key, val in params.items():
				self.v[key] = numpy.zeros_like(val)
		for key in params.keys():
			self.v[key] = self.momentum * self.v[key] - self.lr * grads[key]
			params[key] += self.v[key]


class AdaGrad:

	def __init__(self, lr=.01):
		self.lr = lr
		self.h = None

	def update(self, params, grads):
		if self.h is None:
			self.h = {}
			for key, val in params.items():
				self.h[key] = numpy.zeros_like(val)
		for key in params.keys():
			self.h[key] += grads[key] * grads[key]
			params[key] -= self.lr * grads[key] / (numpy.sqrt(self.h[key]) +
			                                       1e-7)


import numpy as np
import matplotlib.pyplot as plt


def sigmoid(x):
	"""here is sigmoid"""
	return 1 / (1 + np.exp(-x))


#x = np.random.randn(1000, 100)
#node_num = 100
#hidden_layer_size = 5
#activations = {}
#for i in range(hidden_layer_size):
#	if i != 0:
#		x = activations[i - 1]
#	w = np.random.randn(node_num, node_num) / np.sqrt(node_num)
#	a = np.dot(x, w)
#	z = sigmoid(a)
#	activations[i] = z
#
#for i, a in activations.items():
#	plt.subplot(1, len(activations), i + 1)
#	plt.title(str(i + 1) + '-layer')
#	plt.hist(a.flatten(), 30, range=(0, 1))
#plt.show()

from templates.dataset.mnist import load_mnist
from templates.common.util import smooth_curve
from templates.common.multi_layer_net import MultiLayerNet
from templates.common.optimizer import SGD

## 0: get data
#(x_train, t_train), (x_test, t_test) = load_mnist(normalize=True)
#train_size = x_train.shape[0]
#batch_size = 128
#max_iterations = 2000
#
## 1: initial config
#weight_init_types = {'std=0.01': .01, 'Xavier': 'sigmoid', 'He': 'relu'}
#optimizer = SGD(lr=.01)
#networks = {}
#train_loss = {}
#for key, weight_type in weight_init_types.items():
#	networks[key] = MultiLayerNet(input_size=784,
#	                              hidden_size_list=[100, 100, 100, 100],
#	                              output_size=10,
#	                              weight_init_std=weight_type)
#	train_loss[key] = []
#
## 2: start learning
#for i in range(max_iterations):
#	batch_mask = np.random.choice(train_size, batch_size)
#	x_batch = x_train[batch_mask]
#	t_batch = t_train[batch_mask]
#
#	for key in weight_init_types.keys():
#		grads = networks[key].gradient(x_batch, t_batch)
#		optimizer.update(networks[key].params, grads)
#		loss = networks[key].loss(x_batch, t_batch)
#		train_loss[key].append(loss)
#	if i % 100 == 0:
#		print('iter|> ' + str(i))
#		for key in weight_init_types.keys():
#			loss = networks[key].loss(x_batch, t_batch)
#			print(key + ':' + str(loss))
#
## 3: draw graph
#markers = {'std=0.01': 'o', 'Xavier': 's', 'He': 'D'}
#x = np.arange(max_iterations)
#for key in weight_init_types.keys():
#	plt.plot(x,
#	         smooth_curve(train_loss[key]),
#	         marker=markers[key],
#	         markevery=100,
#	         label=key)
#plt.xlabel('iterations')
#plt.ylabel('loss')
#plt.ylim(0, 2.5)
#plt.legend()
#plt.show()


class Dropout:

	def __init__(self, dropout_ratio=.5):
		self.dropout_ratio = dropout_ratio
		self.mask = None

	def forward(self, x, train_flg=True):
		if train_flg:
			self.mask = np.random.rand(*x.shape) > self.dropout_ratio
			return x * self.mask
		else:
			return x * (1.0 - self.dropout_ratio)

	def backward(self, dout):
		return dout * self.mask


from templates.common.multi_layer_net_extend import MultiLayerNetExtend
from templates.common.trainer import Trainer

(x_train, t_train), (x_test, t_test) = load_mnist(normalize=True)

# cut numbers of data to reproduce overfit
x_train = x_train[:300]
t_train = t_train[:300]

use_dropout = True
dropout_ratio = .2

network = MultiLayerNetExtend(input_size=784,
                              hidden_size_list=[
                                  100,
                                  100,
                                  100,
                                  100,
                                  100,
                                  100,
                              ],
                              output_size=10,
                              use_dropout=use_dropout,
                              dropout_ration=dropout_ratio)
trainer = Trainer(network,
                  x_train,
                  t_train,
                  x_test,
                  t_test,
                  epochs=301,
                  mini_batch_size=100,
                  optimizer='sgd',
                  optimizer_param={'lr': .01},
                  verbose=True)
trainer.train()

trainer.test_acc_list, test_acc_list = trainer.test_acc_list, trainer, test_acc_list

# show graph
markers = {'train': 'o', 'test': 's'}
x = np.arange(len(train_acc_list))
plt.plot(x, train_acc_list, markers='o', label='train', markevery=10)
plt.plot(x, test_acc_list, markers='s', label='test', markevery=10)
plt.xlabel('epochs')
plt.ylabel('accuracy')
plt.ylim(0, 1)
plt.legend(loc='lower right')
plt.show()
