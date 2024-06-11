#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int main() {
	int N = 10000000, m = 0;
	srand((unsigned int)time(NULL));
	for (int i = 0; i < N; i++) {
		double x = (double)rand() / (double)RAND_MAX,
				 y = (double)rand() / (double)RAND_MAX;
		if (x * x + y * y < 1.0) {
			m = m + 1;
		}
	}

	printf("%f", (double)m * 4.0 / (double)N);
}
