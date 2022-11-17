# 第八次作業第三題
# 題目
Write a program to dispense change. The user enters the amount paid and the amount due.  
The program determines how many $50, $10, $5, and $1 coins should be given as change.  
(Adapted from Programming Project 2.7 in King's book, and Programming Project 7.3 in Hanly and Koffman's Problem Solving and Program Design in C.)  

This program should include the following function:  

```cpp
void pay_amount(int dollars, int *fifties, int *tens, int *fives, int *ones);
```
The approach of change dispensing is to find the most coins with the highest face value.
In other words,  

* Step 1: Find the most $50 coins under the whole price
* Step 2: Find the most $10 coins under the rest of the price
* Step 3: Find the most $5 coins under the rest of the price
* Step 4: Find the number of the $1 coins under the rest of the price

Use the following `main()` function to get an integer from the keyboard and print out the way to dispense change for several times.
You CANNOT modify this `main()` function except the ??? parts.

```cpp
int main() {
    int price, num1, num5, num10, num50, ti, repeatTimes;
    ??? // get repeatTimes from keyboard
    for (ti = 0; ti < repeatTimes; ti++) {
        printf("\nPlease enter the price: ");
        scanf("%d", &price);
        pay_amount(???);
        printf("You need %d coin(s), including:\n", ???);
        printf("\t$50: %d\n", num50);
        printf("\t$10: %d\n", num10);
        ...???
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