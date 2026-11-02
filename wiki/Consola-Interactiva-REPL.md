# 💬 Consola Interactiva REPL (`yaguarete repl`)

La consola interactiva REPL (*Read-Eval-Print Loop*) de **YAGUARETÉ** permite a estudiantes, docentes y desarrolladores experimentar con algoritmos de forma inmediata, evaluar expresiones aritméticas y lógicas, y probar funciones sin necesidad de crear archivos temporales.

---

## 🚀 Inicio de la Consola

Para iniciar la consola REPL, ejecuta cualquiera de los siguientes comandos en tu terminal:

```bash
yaguarete repl
# o simplemente:
yaguarete
```

Al iniciar, se presentará el banner oficial del jaguar y el cursor interactivo en color magenta:

```text
Consola Interactiva REPL. Escribe expresiones o código de pseudocódigo.
Comandos especiales: :ayuda, :variables, :limpiar, :salir

yaguarete> 
```

---

## ⚡ Evaluación Inmediata de Expresiones

En el REPL no es obligatorio envolver el código en `INICIO ... FIN` ni escribir `IMPRIMIR(...)` para ver resultados. Si ingresas una expresión directa, el motor la evalúa y muestra su valor resultante:

```text
yaguarete> 10 + 20 * 3
70

yaguarete> "Yaguareté " + "v0.1.1"
Yaguareté v0.1.1

yaguarete> [1, 2, 3] + [4, 5]
[1, 2, 3, 4, 5]

yaguarete> ESPAR(42) Y (100 > 50)
VERDADERO
```

---

## 📝 Entrada Multilínea Inteligente

El motor incorpora un detector de completitud sintáctica (`is_input_complete`). Si abres un bloque de control (`SI`, `PARA`, `MIENTRAS`, `FUNCION`, `ESTRUCTURA`) o dejas paréntesis o corchetes abiertos, el prompt cambia automáticamente a `...` para permitirte continuar escribiendo:

```text
yaguarete> FUNCION sumar(a: ENTERO, b: ENTERO): ENTERO
...           RETORNO a + b
...       FIN FUNCION
yaguarete> sumar(15, 27)
42
```

---

## 📋 Metacomandos del REPL

Los comandos especiales inician con dos puntos `:` y permiten gestionar la sesión interactiva:

| Metacomando | Alias | Descripción |
| :--- | :--- | :--- |
| **`:variables`** | `:vars` | Despliega una tabla con todas las variables activas en memoria, sus tipos formales y sus valores actuales. |
| **`:ayuda`** | `:help`, `:?` | Muestra la guía de uso rápido, atajos y ejemplos de sintaxis. |
| **`:limpiar`** | `:clear` | Reinicia el intérprete por completo, liberando la memoria y eliminando variables y funciones declaradas. |
| **`:cancel`** | `:c` | Descarta el buffer multilínea actual si cometiste un error antes de cerrarlo. |
| **`:salir`** | `:exit`, `:quit`, `:q` | Cierra la sesión interactiva y regresa a la terminal del sistema. |

---

## 🔍 Inspección de Memoria en Vivo con `:variables`

El comando `:variables` dibuja una tabla Unicode que refleja con precisión el estado de la memoria del intérprete:

```text
yaguarete> ENTERO contador = 10
yaguarete> REAL factor = 1.75
yaguarete> CADENA usuario = "Facundo"
yaguarete> BOOLEANO verificado = VERDADERO
yaguarete> :variables
┌────────────┬──────────┬───────────┐
│ NOMBRE     │ TIPO     │ VALOR     │
├────────────┼──────────┼───────────┤
│ contador   │ ENTERO   │ 10        │
│ factor     │ REAL     │ 1.75      │
│ usuario    │ CADENA   │ Facundo   │
│ verificado │ BOOLEANO │ VERDADERO │
└────────────┴──────────┴───────────┘
```

---

## ⌨️ Atajos de Teclado y Control de Flujo

* **`Enter`**: Ejecuta la línea o avanza a la siguiente línea si el bloque no está completo.
* **`Ctrl + C`**: Interrumpe una ejecución prolongada o cancela la línea actual.
* **`Ctrl + D` (Linux/macOS) o `Ctrl + Z` luego `Enter` (Windows)**: Envía señal de Fin de Archivo (EOF) y sale del REPL.
