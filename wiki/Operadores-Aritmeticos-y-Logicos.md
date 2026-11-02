# 🧮 Operadores Aritméticos, Relacionales y Lógicos

En esta sección se detalla el conjunto completo de operadores y funciones matemáticas incorporadas disponibles en **YAGUARETÉ**, su nivel de precedencia y las reglas de resolución sintáctica.

---

## ➕ Operadores Aritméticos

| Operador | Nombre | Descripción | Ejemplo | Resultado |
| :---: | :--- | :--- | :--- | :--- |
| **`+`** | Suma / Concatenación | Suma aritmética entre números o concatenación si interviene una `CADENA`. | `15 + 27`<br>`"Hola " + "Mundo"` | `42`<br>`"Hola Mundo"` |
| **`-`** | Resta / Negación | Resta entre números o negación unaria. | `100 - 35`<br>`-valor` | `65`<br>`-valor` |
| **`*`** | Multiplicación | Producto de dos operandos numéricos. | `6 * 7` | `42` |
| **`/`** | División Real | División aritmética que produce un número con decimales (`REAL` o `FLOTANTE`). | `10 / 4` | `2.5` |
| **`//`** | División Entera | División truncada al entero más cercano (cociente sin decimales). | `10 // 4`<br>`17 // 5` | `2`<br>`3` |
| **`%`** | Módulo | Resto o residuo de la división entera. | `17 % 5` | `2` |
| **`++`** | Incremento | Aumenta en 1 la variable objetivo. | `i++` | `i = i + 1` |
| **`--`** | Decremento | Disminuye en 1 la variable objetivo. | `n--` | `n = n - 1` |

### 💡 Disambiguación entre Comentario `//` y División Entera `//`

En YAGUARETÉ, `//` tiene doble significado según el contexto sintáctico:
1. Como **comentario de línea**: Si aparece al comienzo de una línea o precedido de espacios en blanco sin formar parte de una expresión activa.
2. Como **operador infijo de división entera**: Si aparece rodeado por expresiones o dentro de índices matriciales (por ejemplo: `matriz[p // columnas][p % columnas]`).

El analizador léxico de Rust identifica con precisión el contexto del token para evitar falsos positivos y permitir el acceso algebraico bidimensional sin requerir paréntesis adicionales.

---

## ⚖️ Operadores Relacionales

Todos los operadores relacionales devuelven un valor de tipo `BOOLEANO` (`VERDADERO` o `FALSO`):

| Operador | Significado | Ejemplo | Resultado |
| :---: | :--- | :--- | :--- |
| **`==`** | Igual a | `10 == 10`<br>`"a" == "b"` | `VERDADERO`<br>`FALSO` |
| **`!=`** | Distinto de | `10 != 5` | `VERDADERO` |
| **`<`** | Menor que | `3 < 8` | `VERDADERO` |
| **`>`** | Mayor que | `12 > 20` | `FALSO` |
| **`<=`** | Menor o igual que | `5 <= 5` | `VERDADERO` |
| **`>=`** | Mayor o igual que | `9 >= 10` | `FALSO` |

---

## 🔀 Operadores Lógicos

Los operadores lógicos admiten tanto las palabras reservadas pedagógicas en español (`Y`, `O`, `NO`) como la notación simbólica internacional (`&&`, `||`, `!`):

| Operador en Español | Notación Simbólica | Operación | Ejemplo |
| :---: | :---: | :--- | :--- |
| **`Y`** | `&&` | Conjunción lógica (AND). Es `VERDADERO` únicamente si ambos operandos son verdaderos. | `(edad >= 18) Y (tiene_registro == VERDADERO)` |
| **`O`** | `\|\|` | Disyunción lógica (OR). Es `VERDADERO` si al menos un operando es verdadero. | `(es_socio == VERDADERO) O (saldo > 0)` |
| **`NO`** | `!` | Negación lógica (NOT). Invierte el valor de verdad. | `NO (archivo.findearchivo)` |

> **Cortocircuito**: En expresiones `A Y B`, si `A` es `FALSO`, `B` no se evalúa. Igualmente, en `A O B`, si `A` es `VERDADERO`, `B` no se evalúa.

---

## 🔢 Funciones Matemáticas Incorporadas

YAGUARETÉ ofrece funciones matemáticas integradas en el runtime:

### 1. `MOD(dividendo, divisor)`
Devuelve el resto de la división entre dos enteros:
```yaguarete
ENTERO resto = MOD(29, 6)
IMPRIMIR("El resto es: " + resto) // Imprime: 5
```

### 2. `ESPAR(numero)`
Devuelve `VERDADERO` si el número entero recibido es divisible por 2:
```yaguarete
SI (ESPAR(42))
    IMPRIMIR("42 es par")
FIN SI
```

### 3. `ESIMPAR(numero)`
Devuelve `VERDADERO` si el número entero recibido no es divisible por 2:
```yaguarete
SI (ESIMPAR(15))
    IMPRIMIR("15 es impar")
FIN SI
```

---

## 📝 Operadores de Asignación

* **`=`**: Asignación simple (`variable = expresion`).
* **`+=`**: Suma acumulativa (`acumulador += valor`).
* **`-=`**: Resta acumulativa (`saldo -= descuento`).

---

## 📊 Tabla de Precedencia de Operadores

A continuación se muestra el orden de evaluación de mayor a menor prioridad:

1. **Agrupación y Llamadas**: Paréntesis `(...)`, corchetes `[...]`, accesos a campos `.` y llamadas a funciones `f(...)`.
2. **Operadores Unarios**: `NO`, `!`, `-` (negativo), `+` (positivo).
3. **Multiplicación y División**: `*`, `/`, `//` (división entera), `%` (módulo).
4. **Suma y Resta**: `+`, `-`.
5. **Operadores Relacionales**: `<`, `<=`, `>`, `>=`.
6. **Operadores de Igualdad**: `==`, `!=`.
7. **Conjunción Lógica**: `Y`, `&&`.
8. **Disyunción Lógica**: `O`, `||`.
9. **Asignaciones**: `=`, `+=`, `-=`.
