# 🧩 Estructuras de Datos, Referencias y el Puntero `NULO`

Las estructuras (`ESTRUCTURA`) son los tipos de datos compuestos definidos por el usuario en **YAGUARETÉ**. Permiten agrupar múltiples campos heterogéneos bajo un mismo nombre de tipo y construir estructuras dinámicas enlazadas (listas, pilas, colas y árboles) mediante referencias en memoria.

---

## 🏛️ Declaración de una `ESTRUCTURA`

Una estructura se define indicando el identificador del tipo seguido de dos puntos `:`, los nombres y tipos de cada campo, y cerrando con `FIN ESTRUCTURA`:

```yaguarete
ESTRUCTURA Libro:
    CADENA titulo
    CADENA autor
    ENTERO anio_publicacion
    REAL precio
    BOOLEANO disponible
FIN ESTRUCTURA
```

---

## 📦 Instanciación y Acceso a Campos

Para instanciar una estructura, se declara una variable con el nombre del tipo. Sus campos se inicializan automáticamente en sus valores por defecto (`0`, `0.0`, `""`, `FALSO` o `NULO`):

```yaguarete
INICIO
    Libro mi_libro

    mi_libro.titulo = "Ficciones"
    mi_libro.autor = "Jorge Luis Borges"
    mi_libro.anio_publicacion = 1944
    mi_libro.precio = 3500.50
    mi_libro.disponible = VERDADERO

    IMPRIMIR("Libro: " + mi_libro.titulo + " de " + mi_libro.autor)
    IMPRIMIR("Ficha completa: " + mi_libro)
FIN
```

Salida al imprimir una estructura completa:
```text
Ficha completa: Libro{titulo: 'Ficciones', autor: 'Jorge Luis Borges', anio_publicacion: 1944, precio: 3500.5, disponible: VERDADERO}
```

---

## 🧠 Modelo de Memoria y Referencias Compartidas

A nivel interno en el motor de Rust, cada instancia de una estructura se almacena en un puntero inteligente con conteo de referencias y mutabilidad interior:

```rust
Rc<RefCell<StructInstance>>
```

Esto implica que:
1. **Semántica de Referencia**: Si asignas una variable de tipo estructura a otra (`Libro copia = original`), ambas variables apuntan **exactamente a la misma instancia en memoria**. Cualquier cambio a través de `copia` se refleja inmediatamente en `original`.
2. **Paso Eficiente a Subrutinas**: Al pasar una estructura como parámetro a una `FUNCION` o `PROCEDIMIENTO`, no se realiza un clonado profundo de los campos, garantizando tiempo $\mathcal{O}(1)$ en la invocación.

---

## 🪹 El Puntero Universal `NULO`

En estructuras dinámicas, `NULO` (también aceptado como `NULL` o `NONE`) representa la ausencia de un nodo o el extremo terminal de una lista:

* Es el valor de referencia por defecto para campos de tipo autorreferencial.
* Puede compararse explícitamente mediante `== NULO` o `!= NULO`.

```yaguarete
ESTRUCTURA Nodo:
    CADENA valor
    Nodo siguiente
FIN ESTRUCTURA

INICIO
    Nodo cabeza
    cabeza.valor = "Primer elemento"
    cabeza.siguiente = NULO // Marcador explícito de fin de lista

    SI (cabeza.siguiente == NULO)
        IMPRIMIR("La lista tiene un único nodo.")
    FIN SI
FIN
```

### 🛡️ Protección contra Desreferenciación Nula

Si el código intenta acceder a un campo sobre una referencia que vale `NULO`, el intérprete detiene la ejecución de forma segura con un diagnóstico preciso en lugar de producir una falla catastrófica de memoria (*segmentation fault*):

```text
Error de ejecución en línea 12, columna 15: Intento de acceder al campo 'valor' en una referencia nula (NULO).
```

---

## 🔗 Estructuras Autorreferenciales y Recorridos Dinámicos

Una estructura es **autorreferencial** cuando uno o más de sus campos tienen como tipo a la misma estructura que se está definiendo. Esto permite enlazar nodos en secuencias continuas:

```yaguarete
ESTRUCTURA NodoCadena:
    CADENA texto
    NodoCadena proximo
FIN ESTRUCTURA

INICIO
    NodoCadena n1
    NodoCadena n2
    NodoCadena n3

    n1.texto = "A"
    n2.texto = "B"
    n3.texto = "C"

    // Enlazar los tres nodos: A -> B -> C -> NULO
    n1.proximo = n2
    n2.proximo = n3
    n3.proximo = NULO

    // Recorrido secuencial hasta encontrar NULO
    NodoCadena actual
    actual = n1

    MIENTRAS (actual != NULO)
        IMPRIMIR("Visitando nodo: " + actual.texto)
        actual = actual.proximo
    FIN MIENTRAS
FIN
```

---

## 💡 Sugerencias Inteligentes de Campos (Anti-Typos)

El analizador de YAGUARETÉ ayuda a los estudiantes a encontrar errores tipográficos en los nombres de los miembros. Si escribes un campo inexistente, el motor calcula la distancia ortográfica frente a los campos válidos y propone la corrección en el mensaje de error:

```yaguarete
Libro l
l.titul = "El Aleph" // Error: el campo correcto es 'titulo'
```
Salida en consola:
```text
Error de ejecución en línea 2, columna 3: La estructura 'Libro' no tiene el campo 'titul'. ¿Quisiste decir 'titulo'?
```
