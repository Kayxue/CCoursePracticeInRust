# 第五次作業第一題

## 題目
Write a function to check if a given date is an invalid date (不合法日期), whose function prototype should be:  

* `int isInvalidDate(int year, int month, int day); // in C `or  
* `bool isInvalidDate(int year, int month, int day); // in C++`  

Given a date (year/month/day), this function will return  
* `true` or 1 if it is NOT a valid date;  
* `false` or 0 otherwise.  

When check a date in Feburary,
you also need to know if the given year is a leap year (閏年),
so that you can know that there are 28 or 29 days in Feburary.
Note that the rules of leap years are:  

* 逢四則閏 (4's multiples are leap years)
* 逢百不閏 (but 100's multiples are not leap years)
* 逢四百又閏 (but 400's multiples are leap years)  

Write a function to decide if a given year is a leap, whose function prototype should be:  

* `int isLeapYear(int year); // in C` or  
* `bool isLeapYear(int year); // in C++`

Given a year, this function will return  
* `true` or 1 if it is a leap year;
* `false` or 0 otherwise.  

Use the following main() to test your function.
```cpp
int main() {
    int ti, repeatTimes, year, month, day;
    // get repeatTimes here
    for (ti = 0; ti < repeatTimes; ti++) {
        printf("\nInput a date (year/month/day): ");
        scanf("%d/%d/%d", &year, &month, &day);
        printf("%d/%d/%d %s a valid date.\n", year, month, day, isInvalidDate(year, month, day) ? "is not" : "is");
    }
    return 0;
}
```
## 範例
```
How many sets of test data: 5

Input a date (year/month/day): 2015/-7/23
2015/-7/23 is not a valid date.

Input a date (year/month/day): 2015/5/0
2015/5/0 is not a valid date.

Input a date (year/month/day): 2016/2/29
2016/2/29 is a valid date.

Input a date (year/month/day): 2015/2/29
2015/2/29 is not a valid date.

Input a date (year/month/day): 777/3/6
777/3/6 is a valid date.
```