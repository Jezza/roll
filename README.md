### [WIP] Roll

A small command line util to roll dice.  
Can also be used as a rust library.

```text
$ roll 1d3
1d3: [3] = 3
Result: 3

$ roll (1d8)d3
1d8: [8] = 8
8d3: [2, 3, 2, 1, 3, 1, 2, 3] = 17
Result: 17

$ roll (1d8)d3 + 5 + 12d(12d12d12)
1d8: [6] = 6
6d3: [3, 3, 3, 1, 3, 2] = 15
12d12: [6, 2, 7, 5, 5, 8, 11, 2, 8, 5, 1, 5] = 65
12d65: [6, 26, 10, 24, 44, 57, 59, 45, 15, 47, 41, 48] = 422
12d422: [364, 330, 209, 268, 315, 330, 219, 82, 392, 265, 165, 240] = 3179
Result: 3199
```
