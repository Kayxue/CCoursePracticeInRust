# 第十次作業第一題
## 題目
Write a function

```c++
void duplicateStr(char dest[], char *src, int n);
```

which duplicates the source string for n times and save it in the destination array.  
**NOTE** that `dest[]` becomes empty string (i.e. `dest[0]` is `'\0'`) if **n** ≤ 0.  
Declare two character arrays with 1000 elements in your `main()` function.  
Ask the user to input a string and a number.  
Call `duplicateStr()` to create the new string. Print it out.

Use the following `main()` function to test your function.  
You CANNOT modify this `main()` function except the ??? part.

```c++
void duplicateStr(char dest[], char *src, int n) { ??? }

int main() {
    char newStr[???], inputStr[???];
    int ti, repeatTimes, rpTimes;
    ??? // get repeatTimes from keyboard
    for (ti = 0; ti < repeatTimes; ti++) {
        printf("\nInput a string: ");
        scanf(???); // or fgets(???)
        printf("Duplicate how many times: ");
        scanf(???);
        duplicateStr(newStr, inputStr, rpTimes);
        printf("The new string is [%s]\n", ???);
    }
    return 0;
}
```

Programs not using `duplicateStr()` will receive 0 points.
## 範例
```
How many sets of test data: 5

Input a string: Koka
Duplicate how many times: 5
The new string is [KokaKokaKokaKokaKoka]

Input a string: yesterday
Duplicate how many times: -2
The new string is []

Input a string: \(^_^)/
Duplicate how many times: 10
The new string is [\(^_^)/\(^_^)/\(^_^)/\(^_^)/\(^_^)/\(^_^)/\(^_^)/\(^_^)/\(^_^)/\(^_^)/]

Input a string: S.H.E.
Duplicate how many times: 8
The new string is [S.H.E.S.H.E.S.H.E.S.H.E.S.H.E.S.H.E.S.H.E.S.H.E.]

Input a string: see\
Duplicate how many times: 15
The new string is [see\see\see\see\see\see\see\see\see\see\see\see\see\see\see\]
```
