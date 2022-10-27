#include <stdio.h>

int main() {
    int num1 = 0, num2 = 0, num3 = 0, first = 0, second = 0, third = 0;

    scanf("%d %d", &num1, &num2);

    num3 = num2;

    first = num3 % 10;
    num3 = num3 - first;

    second = num3 % 100;
    num3 = num3 - second;
    second = second / 10;

    third = num3 / 100;

    printf("%d\n%d\n%d\n%d", num1 * first, num1 * second, num1 * third, num1 * num2);
}