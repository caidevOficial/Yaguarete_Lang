# 🛡️ Sistema de Tipos Estático y Fuerte

El sistema de tipos de **YAGUARETÉ** fue diseñado bajo el principio de **seguridad y previsibilidad**. A diferencia de los lenguajes dinámicos donde una variable puede cambiar de tipo arbitrariamente, en YAGUARETÉ cada variable posee un tipo formal declarado que es verificado durante el análisis del programa.

---

## 📊 Tipos de Datos Primitivos

| Tipo | Representación en Rust | Descripción | Valor por Defecto |
| :--- | :--- | :--- | :--- |
| **`ENTERO`** | Entero con signo de 64 bits (`i64`) | Números enteros positivos, negativos y cero. Rango: $-2^{63}$ a $2^{63}-1$. | `0` |
| **`FLOTANTE`** | IEEE 754 de precisión simple (`f32`) | Números con parte decimal de 32 bits. Ideal para cálculos gráficos o pedagógicos estándar. | `0.0` |
| **`REAL`** | IEEE 754 de precisión doble (`f64`) | Números con parte decimal de 64 bits y alta precisión científica. | `0.0` |
| **`CADENA`** | Cadena de texto UTF-8 (`String`) | Secuencias de texto delimitadas por comillas dobles `"..."` o simples `'...'`. | `""` |
| **`CARACTER`** | Carácter Unicode (`char`) | Un único carácter encerrado entre comillas simples `'a'`, `'#'`. | `' '` |
| **`BOOLEANO`** | Valor lógico booleano (`bool`) | Literales `VERDADERO` o `FALSO` (admite `TRUE` o `FALSE`). | `FALSO` |
| **`ARCHIVO`** | Descriptor de E/S (`Rc<RefCell<ArchivoHandle>>`) | Puntero a un flujo de archivo en disco para lectura o escritura secuencial. | `<No inicializado>` |
| **`VOID`** | Tipo vacío (`()`) | Empleado como tipo de retorno en `PROCEDIMIENTO` o funciones sin retorno. | N/A |
| **`NULO`** | Referencia nula universal | Representa la ausencia de valor o final de una estructura dinámica. | `NULO` |

---

## 🔍 Diferencia entre `FLOTANTE` y `REAL`

Muchos lenguajes de alto nivel confunden o unifican todos los números decimales bajo un mismo rótulo genérico. En las carreras de ciencias de la computación e ingeniería de software, enseñar la distinción entre **precisión simple** y **precisión doble** según la norma IEEE 754 es fundamental para comprender:
1. El consumo de memoria en matrices numéricas a gran escala.
2. Los errores de redondeo y pérdida de significación en cálculo numérico.

* **`FLOTANTE`** ocupa 4 bytes (32 bits) en memoria.
* **`REAL`** ocupa 8 bytes (64 bits) en memoria y ofrece 53 bits de mantisa (aproximadamente 15 a 17 dígitos decimales significativos).

```yaguarete
INICIO
    FLOTANTE peso = 72.45
    REAL constante_planck = 0.000000000000000000000000000000000662607015
    IMPRIMIR("Peso: " + peso)
    IMPRIMIR("Constante: " + constante_planck)
FIN
```

---

## 📦 Tipos Compuestos y Derivados

Además de los primitivos, el sistema de tipos soporta tipos de orden superior:

### 1. Vectores Unidimensionales (`TIPO[]`)
Arreglos contiguos de elementos del mismo tipo:
```yaguarete
ENTERO edades[5] = [18, 20, 22, 19, 25]
CADENA paises[] = ["Argentina", "Uruguay", "Paraguay"]
```

### 2. Matrices Bidimensionales (`TIPO[][]`)
Tablas multidimensionales estructuradas en filas y columnas:
```yaguarete
REAL tablero[3][3]
```

### 3. Estructuras Personalizadas (`ESTRUCTURA`)
Tipos de datos definidos por el usuario que agrupan múltiples campos tipados:
```yaguarete
ESTRUCTURA Estudiante:
    CADENA nombre
    ENTERO legajo
    REAL promedio
FIN ESTRUCTURA
```

---

## 🔄 Reglas de Coerción y Compatibilidad de Tipos

