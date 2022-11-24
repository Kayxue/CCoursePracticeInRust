# 第九次作業第一題
## 題目
Declare a character array `sent[300]` and get a string (containing white space) from keyborad.  
  
Use the following statements to do these problems.  

```c++
#include <stdio.h>
#include <string.h>
int main() {
    int repeatTimes, ti;
    
    // ask the user to input repeat times
    scanf("%d", &repeatTimes); if (getchar() == '\r') getchar();
    
    char sent[300];
    for (ti = 0; ti < repeatTimes; ti++) {
        // print an empty line
		
        // get one line from keyboard
        fgets(sent, sizeof(sent), stdin); strtok(sent, "\r\n");
		
        // Change all the target characters in the string into underline symbols here
        ...
    }
    return 0;
}
```

Change **all** the following characters in the string into underline symbols ('_') **except**  
  
*number digits (0 ~ 9)  
*dot ('.')  
*percent ('%')  

Print out the modified string after changing.
(You can write this program by either array manipulation or pointers.)

## 範例
```
How many sets of test data: 2

Input a line:
The Dow Jones Industrial Average ended 32.62 higher, or 0.28%, to 11555.63;
Modified line:
_______________________________________32.62____________0.28%_____11555.63_

Input a line:
and the Nasdaq finished 11.83 lower, or 0.47%, to 2515.51.
Modified line:
________________________11.83___________0.47%_____2515.51.
```