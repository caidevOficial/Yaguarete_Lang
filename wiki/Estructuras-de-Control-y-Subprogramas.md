# 🔄 Estructuras de Control, Funciones y Procedimientos

Las estructuras de control en **YAGUARETÉ** proporcionan una sintaxis clara, estructurada y pedagógica en español con delimitadores explícitos de fin de bloque (`FIN SI`, `FIN PARA`, etc.), eliminando la ambigüedad de llaves anidadas o sangrías accidentales.

---

## 🏛️ Bloque Maestro del Programa

Un programa en YAGUARETÉ puede escribirse dentro del bloque maestro canónico `INICIO ... FIN` o directamente con sentencias en el nivel superior del archivo:

```yaguarete
INICIO
    // Punto de entrada principal del algoritmo
    IMPRIMIR("Programa iniciado")
FIN
```

---

## 🔀 Estructuras Condicionales: `SI` / `SINO SI` / `SINO`

La estructura condicional permite bifurcar el flujo de ejecución evaluando expresiones booleanas. Se admiten múltiples ramas intermedias con `SINO SI` y una rama alternativa por defecto con `SINO`:

```yaguarete
INICIO
    REAL calificacion = 8.5

    SI (calificacion >= 9.0)
        IMPRIMIR("Calificación: Sobresaliente")
    SINO SI (calificacion >= 7.0)
        IMPRIMIR("Calificación: Notable")
    SINO SI (calificacion >= 4.0)
        IMPRIMIR("Calificación: Aprobado")
    SINO
        IMPRIMIR("Calificación: Desaprobado - Requiere recuperación")
    FIN SI
FIN
```

---

## 🔁 Bucles e Iteraciones

YAGUARETÉ soporta los tres patrones canónicos de repetición:

### 1. Bucle `PARA` (Iteración Determinada)
Ideal cuando se conoce de antemano la cantidad de iteraciones. Se compone de inicialización, condición de permanencia y paso de actualización:

```yaguarete
INICIO
    ENTERO suma = 0

    // Suma los números pares del 0 al 10
    PARA (ENTERO i = 0; i <= 10; i += 2)
        suma += i
        IMPRIMIR("Paso " + i + " -> Suma acumulada: " + suma)
    FIN PARA

    IMPRIMIR("Total final: " + suma)
FIN
```

### 2. Bucle `MIENTRAS` (Precondición)
Evalúa la condición antes de cada iteración. Si la condición es inicialmente falsa, el cuerpo no se ejecuta ninguna vez:

```yaguarete
INICIO
    ENTERO energia = 100

    MIENTRAS (energia > 0)
        IMPRIMIR("Nivel de batería: " + energia + "%")
        energia -= 25
    FIN MIENTRAS

    IMPRIMIR("Dispositivo descargado")
FIN
```

### 3. Bucle `HACER ... MIENTRAS` (Postcondición)
Garantiza que el cuerpo se ejecute **al menos una vez**, evaluando la condición de repetición al final del bloque:

```yaguarete
INICIO
    ENTERO intentos = 0

    HACER
        intentos++
        IMPRIMIR("Intento de conexión número: " + intentos)
    MIENTRAS (intentos < 3)
    FIN HACER

    IMPRIMIR("Proceso finalizado tras " + intentos + " intentos")
FIN
```

---

## 📦 Modularización: Funciones y Procedimientos

Para estructurar algoritmos complejos, YAGUARETÉ distingue claramente entre rutinas que calculan y devuelven un valor (`FUNCION`) y aquellas que ejecutan una acción sin retornar valor (`PROCEDIMIENTO`):

### 1. `FUNCION` (Subrutina con Retorno)
Debe declarar el tipo de retorno después de los dos puntos `: TIPO` y retornar obligatoriamente un valor con `RETORNAR` o `RETORNO`:

```yaguarete
// Definición de una función matemática
FUNCION calcular_potencia(base: ENTERO, exponente: ENTERO): ENTERO
    ENTERO resultado = 1
    PARA (ENTERO k = 1; k <= exponente; k++)
        resultado = resultado * base
    FIN PARA
    RETORNAR resultado
FIN FUNCION

INICIO
    ENTERO pot = calcular_potencia(2, 8)
    IMPRIMIR("2 elevado a la 8 es: " + pot) // Imprime: 256
FIN
```

### 2. `PROCEDIMIENTO` (Subrutina sin Retorno / `VOID`)
Diseñado para operaciones con efectos secundarios (mostrar datos, modificar estructuras recibidas por referencia o escribir archivos):

```yaguarete
PROCEDIMIENTO imprimir_separador(titulo: CADENA)
    IMPRIMIR("======================================")
    IMPRIMIR("   " + titulo)
    IMPRIMIR("======================================")
FIN PROCEDIMIENTO

INICIO
    imprimir_separador("REPORTE DE ALUMNOS")
    imprimir_separador("FIN DEL INFORME")
FIN
```

---

## 🔒 Ámbitos Léxicos y Aislamiento de Variables

* **Ámbito Local**: Las variables declaradas dentro de una `FUNCION` o `PROCEDIMIENTO` residen en un marco de pila (`Scope`) independiente y son destruidas al salir de la rutina.
* **Paso por Valor y Referencia**:
  * Los tipos primitivos (`ENTERO`, `REAL`, `BOOLEANO`, `CADENA`) se pasan por valor.
  * Las estructuras (`ESTRUCTURA`) se administran como punteros de referencia compartida (`Rc<RefCell<StructInstance>>`), por lo que cualquier modificación sobre sus campos dentro de una subrutina impacta directamente en la instancia original.
