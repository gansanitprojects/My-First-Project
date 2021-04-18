#include <stdlib.h>
#include <stdio.h>
#include <memory.h>

#define f64 double

int main() {
    size_t limit = 200000;
    f64 arr1[limit];
    f64 arr2[limit];

    for(size_t i = 0; i < limit; i++) {
        arr1[i] = (f64)(i + 1);
        arr2[i] = (f64)(i + 1);
    }

    f64 dotP = 0.0;
    for(size_t i = 0; i < limit; i++) {
        dotP += ((f64) arr1[i]) * ((f64) arr2[i]);
    }

    printf("%lf\n", dotP);
    return 0;

}
