# 🍳 Recetario de Algoritmos Canónicos

Esta sección reúne implementaciones completas y verificadas de algoritmos clásicos de ciencias de la computación escritos en **YAGUARETÉ**, listos para ser utilizados como material de estudio o referencia práctica.

---

## 1. Quicksort Optimizado de 3 Vías ($O(N \log N)$)

El algoritmo **Quicksort** es uno de los métodos de ordenamiento más veloces. Esta versión (`sort.yaguarete`) implementa la partición en tres vías (*3-way partition*), dividiendo el arreglo en tres segmentos: menores que el pivote, iguales al pivote y mayores que el pivote. Es altamente eficiente incluso en presencia de elementos duplicados.

```yaguarete
// Archivo: quicksort.yaguarete
PROCEDIMIENTO intercambiar(arr: ENTERO[], i: ENTERO, j: ENTERO)
    ENTERO temp = arr[i]
    arr[i] = arr[j]
    arr[j] = temp
FIN PROCEDIMIENTO

PROCEDIMIENTO quicksort_recursivo(arr: ENTERO[], bajo: ENTERO, alto: ENTERO)
    SI (bajo >= alto)
        RETORNO
    FIN SI

    ENTERO pivote = arr[bajo]
    ENTERO menor = bajo
    ENTERO actual = bajo + 1
    ENTERO mayor = alto

    MIENTRAS (actual <= mayor)
        SI (arr[actual] < pivote)
            intercambiar(arr, menor, actual)
            menor++
            actual++
        SINO SI (arr[actual] > pivote)
            intercambiar(arr, actual, mayor)
            mayor--
        SINO
            actual++
        FIN SI
    FIN MIENTRAS

    quicksort_recursivo(arr, bajo, menor - 1)
    quicksort_recursivo(arr, mayor + 1, alto)
FIN PROCEDIMIENTO

PROCEDIMIENTO ordenar(arr: ENTERO[])
    SI (arr.largo > 1)
        quicksort_recursivo(arr, 0, arr.largo - 1)
    FIN SI
FIN PROCEDIMIENTO

INICIO
    ENTERO datos[] = [88, 14, 92, 45, 14, 3, 77, 21, 60]
    IMPRIMIR("Arreglo original: " + datos)

    ordenar(datos)

    IMPRIMIR("Arreglo ordenado: " + datos)
FIN
```

---

## 2. Selection Sort sobre Matriz 2D con `//` y `%`

Este algoritmo (`algoritmo.yaguarete`) ordena una matriz bidimensional completa *in-situ* utilizando el método de Selección Directa, mapeando las celdas bidimensionales a un índice plano mediante la **división entera** `//` y el **módulo** `%` sin requerir la creación de vectores intermedios:

```yaguarete
// Archivo: ordenar_matriz.yaguarete
INICIO
    ENTERO filas = 3
    ENTERO columnas = 3
    ENTERO matriz[3][3]

    // Carga de matriz desordenada
    matriz[0][0] = 45; matriz[0][1] = 12; matriz[0][2] = 89
    matriz[1][0] = 7;  matriz[1][1] = 33; matriz[1][2] = 21
    matriz[2][0] = 90; matriz[2][1] = 3;  matriz[2][2] = 56

    ENTERO total_celdas = filas * columnas

    // Algoritmo Selection Sort sobre grilla plana
    PARA (ENTERO i = 0; i < total_celdas - 1; i++)
        ENTERO min_pos = i

        PARA (ENTERO j = i + 1; j < total_celdas; j++)
            ENTERO fj = j // columnas
            ENTERO cj = j % columnas
            ENTERO fm = min_pos // columnas
            ENTERO cm = min_pos % columnas

            SI (matriz[fj][cj] < matriz[fm][cm])
                min_pos = j
            FIN SI
        FIN PARA

        SI (min_pos != i)
            ENTERO fi = i // columnas
            ENTERO ci = i % columnas
            ENTERO fmin = min_pos // columnas
            ENTERO cmin = min_pos % columnas

            ENTERO aux = matriz[fi][ci]
            matriz[fi][ci] = matriz[fmin][cmin]
            matriz[fmin][cmin] = aux
        FIN SI
    FIN PARA

    IMPRIMIR("Matriz 3x3 ordenada:")
    PARA (ENTERO f = 0; f < filas; f++)
        CADENA fila_texto = ""
        PARA (ENTERO c = 0; c < columnas; c++)
            fila_texto = fila_texto + matriz[f][c] + "\t"
        FIN PARA
        IMPRIMIR(fila_texto)
    FIN PARA
FIN
```

