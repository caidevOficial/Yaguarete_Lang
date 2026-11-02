# 🥞 Pilas y Colas (Estructuras Dinámicas LIFO y FIFO)

Las estructuras de datos dinámicas son uno de los componentes pedagógicos centrales de **YAGUARETÉ**. A través de `pilas.yaguarete` y `colas.yaguarete`, el lenguaje demuestra cómo implementar tipos de datos abstractos (TDA) eficientes con complejidad temporal $\mathcal{O}(1)$ en todas sus operaciones elementales.

---

## 🥞 Módulo Pilas (LIFO: *Last In, First Out*)

En una **Pila**, el último elemento en ingresar es el primero en ser extraído. Esta implementación (`pilas.yaguarete`) utiliza una **lista doblemente enlazada** con referencias bidireccionales en cada nodo y dos punteros de control en la estructura contenedora (`procesar_primero` para la cima y `procesar_ultimo` para la base).

### Definición de Estructuras

```yaguarete
ESTRUCTURA NodoCliente:
    CADENA nombre
    NodoCliente proximo_cliente    // Puntero hacia la base de la pila
    NodoCliente anterior_cliente   // Puntero hacia la cima de la pila
FIN ESTRUCTURA

ESTRUCTURA Pila:
    NodoCliente procesar_primero   // Cima / tope de la pila (inserción y extracción)
    NodoCliente procesar_ultimo    // Base de la pila (elemento más antiguo)
FIN ESTRUCTURA
```

### Funciones Constructoras

```yaguarete
FUNCION instanciar_nodo_cliente(): NodoCliente
    NodoCliente nodo
    nodo.nombre = NULO
    nodo.proximo_cliente = NULO
    nodo.anterior_cliente = NULO
    RETORNO nodo
FIN FUNCION

FUNCION instanciar_pila(): Pila
    Pila pila
    pila.procesar_primero = NULO
    pila.procesar_ultimo = NULO
    RETORNO pila
FIN FUNCION
```

### Operación Apilar / *PUSH* ($\mathcal{O}(1)$)

```yaguarete
PROCEDIMIENTO agregar_elemento_pila(pila: Pila, nuevo_nodo: NodoCliente)
    SI (nuevo_nodo == NULO)
        RETORNO
    FIN SI

    SI (pila.procesar_primero != NULO)
        // La pila ya tiene elementos: se inserta en la cima
        nuevo_nodo.proximo_cliente = pila.procesar_primero
        nuevo_nodo.anterior_cliente = NULO
        pila.procesar_primero.anterior_cliente = nuevo_nodo
        pila.procesar_primero = nuevo_nodo
    SINO
        // La pila está vacía: el nodo es cima y base simultáneamente
        nuevo_nodo.proximo_cliente = NULO
        nuevo_nodo.anterior_cliente = NULO
        pila.procesar_primero = nuevo_nodo
        pila.procesar_ultimo = nuevo_nodo
    FIN SI
FIN PROCEDIMIENTO
```

### Operación Desapilar / *POP* ($\mathcal{O}(1)$)

```yaguarete
FUNCION sacar_elemento_pila(pila: Pila): NodoCliente
    SI (pila.procesar_primero == NULO)
        RETORNO NULO // Pila vacía
    FIN SI

    NodoCliente nodo_retorno = pila.procesar_primero
    pila.procesar_primero = pila.procesar_primero.proximo_cliente

    SI (pila.procesar_primero != NULO)
        // Todavía quedan elementos: la nueva cima no tiene anterior
        pila.procesar_primero.anterior_cliente = NULO
    SINO
        // La pila se vació completamente: la base debe apuntar a NULO
        pila.procesar_ultimo = NULO
    FIN SI

    // Aislamiento de memoria: limpiar punteros del nodo extraído
    nodo_retorno.proximo_cliente = NULO
    nodo_retorno.anterior_cliente = NULO

    RETORNO nodo_retorno
FIN FUNCION
```

