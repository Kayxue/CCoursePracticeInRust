# 第八次作業第二題
# 題目
Write a function meanValue(...) with 5 parameters
which receives two real numbers a and b,
and evaluate their means in three kinds: arithmetic mean (算術平均數), geometric mean (幾何平均數), and harmonic mean (調和平均數).
The three means are saved in variables given by call-by-reference.

Use the following main() function to get two real numbers from the keyboard and print out their means for several times.
You CANNOT modify this main() function except the ??? part.

```cpp
int main() {
    double a, b, aMean, gMean, hMean;
    int ti, repeatTimes;
    ?? // get repeatTimes from keyboard
    for (ti = 0; ti < repeatTimes; ti++) {
        printf("\nPlease input two numbers: ");
        scanf("%lf%lf", &a, &b);
        meanValue(???);
        printf("The arithmetic mean of %f and %f is %f\n", a, b, aMean);
        printf("The geometric mean of %f and %f is %f\n", a, b, gMean);
        printf("The harmonic mean of %f and %f is %f\n", a, b, hMean);
    }
    return 0;
}
```
# 範例
```
How many sets of test data: 5

Please enter the price: 27
You need 5 coin(s), including:
    $50: 0
    $10: 2
    $5: 1
    $1: 2

Please enter the price: 1
You need 1 coin(s), including:
    $50: 0
    $10: 0
    $5: 0
    $1: 1

Please enter the price: 500
You need 10 coin(s), including:
    $50: 10
    $10: 0
    $5: 0
    $1: 0

Please enter the price: 53
You need 4 coin(s), including:
    $50: 1
    $10: 0
    $5: 0
    $1: 3

Please enter the price: 169
You need 9 coin(s), including:
    $50: 3
    $10: 1
    $5: 1
    $1: 4
```