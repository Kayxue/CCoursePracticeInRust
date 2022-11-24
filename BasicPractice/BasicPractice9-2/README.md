# 第九次作業第二題
## 題目
Write a program to print substrings of a given sentence.
Each time you get a line `sent` and a number `k` from the user,
print out all the substrings of the line by skipping `k` characters at the beginning of the line `sent`.
## 範例
```
How many sets of test data: 3

Input a string: The Dow Jones Industrial Average ended 32.62 higher, or 0.28%, to 11555.63;
Skipping = 4
[The Dow Jones Industrial Average ended 32.62 higher, or 0.28%, to 11555.63;]
[Dow Jones Industrial Average ended 32.62 higher, or 0.28%, to 11555.63;]
[Jones Industrial Average ended 32.62 higher, or 0.28%, to 11555.63;]
[s Industrial Average ended 32.62 higher, or 0.28%, to 11555.63;]
[dustrial Average ended 32.62 higher, or 0.28%, to 11555.63;]
[rial Average ended 32.62 higher, or 0.28%, to 11555.63;]
[ Average ended 32.62 higher, or 0.28%, to 11555.63;]
[rage ended 32.62 higher, or 0.28%, to 11555.63;]
[ ended 32.62 higher, or 0.28%, to 11555.63;]
[ed 32.62 higher, or 0.28%, to 11555.63;]
[2.62 higher, or 0.28%, to 11555.63;]
[ higher, or 0.28%, to 11555.63;]
[her, or 0.28%, to 11555.63;]
[ or 0.28%, to 11555.63;]
[0.28%, to 11555.63;]
[%, to 11555.63;]
[o 11555.63;]
[555.63;]
[63;]

Input a string: and the Nasdaq finished 11.83 lower, or 0.47%, to 2515.51
Skipping = 7
[and the Nasdaq finished 11.83 lower, or 0.47%, to 2515.51]
[ Nasdaq finished 11.83 lower, or 0.47%, to 2515.51]
[ finished 11.83 lower, or 0.47%, to 2515.51]
[ed 11.83 lower, or 0.47%, to 2515.51]
[3 lower, or 0.47%, to 2515.51]
[, or 0.47%, to 2515.51]
[47%, to 2515.51]
[ 2515.51]
[1]

Input a string: National Taiwan Ocean University was originally established in 1953
Skipping = 5
[National Taiwan Ocean University was originally established in 1953]
[nal Taiwan Ocean University was originally established in 1953]
[aiwan Ocean University was originally established in 1953]
[ Ocean University was originally established in 1953]
[n University was originally established in 1953]
[versity was originally established in 1953]
[ty was originally established in 1953]
[s originally established in 1953]
[ginally established in 1953]
[ly established in 1953]
[tablished in 1953]
[shed in 1953]
[in 1953]
[53]
```