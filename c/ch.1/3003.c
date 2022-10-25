#include <stdio.h>

int main() {
    int a, b, c, d, e, f;
    scanf("%d %d %d %d %d %d", &a, &b, &c, &d, &e, &f);
    a = 0 - (a - 1);
    b = 0 - (b - 1);
    c = 0 - (c - 2);
    d = 0 - (d - 2);
    e = 0 - (e - 2);
    f = 0 - (f - 8);
    printf("%d %d %d %d %d %d", a, b, c, d, e, f);
}