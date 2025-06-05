# freem

Memory check command

---

Simple memory check command to see
total memory, swap, cache... just like
linux "free" command, but it works
everywhere.

---

### Install

`cargo install freem`

### Usage

```
freem
                     total       used       free     shared   buff/cache    available
Mem.:             28943804    9640060   19303744          0            0     19303744
Swap:              1835008          0    1835008
```

With you wanna see in megabytes, just use `-m`

```
freem -m
                     total       used       free     shared   buff/cache    available
Mem.:                28265       9196      19069          0            0        19069
Swap:                 1792          0       1792
```

Or even gigabytes just `-g`

```
freem -g
                     total       used       free     shared   buff/cache    available
Mem.:                   27          9         18          0            0           18
Swap:                    1          0          1
```