YAGUARETÉ implementa un modelo de compatibilidad controlado en [`eval.rs`](file:///crates/yaguarete_core/src/eval.rs):

1. **Promoción Numérica Automática**:
   * Si se asigna un `ENTERO` a una variable de tipo `REAL` o `FLOTANTE`, el valor se promociona automáticamente sin pérdida de información (ej. `25` se convierte en `25.0`).
   * Si se asigna un `REAL` o `FLOTANTE` a un `ENTERO`, el motor trunca la parte decimal (`25.8` se convierte en `25`), manteniendo la integridad del tipo entero.

2. **Incompatibilidad Estricta**:
   * No se permite asignar una `CADENA` o un `BOOLEANO` a un `ENTERO`. El compilador emitirá un `Error de tipos` con indicación de línea y columna:
   ```text
   Error de tipos en línea 4, columna 5: No se puede asignar 'CADENA' a una variable de tipo 'ENTERO'.
   ```

3. **Concatenación Universal de Cadenas**:
   * El operador de suma `+` entre una `CADENA` y cualquier otro tipo (`ENTERO`, `REAL`, `BOOLEANO`, `CARACTER`, `ESTRUCTURA`) convierte automáticamente el operando a su representación en texto para facilitar la generación de mensajes e informes de consola:
   ```yaguarete
   ENTERO dias = 7
   IMPRIMIR("Una semana tiene: " + dias + " días.")
   // Salida: Una semana tiene: 7 días.
   ```

---

## 🪹 Inicialización y Valores por Defecto

En YAGUARETÉ **las variables nunca contienen basura de memoria**, garantizando un entorno de ejecución predecible y seguro. Es fundamental distinguir la regla de inicialización por defecto según el contexto de la variable:

### 1. Variables Escalares Independientes
Cuando se declara una variable escalar local o global sin inicializador explícito, el entorno le asigna automáticamente el valor neutro canónico correspondiente a su tipo:
* `ENTERO`: `0`
* `REAL`: `0.0`
* `CADENA`: `""`
* `BOOLEANO`: `FALSO`
* **Arreglos/Vectores**: se dimensionan inicializando cada elemento con el valor neutro de su tipo base (por ejemplo, enteros en `0`; en vectores de estructuras, cada instancia se crea con todos sus atributos inicializados en `NULO`).

```yaguarete
INICIO
    ENTERO contador      // Inicia automáticamente en 0
    BOOLEANO activo      // Inicia en FALSO
    CADENA log           // Inicia en ""
    REAL balance         // Inicia en 0.0

    IMPRIMIR("Contador: " + contador)
    IMPRIMIR("Activo: " + activo)
FIN
```
Salida en consola:
```text
Contador: 0
Activo: FALSO
```

### 2. Atributos de Estructuras (`ESTRUCTURA`) (v0.2.1+)
A diferencia de las variables escalares independientes, **todos los atributos de cualquier estructura** (`ESTRUCTURA`) —tanto los tipos primitivos (`ENTERO`, `FLOTANTE`, `REAL`, `CADENA`, `CARACTER`, `BOOLEANO`) como las referencias a otras estructuras anidadas o autorreferenciales— **se inicializan automáticamente en `NULO` por defecto** hasta que el programador les asigne un valor explícitamente.

```yaguarete
ESTRUCTURA Registro:
    ENTERO id
    CADENA nombre
    BOOLEANO confirmado
FIN ESTRUCTURA

INICIO
    Registro reg
    // reg.id, reg.nombre y reg.confirmado inician todos en NULO
    SI (reg.id == NULO Y reg.nombre == NULO)
        IMPRIMIR("Los atributos de la estructura se inicializan en NULO por defecto.")
    FIN SI
FIN
```

Esta distinción arquitectónica:
* **Elimina la auto-instanciación anidada no deseada**: Evita crear objetos hijos recursivos artificiales en memoria que podrían derivar en bucles infinitos o desbordamientos de memoria.
* **Permite comprobaciones de estado no inicializado**: Facilita evaluar de manera inequívoca si un atributo fue fijado por el usuario o aún no (`campo == NULO` vs. `campo != NULO`), diferenciándolo limpiamente de un valor neutro legítimo (como `0`, `""` o `FALSO`).
* **Optimiza estructuras dinámicas**: Agiliza la implementación de listas enlazadas, pilas, colas y árboles binarios donde la ausencia de sucesor se representa naturalmente con `NULO`.
