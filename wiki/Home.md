# 🐆 Bienvenido a la Wiki Oficial de YAGUARETÉ

[![Version](https://img.shields.io/badge/versión-0.1.1-00d4ff.svg?style=flat-square)](https://github.com/caidevOficial/Yaguarete_Lang/releases/tag/v0.1.1)
[![Engine](https://img.shields.io/badge/motor-Rust%20Nativo-ff6600.svg?style=flat-square)](https://github.com/caidevOficial/Yaguarete_Lang)
[![License](https://img.shields.io/badge/licencia-GPL--3.0-blue.svg?style=flat-square)](https://github.com/caidevOficial/Yaguarete_Lang/blob/main/LICENSE)
[![VS Code](https://img.shields.io/badge/VS%20Code-v0.1.1-007acc.svg?style=flat-square)](https://github.com/caidevOficial/Yaguarete_Lang/tree/main/extension-vscode)

**YAGUARETÉ** es un lenguaje de programación educativo en español, fuertemente tipado e interpretado por un **motor nativo escrito en Rust**. Está diseñado para transformar la enseñanza tradicional de algoritmos y estructuras de datos en universidades e institutos técnicos, eliminando la ambigüedad del pseudocode informal sin sacrificar la expresividad pedagógica de la lengua hispana.

---

## 🎯 ¿Por qué YAGUARETÉ?

En la mayoría de los cursos de ciencias de la computación, el pseudocódigo clásico presenta un grave problema: **no se ejecuta o carece de rigor de tipos**. Cuando los alumnos pasan a lenguajes industriales como C++, Java o Rust, sufren una brecha conceptual severa por falta de comprensión de:
1. Tipos de datos estrictos y límites de representación binaria.
2. Manejo de referencias, punteros y el concepto universal de `NULO`.
3. Estructuras de datos dinámicas en memoria (listas enlazadas, pilas y colas).
4. Depuración formal con puntos de interrupción (*breakpoints*) y evaluación de variables.

**YAGUARETÉ resuelve esto de forma nativa:**

* 🛡️ **Tipado Estático y Seguro**: No permite asignar un `REAL` a un `ENTERO` sin conversión explícita. Posee tipos primitivos precisos (`ENTERO`, `FLOTANTE`, `REAL`, `CADENA`, `CARACTER`, `BOOLEANO`, `ARCHIVO`, `VOID`, `NULO`).
* ⚡ **Rendimiento Nativo en Rust**: Tiempo de arranque en frío menor a **1 milisegundo** y ordenamiento de miles de elementos en **1 ms**, superando en más de 20x al prototipo inicial en Python.
* 🐞 **Depurador Nativo DAP (Debug Adapter Protocol)**: Permite colocar puntos de interrupción (*breakpoints*), avanzar paso a paso (*Step In*, *Step Over*, *Step Out*), inspeccionar la pila de llamadas y examinar campos de estructuras en tiempo real desde Visual Studio Code y Antigravity IDE.
* 💬 **Consola Interactiva REPL**: Permite probar expresiones, definir funciones sobre la marcha e inspeccionar la tabla de variables activas en memoria con el metacomando `:variables`.
* 🎨 **Ecosistema Visual Completo**: Extensión para VS Code con coloreado sintáctico TextMate idéntico a Dark+, tema de iconos oficial *"Jaguar Gamer"* y sugerencias IntelliSense contextuales.

---

## ⚡ Ejemplo Rápido: Primer Programa

Crea un archivo llamado `hola_mundo.yaguarete`:

```yaguarete
// Archivo: hola_mundo.yaguarete
INICIO
    CADENA mensaje = "¡Hola desde YAGUARETÉ v0.1.1!"
    IMPRIMIR(mensaje)

    PARA (ENTERO i = 1; i <= 5; i++)
        SI (ESPAR(i))
            IMPRIMIR("El número " + i + " es PAR")
        SINO
            IMPRIMIR("El número " + i + " es IMPAR")
        FIN SI
    FIN PARA
FIN
```

Ejecútalo desde tu terminal:

```bash
yaguarete run hola_mundo.yaguarete --time
```

---

## 📖 Mapa de Contenidos de la Wiki

Navega por las secciones detalladas utilizando el menú lateral o los enlaces directos a continuación:

### 1. 🚀 Primeros Pasos
* [[Instalación y Configuración|Instalacion-y-Primeros-Pasos]]: Cómo instalar el compilador nativo en Windows y Linux, configuración de variables de entorno PATH y resolución de dependencias.
* [[Interfaz CLI (yaguarete)|Interfaz-CLI-(yaguarete)]]: Guía completa de subcomandos `run`, `check`, `lsp-check`, `dap`, `update` y flags de medición de tiempo.
* [[Consola Interactiva REPL|Consola-Interactiva-REPL]]: Uso de la consola interactiva, buffer multilínea y metacomandos especiales (`:ayuda`, `:variables`, `:limpiar`, `:cancel`, `:salir`).

### 2. 📚 Fundamentos del Lenguaje
* [[Sistema de Tipos Estático|Sistema-de-Tipos-Estatico]]: Tipos primitivos, distinción `FLOTANTE` vs `REAL` (IEEE 754), compatibilidad, valores por defecto y *type safety*.
* [[Operadores Aritméticos y Lógicos|Operadores-Aritmeticos-y-Logicos]]: Operadores `+`, `-`, `*`, `/`, división entera `//`, módulo `%`, operadores relacionales y conectores lógicos `Y`, `O`, `NO`.
* [[Estructuras de Control y Funciones|Estructuras-de-Control-y-Subprogramas]]: Bloques `INICIO/FIN`, condicionales `SI / SINO SI / SINO`, bucles `PARA`, `MIENTRAS`, `HACER...MIENTRAS`, y modularización con `FUNCION` y `PROCEDIMIENTO`.
* [[Vectores y Matrices|Vectores-y-Matrices]]: Arreglos unidimensionales (`TIPO[]`), matrices bidimensionales (`TIPO[][]`), propiedad `.largo` y concatenación con `+`.
* [[Manejo de Archivos (ARCHIVO)|Manejo-de-Archivos-(ARCHIVO)]]: Tipo `ARCHIVO`, función `ABRIR_ARCHIVO`, lectura tipada automática con `.LEER_LINEA()`, escritura con `.ESCRIBIR_LINEA()`, detección de fin con `.findearchivo` y cierre seguro con `.CERRAR_ARCHIVO()`.

### 3. 🧩 Estructuras Avanzadas y Algoritmos
* [[Estructuras y Puntero NULO|Estructuras-de-Datos-y-Punteros]]: Bloques `ESTRUCTURA ... FIN ESTRUCTURA`, estructuras autorreferenciales, puntero universal `NULO` y sugerencias anti-errores tipográficos.
* [[Pilas y Colas (LIFO / FIFO)|Pilas-y-Colas-(Estructuras-Dinamicas)]]: Arquitectura de estructuras de datos en memoria dinámica: Pilas con listas doblemente enlazadas y Colas con listas simplemente enlazadas y doble puntero en tiempo $O(1)$.
* [[Recetario de Algoritmos|Recetario-de-Algoritmos]]: Implementaciones de algoritmos canónicos: Quicksort de 3 vías optimizado, Selection Sort matricial 2D, listas enlazadas dinámicas y cálculo de liquidación salarial.

### 4. 🛠️ Herramientas, Arquitectura y Comunidad
* [[Extensión VS Code y DAP|Extension-VS-Code-y-Depurador-DAP]]: Instalación del paquete `.vsix`, configuración del depurador DAP, puntos de interrupción, tema de iconos oficial y proveedores de autocompletado e IntelliSense.
* [[Arquitectura del Compilador|Arquitectura-del-Compilador-(Rust)]]: Arquitectura interna en Rust (crates `yaguarete_core`, `yaguarete_cli`, `yaguarete_installer`), autómata determinista `logos`, AST, intérprete basado en árboles y registro de decisiones arquitectónicas (`ADR-001`).
* [[Historial de Versiones (Changelog)|Historial-de-Versiones-(Changelog)]]: Bitácora de cambios detallada de la versión oficial estable `0.1.1` y las pre-releases semánticas `0.1.0-*`.

---

## 🤝 Cómo Contribuir

El código fuente de YAGUARETÉ es software libre publicado bajo la licencia **GNU General Public License v3.0 or later**. Invitamos a docentes, investigadores y desarrolladores a colaborar mediante:
- Reporte de errores y solicitud de características en [GitHub Issues](https://github.com/caidevOficial/Yaguarete_Lang/issues).
- Envío de pull requests con mejoras en el motor nativo o en la extensión de VS Code.
- Elaboración de material pedagógico y ejercicios prácticos en pseudocódigo.
