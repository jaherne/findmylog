### Find My Log

Find My Log will print the contents of a file or stdin while highlighting any lines that match the given pattern. This makes it easier to scan for a particular message in a log file without losing the context of the other messages.


### Sample Usage

```
$ cat fruit.txt

apples
bananas
kiwis
oranges

$ cat fruit.txt | fml app

**apples**
bananas
kiwis
oranges
```

### Syntax

```
fml <pattern> [file]

Arguments:
  <pattern>  The search pattern
  [file]     The input file (optional)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Todo
- [ ] Add additional color schemes and highlighting options
- [ ] Choose between highlighting the entire line and just the match
- [ ] Better pattern matching (case sensitivitiy, regex)
- [ ] Add wide character support