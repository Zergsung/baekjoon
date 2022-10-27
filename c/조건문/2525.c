int main() {
	int t = 0, m = 0, it=0, num=0, num1=0, num2=0;

	scanf("%d %d %d", &t, &m, &it);
	
	num = m + it;
	
	if (num < 60) {
		printf("%d %d", t, num);
	}
	
	else if (num < 120) {

		num = num % 60;
		t = t + 1;

			if (t < 24) {
				printf("%d %d", t, num);
			}
			else {
				t = 24 - t;
				printf("%d %d", t, num);
			}
	}
	else {
		num2 = num % 60;
		num1 = num / 60;
		t = t + num1;

		if (t < 24) {
			printf("%d %d", t, num2);
		}

		else {
			t = t - 24;
			
			printf("%d %d", t, num2);
		}
		
	}
}