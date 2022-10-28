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

|Index|項目  Prize Item|獎金  Prize Amount|對中個數  Matched First-Prize|對中特別號  Matched Special|範例  Examples|  
|-----|-----|-----|-----|-----|-----|  
|8|頭獎|19,100,192|6|0|first: {7, 17, 34, 8, 10, 36} + 43 vs. ticket: {8, 7, 36, 10, 17, 34}|  
|7|貳獎|1,569,878|5|1|first: {7, 17, 34, 8, 10, 36} + 43 vs. ticket: {8, 43, 36, 10, 17, 34}|  
|6|參獎|32,705|5|0|first: {7, 17, 34, 8, 10, 36} + 43 vs. ticket: {8, 7, 36, 19, 17, 34}|  
|5|肆獎|9,781|4|1|first: {7, 17, 34, 8, 10, 36} + 43 vs. ticket: {43, 7, 36, 10, 6, 34}|  
|4|伍獎|1,286|4|0|first: {7, 17, 34, 8, 10, 36} + 43 vs. ticket: {25, 7, 36, 10, 17, 6}|  
|3|陸獎|1000|3|1|irst: {7, 17, 34, 8, 10, 36} + 43 vs. ticket: {18, 27, 36, 10, 17, 43}|  
|2|柒獎|400|2|1|first: {7, 17, 34, 8, 10, 36} + 43 vs. ticket: {28, 5, 10, 43, 17, 4}|  
|1|普獎|400|3|0|first: {7, 17, 34, 8, 10, 36} + 43 vs. ticket: {8, 7, 23, 1, 49, 17}|  
  
(Selected from the prize amounts of Big Lottery on 103/10/14, cf. http://www.taiwanlottery.com.tw/lotto/Lotto649/history.aspx)  
  
Use the following main() to test your function.  
```cpp
int main() {
    char *prizeName[9] = {"", "General", "Seventh", "Sixth", "Fifth", "Fourth", "Third", "Second", "First"};
    int prize[9] = {0, 400, 400, 1000, 1286, ...};
    int first[6], ticket[6], special, i;

    printf("Input the first-prize numbers: ");
    for (i = 0; i < 6; i++) scanf("%d", ???);
    printf("Input the special number: ");
    scanf("%d", ???);
    printf("Input the numbers on the lottery ticket: ");
    for (i = 0; i < 6; i++) scanf("%d", ???);

    int prizeCode = checkPrize(???);
    if (prizeCode > 0) printf("You have won the %s Prize which is NT$%d!!\n", prizeName[???], prize[???]);
    else printf("You did not win any prize.\n");
    return 0;
}
```
Call `shuffling(a, ...)` to get a random permutation of 0 ~ 48 in `a[]`.
Take the first 7 numbers of `a[]` as the Lottery First Prize Numbers. `// first[i] = a[i] + 1`
Take the first 6 numbers of `a[]` as a Lottery ticket.
Print out the numbers and the prize won by this ticket.
Do it for 5 times.
## 範例
```
Input the first-prize numbers: 7 17 34 8 10 36
Input the special number: 43
Input the numbers on the lottery ticket: 43 7 36 10 6 34
You have won the Fourth Prize which is NT$9781!!

[Random similation for 5 times]

The first-prize numbers are 8 12 45 46 41 7
The special number is 28
Your ticket numbers are 6 21 40 37 9 46
You did not win any prize.

The first-prize numbers are 27 8 17 37 32 47
The special number is 22
Your ticket numbers are 32 9 5 41 42 44
You did not win any prize.

The first-prize numbers are 32 22 42 36 44 43
The special number is 38
Your ticket numbers are 39 38 49 32 44 29
You have won the Seventh Prize which is NT$400!!

The first-prize numbers are 13 14 39 28 45 49
The special number is 9
Your ticket numbers are 16 13 34 11 33 27
You did not win any prize.

The first-prize numbers are 3 5 12 10 27 47
The special number is 6
Your ticket numbers are 47 20 49 21 5 10
You have won the General Prize which is NT$400!!
```
