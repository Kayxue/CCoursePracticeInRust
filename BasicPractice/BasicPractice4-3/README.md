# 第四次作業第三題
## 題目
Write a function power(x, y) to calculate the value of xy,
where x is a real number and y is a non-negative integer.

Use the following main() to test your function.
```cpp
double myPower(double x, int y) {
    // calculate xy here
}

int main() {
    int ti, repeatTimes, year, month, day;
    // get repeatTimes here
    for (ti = 0; ti < repeatTimes; ti++) {
        printf("\nx = "); scanf("%lf", &x);
        printf("n = "); scanf("%d", &n);
        printf("power(%f, %d) = %f\n", x, n, myPower(x, n));
    }
    return 0;
}
```
## 範例
```
How many sets of test data: 4

x = 3.546
n = 2
power(3.546000, 2) = 12.574116

x = 3.546
n = 0
power(3.546000, 0) = 1.000000

x = -4.1
n = 3
power(-4.100000, 3) = -68.921000

x = -4.1
n = 4
power(-4.100000, 4) = 282.576100
```
## 備註
範例中的 `power(3.456000, 2)` 計算結果可能與此程式執行此運算時結果不同，此為正常現象，因為 C++ 的 double 精度不夠。