---

## 🎟️ Módulo Colas (FIFO: *First In, First Out*)

En una **Cola**, el primer elemento en ingresar es el primero en ser atendido. Esta implementación (`colas.yaguarete`) utiliza una **lista simplemente enlazada** optimizada con dos punteros: `procesar_primero` (frente de atención) y `procesar_ultimo` (final de inserción), logrando tiempo constante $\mathcal{O}(1)$ en ambos extremos sin sobrecosto de memoria.

### Definición de Estructuras

```yaguarete
ESTRUCTURA NodoCliente:
    CADENA nombre
    NodoCliente proximo_cliente    // Puntero hacia el siguiente cliente en la fila
FIN ESTRUCTURA

ESTRUCTURA Cola:
    NodoCliente procesar_primero   // Frente de la cola (próximo a ser atendido)
    NodoCliente procesar_ultimo    // Final de la cola (último agregado)
FIN ESTRUCTURA
```

### Funciones Constructoras

```yaguarete
FUNCION instanciar_nodo_cliente(): NodoCliente
    NodoCliente nodo
    nodo.nombre = NULO
    nodo.proximo_cliente = NULO
    RETORNO nodo
FIN FUNCION

FUNCION instanciar_cola(): Cola
    Cola cola
    cola.procesar_primero = NULO
    cola.procesar_ultimo = NULO
    RETORNO cola
FIN FUNCION
```

### Operación Encolar / *ENQUEUE* ($\mathcal{O}(1)$)

```yaguarete
PROCEDIMIENTO agregar_elemento_cola(cola: Cola, nuevo_nodo: NodoCliente)
    SI (nuevo_nodo == NULO)
        RETORNO
    FIN SI

    nuevo_nodo.proximo_cliente = NULO

    SI (cola.procesar_ultimo != NULO)
        // La cola tiene elementos: se enlaza al final existente
        cola.procesar_ultimo.proximo_cliente = nuevo_nodo
        cola.procesar_ultimo = nuevo_nodo
    SINO
        // La cola está vacía: el nodo es frente y final a la vez
        cola.procesar_primero = nuevo_nodo
        cola.procesar_ultimo = nuevo_nodo
    FIN SI
FIN PROCEDIMIENTO
```

### Operación Desencolar / *DEQUEUE* ($\mathcal{O}(1)$)

```yaguarete
FUNCION sacar_elemento_cola(cola: Cola): NodoCliente
    SI (cola.procesar_primero == NULO)
        RETORNO NULO // Cola vacía
    FIN SI

    NodoCliente nodo_retorno = cola.procesar_primero
    cola.procesar_primero = cola.procesar_primero.proximo_cliente
    nodo_retorno.proximo_cliente = NULO // Desconectar nodo extraído

    // Sincronización de vaciado: si el frente es NULO, la base también debe serlo
    SI (cola.procesar_primero == NULO)
        cola.procesar_ultimo = NULO
    FIN SI

    RETORNO nodo_retorno
FIN FUNCION
```

---

## ⚖️ Tabla Comparativa de Complejidades

| Estructura | Operación | Complejidad Temporal | Complejidad Espacial | Extremo Operado |
| :--- | :--- | :---: | :---: | :--- |
| **Pila (LIFO)** | `agregar_elemento_pila` (Push) | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | Cima (`procesar_primero`) |
| **Pila (LIFO)** | `sacar_elemento_pila` (Pop) | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | Cima (`procesar_primero`) |
| **Cola (FIFO)** | `agregar_elemento_cola` (Enqueue) | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | Final (`procesar_ultimo`) |
| **Cola (FIFO)** | `sacar_elemento_cola` (Dequeue) | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | Frente (`procesar_primero`) |

Ambas implementaciones son resistentes a referencias colgantes (*dangling references*) gracias al reseteo estricto de punteros al valor universal `NULO` cuando se extraen nodos.
