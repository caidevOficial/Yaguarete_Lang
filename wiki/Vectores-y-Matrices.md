# 📊 Vectores y Matrices (Arreglos Homogéneos)

En **YAGUARETÉ**, los arreglos son colecciones contiguas y fuertemente tipadas en memoria. Se clasifican en arreglos unidimensionales (**Vectores**) y bidimensionales (**Matrices**).

---

## 📐 Vectores Unidimensionales (`TIPO[]`)

### Declaración e Inicialización

Existen tres formas de declarar e inicializar un vector:

1. **Con tamaño predefinido**:
   ```yaguarete
   ENTERO calificaciones[5] // Vector de 5 enteros inicializados automáticamente en 0
   ```
2. **Con literales explícitos**:
   ```yaguarete
   CADENA frutas[] = ["Manzana", "Banana", "Naranja", "Pera"]
   ```
3. **Vector vacío para crecimiento dinámico**:
   ```yaguarete
   REAL mediciones[] = []
   ```

### Indexación (Base 0) y Seguridad de Límites

El acceso a los elementos se realiza con corchetes `v[indice]`. La indexación comienza en `0` y culmina en `largo - 1`:

```yaguarete
INICIO
    ENTERO numeros[3] = [10, 20, 30]
    IMPRIMIR("Primer elemento: " + numeros[0]) // 10
    IMPRIMIR("Último elemento: " + numeros[2]) // 30
FIN
```

> ⚠️ **Protección contra desbordamientos**: Si se intenta acceder a un índice fuera de rango, el motor nativo de Rust detiene la ejecución inmediatamente con un reporte de error detallado:
> ```text
> Error de ejecución en línea 5, columna 9: Índice fuera de límites: 4 (tamaño: 3)
> ```

---

## 📏 Propiedad `.largo`

Todo vector cuenta con la propiedad `.largo` que devuelve la cantidad actual de elementos como un `ENTERO`:

```yaguarete
INICIO
    CADENA lenguajes[] = ["YAGUARETÉ", "Rust", "C", "Python"]
    IMPRIMIR("Cantidad de lenguajes: " + lenguajes.largo) // 4

    PARA (ENTERO i = 0; i < lenguajes.largo; i++)
        IMPRIMIR("Elemento en [" + i + "]: " + lenguajes[i])
    FIN PARA
FIN
```

---

## ➕ Concatenación de Vectores (`+`)

Dos vectores del mismo tipo pueden fusionarse de forma natural utilizando el operador suma `+`:

```yaguarete
INICIO
    ENTERO parte_a[] = [1, 2, 3]
    ENTERO parte_b[] = [4, 5, 6]
    ENTERO combinado[] = parte_a + parte_b

    IMPRIMIR("Largo combinado: " + combinado.largo) // 6
    IMPRIMIR("Elemento en [4]: " + combinado[4])    // 5
FIN
```

---

## 🔲 Matrices Bidimensionales (`TIPO[][]`)

Las matrices modelan tablas o grillas rectangulares organizadas por filas y columnas:

### Declaración e Inicialización

```yaguarete
INICIO
    // Matriz de 3 filas por 4 columnas (12 celdas numéricas)
    ENTERO tablero[3][4]

    // Asignación de valores por celda
    tablero[0][0] = 100
    tablero[1][2] = 50
    tablero[2][3] = 99

    IMPRIMIR("Valor en fila 1, col 2: " + tablero[1][2]) // 50
FIN
```

### Recorrido Bidimensional Anidado

```yaguarete
INICIO
    ENTERO filas = 3
    ENTERO columnas = 3
    ENTERO identidad[3][3]

    // Crear matriz identidad
    PARA (ENTERO f = 0; f < filas; f++)
        PARA (ENTERO c = 0; c < columnas; c++)
            SI (f == c)
                identidad[f][c] = 1
            SINO
                identidad[f][c] = 0
            FIN SI
        FIN PARA
    FIN PARA
FIN
```

---

## 🧮 Aplanamiento y Mapeo con `//` y `%`

En algoritmos avanzados (como *Selection Sort* sobre matrices o transformaciones lineales), es muy común recorrer una matriz 2D como si fuera un arreglo plano 1D desde la posición $0$ hasta $(filas \times columnas) - 1$.

YAGUARETÉ permite realizar este cálculo limpiamente usando la **división entera** `//` para la fila y el **módulo** `%` para la columna:

$$\text{fila} = p // \text{columnas}$$
$$\text{columna} = p \% \text{columnas}$$

```yaguarete
INICIO
    ENTERO filas = 2
    ENTERO columnas = 3
    ENTERO m[2][3]
    m[0][0] = 10; m[0][1] = 20; m[0][2] = 30
    m[1][0] = 40; m[1][1] = 50; m[1][2] = 60

    ENTERO total_celdas = filas * columnas

    PARA (ENTERO p = 0; p < total_celdas; p++)
        ENTERO f = p // columnas
        ENTERO c = p % columnas
        IMPRIMIR("Posición plana " + p + " -> [" + f + "][" + c + "] = " + m[f][c])
    FIN PARA
FIN
```

---

## 🏢 Vectores de Estructuras

Es posible crear arreglos cuyos elementos sean registros estructurados:

```yaguarete
ESTRUCTURA Alumno:
    CADENA nombre
    ENTERO nota
FIN ESTRUCTURA

INICIO
    Alumno curso[2]
    curso[0].nombre = "María"
    curso[0].nota = 9
    curso[1].nombre = "Juan"
    curso[1].nota = 8

    PARA (ENTERO i = 0; i < curso.largo; i++)
        IMPRIMIR(curso[i].nombre + " tiene nota: " + curso[i].nota)
    FIN PARA
FIN
```