---

## 3. Lista Simplemente Enlazada Dinámica

Implementación canónica de una lista enlazada dinámica con inserción por posición, eliminación y recorrido lineal:

```yaguarete
// Archivo: lista_enlazada.yaguarete
ESTRUCTURA Nodo:
    CADENA producto
    Nodo siguiente
FIN ESTRUCTURA

ESTRUCTURA Lista:
    Nodo cabeza
    ENTERO cantidad
FIN ESTRUCTURA

PROCEDIMIENTO insertar_al_inicio(mi_lista: Lista, nombre_producto: CADENA)
    Nodo nuevo_nodo
    nuevo_nodo.producto = nombre_producto
    nuevo_nodo.siguiente = mi_lista.cabeza
    mi_lista.cabeza = nuevo_nodo
    mi_lista.cantidad++
FIN PROCEDIMIENTO

PROCEDIMIENTO imprimir_lista(mi_lista: Lista)
    IMPRIMIR("--- Lista de Productos (" + mi_lista.cantidad + " elementos) ---")
    Nodo actual = mi_lista.cabeza
    ENTERO pos = 1
    MIENTRAS (actual != NULO)
        IMPRIMIR(pos + ". " + actual.producto)
        actual = actual.siguiente
        pos++
    FIN MIENTRAS
    IMPRIMIR("------------------------------------------")
FIN PROCEDIMIENTO

INICIO
    Lista compras
    compras.cabeza = NULO
    compras.cantidad = 0

    insertar_al_inicio(compras, "Teclado Mecánico")
    insertar_al_inicio(compras, "Monitor 27 pulgadas")
    insertar_al_inicio(compras, "Mouse Gamer")

    imprimir_lista(compras)
FIN
```

---

## 4. Cálculo Salarial y Exportación de Archivo

Este algoritmo demuestra el uso integrado de cálculo matemático, condicionales y persistencia en disco con el descriptor `ARCHIVO`:

```yaguarete
// Archivo: liquidacion_sueldos.yaguarete
INICIO
    CADENA empleados[] = ["Ana Gómez", "Carlos Ruiz", "Sofía Paz"]
    REAL sueldos_base[] = [850000.0, 920000.0, 1150000.0]
    ENTERO horas_extras[] = [12, 0, 8]

    ARCHIVO reporte = ABRIR_ARCHIVO("reporte_sueldos.txt", "escritura")
    reporte.ESCRIBIR_LINEA("EMPLEADO\tSUELDO_BASE\tEXTRAS\tTOTAL_NETO")

    PARA (ENTERO i = 0; i < empleados.largo; i++)
        REAL valor_hora = sueldos_base[i] / 160.0
        REAL bono_extras = horas_extras[i] * valor_hora * 1.5
        REAL sueldo_bruto = sueldos_base[i] + bono_extras
        
        // Descuentos de ley (17% aportes previsionales y obra social)
        REAL descuentos = sueldo_bruto * 0.17
        REAL sueldo_neto = sueldo_bruto - descuentos

        CADENA registro = empleados[i] + "\t" + sueldos_base[i] + "\t" + horas_extras[i] + "\t" + sueldo_neto
        reporte.ESCRIBIR_LINEA(registro)
        IMPRIMIR("Procesado: " + empleados[i] + " -> Neto a cobrar: $" + sueldo_neto)
    FIN PARA

    reporte.CERRAR_ARCHIVO()
    IMPRIMIR("Reporte guardado exitosamente en 'reporte_sueldos.txt'.")
FIN
```
