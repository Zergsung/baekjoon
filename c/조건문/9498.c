int main() {
	int a = 0;
		
	scanf("%d", &a);
		
	if (90 <= a && 100>=a) {
		printf("A");
	}

	else if (80 <= a && a<= 89) {
		printf("B");
	}

	else if (70 <= a && a<= 79) {
		printf("C");
	}

	else if (60 <= a && a<= 69) {
		printf("D");
	}
	else {
		printf("F");
	}

}