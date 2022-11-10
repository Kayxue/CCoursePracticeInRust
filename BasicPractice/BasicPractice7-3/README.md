# 第七次作業第三題
# 題目
Given that  
* an = an-1 + 2bn-1 + 1,
* bn = 3an-1 − bn-1 + 1,

where a1 = 1 and b1 = 1,
write recursion functions to print out (an, bn) pairs where both an and bn are not larger than a number given by the user.
# 範例
```
How many sets of test data: 2

What is the upper limit? 228
n = 1, (a, b) = (1, 1)
n = 2, (a, b) = (4, 3)
n = 3, (a, b) = (11, 10)
n = 4, (a, b) = (32, 24)
n = 5, (a, b) = (81, 73)
n = 6, (a, b) = (228, 171)

What is the upper limit? 1000
n = 1, (a, b) = (1, 1)
n = 2, (a, b) = (4, 3)
n = 3, (a, b) = (11, 10)
n = 4, (a, b) = (32, 24)
n = 5, (a, b) = (81, 73)
n = 6, (a, b) = (228, 171)
n = 7, (a, b) = (571, 514)
```