int main() {

	int it=0, im=0;

	scanf("%d %d", &it, &im);

	im = im - 45;
	
	if (im < 0) {
		if (it!=0) {
			it = it - 1;
		}
		else {
			it = 23;
		}
		im = 60 + im;
	}
	
	printf("%d %d", it, im);
	

}