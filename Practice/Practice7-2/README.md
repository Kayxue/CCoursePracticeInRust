# 第七次第二題
## 題目
Write a program to simulate Big Lottery (大樂透).  
  
**[Definition]**  
The first-prize numbers are 6 distinct numbers in [1, 49].
The special number is also in [1, 49] but not contained in the first-prize numbers.  
A Big Lottery ticket has 6 distinct numbers in [1, 49] and costs you NT$50.  
  
**[Prize Checking]**  
Please write a function which, given a lottery ticket and the first-prize numbers, checks if the buyer wins any prize
and returns the index of the winning prize (0 for no winning):  
```cpp
int checkPrize(int ticket[], int firstPrize[], int special);
```
The prizes are defined as follows:
|Index|項目  Prize Item|獎金  Prize Amount|對中個數  Matched First-Prize|對中特別號  Matched Special範例  Examples|