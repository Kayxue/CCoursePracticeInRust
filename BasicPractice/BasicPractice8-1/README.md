# 第八次作業第一題
# 題目
Write a function `sort3(...)` which receives 3 integers
and save the smallest value in the first parameter,
save the middle one in the second parameter,
and the largest one in the third parameter.

Use the following `main()` function to get three integers from the keyboard and print out the sorting results for several times.
You **CANNOT** modify this `main()` function except the ??? part.
```cpp
int main() {
    int x, y, z, ti, repeatTimes;
    ??? // get repeatTimes from keyboard
    for (ti = 0; ti < repeatTimes; ti++) {
        printf("\nPlease input three numbers: ");
        scanf("%d%d%d", &x, &y, &z);
        sort3(???);
        printf("Results after sorting: %d, %d, %d\n", x, y, z);
    }
    return 0;
}
```
# 範例
```
How many sets of test data: 4

Please input three numbers: 25 19 34
Results after sorting: 19, 25, 34

Please input three numbers: 25 25 19
Results after sorting: 19, 25, 25

Please input three numbers: 19 34 25
Results after sorting: 19, 25, 34

Please input three numbers: 25 34 19
Results after sorting: 19, 25, 34
```