# nandlang

NAND circuit programming language.

## Grammer

**gate** refers to a basic logic gate, such as a NAND gate, while **component** refers to a user-defined custom gate or
a more complex circuit element.

### Define input port

```
IN a BIT 1 0 1 0
IN b BIT
```

`IN`: Reserved keyword to define an input port.

`a`: User-defined variable name of this input component.

`BIT`: Reserved keyword to specify the type of input signals.

Rest of `1 0 ...`: User-defined signal sequence, which is sent at each tick. (Optional)

### Define output port

```
OUT x BIT
```

`OUT`: Reserved keyword to define an output port.

`x`: User-defined variable name for this output component.

`BIT`: Reserved keyword to specify the type of output signals.

### Define variable

In nandlang, a primitive NAND gate or a user-defined custom gate/component is treated as a "Class" in typical
programming languages.

Variables are instances of these gates or components.

```
VAR a NAND
VAR b MY_GATE
```

`VAR`: Reserved keyword to define a variable.

`a`: User-defined variable name.

`NAND`: The name of the gate/component to assign as the type of the variable.

### Wiring

```
FROM a out TO b in
FROM b o1 TO c i1
FROM b o2 TO d in
```

`FROM`: Reserved keyword to define the source of a wire.

`a`: Name of the source gate/component.

`out`: Output port name of the source gate/component.

`TO`: Reserved keyword to define the destination of a wire.

`b`: Name of the destination gate/component.

`in`: Input port name of the destination gate/component.

### Define Gate/Component

A custom gate or component can be defined as follows:

Indentation is used for visual clarity only.

```
GATE START NOT
    IN in BIT
    OUT out BIT
    VAR nand NAND
    FROM in in1 TO nand i1
    FROM in in1 TO nand i2
    FROM nand o1 TO out out
GATE END
```
