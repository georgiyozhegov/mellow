This document describes syntax of the Mellow programming language.

---

## Let

### Immutable

Assigns *identifier* to *expression*.

By default, `let` statement declares an immutable variable.

```
let <identifier> = <expression>
```

### Mutable

To declare a mutable variable add `mutable` keyword after `let`.

```
let mutable <identifier> = <expression>
```

---

## Change

Changes the value of *identifier*.

```
do <identifier> = <expression>
```

This will work only if variable is defined as a mutable.

---

## If

### Statement

Executes *then* code block if *condition* is true, otherwise executes *else* block.

```
if <condition> do
    [<statement>]
end
```

```
if <condition> do
    [<statement>]
else
    [<statement>]
end
```

**WARNING**: Not implemented yet.

```
if <condition> then
    [<statement>]
elif <condition> then
    [<statement>]
else
    [<statement>]
end
```

### Expression

Returns an expression.

```
if <condition> then
    <expression>
else
    <expression>
end
```

**WARNING**: Not implemented yet.

```
if <condition> then
    <expression>
elif <condition> then
    <expression>
else
    <expression>
end
```

---

## While

Executes block of code while condition is true.

```
while <condition> do
    [<statement>]
end
```

## Loop

**WARNING**: Not implemented yet.

Executes block of code until `break`.

```
loop
    [<statement>]
end
```

---

## For

### For-In

For each *item* in a *sequence* executes `do` block.

```
for <item> in <sequence> do
    [<statement>]
end
```

### For-From-To

For each *item* in a range from *from* to *to* executes `do` block.

```
for <item> from <from> to <to> do
    [<statement>]
end
```

**WARNING**: Not implemented yet.

```
for <item> from <from> to <to> step <step> do
    [<statement>]
end
```
