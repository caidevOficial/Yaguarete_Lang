# 📜 Historial de Versiones (Changelog Oficial)

Este documento registra la evolución histórica del lenguaje **YAGUARETÉ**, comenzando por la versión oficial estable más reciente (**0.2.1**) y documentando detalladamente cada versión previa y los hitos de maduración y estabilización organizados como pre-releases semánticas (SemVer).

---

## 🌟 Versiones Estables Oficiales

### [v0.2.1] - 2026-11-13 (Inicialización en NULO por Defecto & Estabilidad de Estructuras)
**Control Explícito de Memoria y Eliminación de Auto-Instanciación Anidada.**

* **Semántica de Estructuras (R1)**:
  * Todos los atributos de cualquier estructura (`ESTRUCTURA`), tanto tipos primitivos (`ENTERO`, `REAL`, `CADENA`, `BOOLEANO`) como tipos estructura compuestos y referencias, se inicializan automáticamente en `NULO` por defecto hasta que el programador les asigne un valor explícitamente.
  * **Eliminación de Auto-Instanciación Anidada**: Se previene la creación automática de sub-estructuras dummy recursivas, eliminando ciclos infinitos y desbordamientos de memoria en la instanciación de estructuras dinámicas.
  * **Comprobación de Nulos Segura**: Soporte completo y uniforme para comprobaciones directas de igualdad y desigualdad (`== NULO` y `!= NULO`) sobre atributos de estructuras sin asignar, funcionando con exactitud tanto en el motor de máquina virtual (`--engine vm`) como en el evaluador AST (`--engine ast`).
* **Compatibilidad y Verificación (R2)**:
  * **`semaforo.yaguarete`**: Corrección del ciclo infinito que ocurría en la simulación de tráfico al evaluar `calle_1.primer_auto == NULO`, permitiendo que la verificación de cola vacía opere correctamente y los autos fluyan de manera impecable.
  * **`colas.yaguarete`**: Validación integral del ciclo de vida FIFO de nodos (`NodoCliente`), garantizando el encolado y desencolado dinámico sin necesidad de asignaciones defensivas manuales.
  * **Suite de Pruebas**: 366 pruebas automatizadas superadas al 100% (271 en Rust con `cargo test --workspace` y 95 en Python con `python -m unittest discover -s tests -v`).
* **Infraestructura y Build (R3)**:
  * Sincronización de versión a `0.2.1` en `yaguarete_core`, `yaguarete_cli`, `yaguarete_installer`, `extension-vscode`, constantes de biblioteca, metadatos PE de Windows y banners de CLI.
  * Compilación y distribución de binarios oficiales (`yaguarete.exe`, `Instalador_Yaguarete.exe` con `yaguarete-language-support-0.2.1.vsix` embebida).

---

### [v0.2.0] - 2026-11-11 (Máquina Virtual Bytecode & Funciones Incorporadas)
**Salto Cuántico en Rendimiento y Ampliación de la Biblioteca Estándar.**

* **Motor de Máquina Virtual Bytecode (`yaguarete_core/src/vm/`)**:
  * Compilador de bytecode (`Compiler`) y VM basada en pila (`VM`) con conjunto de instrucciones compacto (`OpCode`) y pool de constantes (`Chunk`).
  * Resolución estática de variables locales a slots indexados O(1) en tiempo de compilación.
  * Saltos directos (`Jump`, `JumpIfFalse`, `Loop`), marcos de llamada aislados (`CallFrame`), recursión y estructuras autorreferenciales.
  * Modificación in-situ de vectores sin clonado (`SetIndexLocal`, `SetIndexGlobal`, etc.).
  * Selección de motor CLI vía `--engine <vm|ast>` (activado `vm` por defecto).
  * Aceleración de hasta 5.72x en benchmarks intensivos (Selection Sort 10k en 6.9s frente a 39.5s).
* **Optimizaciones en Intérprete AST (`eval.rs`)**:
  * `FxHashMap` en tablas de símbolos, pool de ámbitos (`scope_pool`) y atajos para `+=`, `-=`, `++`, `--`.
* **Nuevas Funciones Built-in del Lenguaje**:
  * Operador de potencia `^` y función `POTENCIA(base, exponente)`.
  * Matemáticas: `RAIZCUADRADA`, `ABS`, `ALEATORIO`, `MIN`, `MAX`, `REDONDEAR`, constante `PI`.
  * Entrada: `LEER()` / `LEER(mensaje)` con detección automática de tipo.
  * Cadenas: `MAYUSCULAS`, `MINUSCULAS`, `RECORTAR`, `DIVIDIR_CADENA`.
  * Conversión de tipos: `A_ENTERO`, `A_REAL`, `A_FLOTANTE`, `A_CADENA`, `A_BOOLEANO`.
* **Ecosistema y Herramientas**:
  * Extensión VS Code: ayuda de firma (*signature help*), autocompletado y sintaxis TextMate enriquecida.

---

### [v0.1.2] - 2026-11-05 (Evaluación en Cortocircuito Lógico)
* **Evaluación en Cortocircuito (*Short-Circuit Evaluation*)**:
  * Implementación para operadores booleanos `Y` y `O` en el evaluador de expresiones binarias (`eval.rs`), permitiendo guardas de seguridad contra desbordamientos y divisiones por cero.
* **Sincronización de Versión**: Sincronización general a `0.1.2` en todos los crates de Rust y metadatos de recursos.

---

### [v0.1.1] - 2026-10-30 (Primer Release Oficial: Identidad "Jaguar Gamer" & Electric Blue)
**Primer Release Oficial de YAGUARETÉ: Identidad "Jaguar Gamer" & Electric Blue.**

* **Rebranding Integral**: Transformación completa del lenguaje hacia la identidad oficial **YAGUARETÉ** con paleta *Electric Blue* (`#00D4FF`), negro profundo y acentos de color gamer.
* **Motor Nativo en Rust**: Versión oficial compilada en Rust con rendimiento de arranque inferior a 1 ms y consumo mínimo de memoria RAM (< 3 MB).
* **Extensión de VS Code Oficial**: Publicación de `yaguarete-language-support-0.1.1.vsix` con coloreado de sintaxis TextMate alineado con el tema Dark+, tema de iconos oficial y 6 proveedores de lenguaje IntelliSense.
* **Depurador DAP Nativo**: Servidor `yaguarete dap` con soporte completo para puntos de interrupción (*breakpoints*), avance paso a paso (*Step In*, *Step Over*, *Step Out*), inspección de variables locales y jerarquías de estructuras.
* **Asistente Gráfico Todo-en-Uno**: Lanzamiento del instalador nativo para Windows (`Instalador_Yaguarete.exe`) con detección automática de PATH, asociación de extensiones e instalación de extensión para VS Code.
* **Soporte Linux**: Paquetes binarios `.deb` y `.tar.gz` con script de instalación desatendida.
* **Licencia**: Publicado formalmente bajo licencia **GNU General Public License v3.0 or later (GPL-3.0-or-later)**.

---

## 🧪 Hitos de Estabilización y Pre-Releases hacia la v0.1.1

Los siguientes hitos documentan el proceso de migración, optimización y maduración arquitectónica que culminó en el lanzamiento de la versión 0.1.1:

### [v0.1.0-rc.3] - 2026-10-26
* **Trazabilidad Precisa de Errores**: Incorporación de reporte estricto de número de línea y columna (1-based) en todos los errores en tiempo de ejecución (`YaguareteError::Runtime`), indicando el punto exacto de la falla en la consola.
* **Sugerencias Ortográficas en Estructuras**: Si el usuario intenta acceder a un campo inexistente, el motor analiza los campos válidos de la estructura y ofrece sugerencias automáticas (ej. `¿Quisiste decir 'nombre'?`).
* **Optimización en Desreferenciación**: Prevención de fallas por puntero nulo emitiendo advertencias formales antes del acceso a memoria.

### [v0.1.0-rc.2] - 2026-10-23
* **Documentación Integral de Estructuras Dinámicas**: Creación de los módulos de referencia pedagógica `pilas.yaguarete` (LIFO con lista doblemente enlazada) y `colas.yaguarete` (FIFO con lista simplemente enlazada y doble puntero), ambos con complejidad temporal $\mathcal{O}(1)$.
* **Retorno de Estructuras**: Mejoras en el analizador para permitir que funciones y procedimientos retornen y reciban estructuras autorreferenciales complejas sin clonación innecesaria de memoria.

### [v0.1.0-rc.1] - 2026-10-23
* **Depurador DAP en Rust**: Implementación del servidor DAP (`crates/yaguarete_cli/src/dap.rs`) comunicándose por `stdin`/`stdout` con encabezados `Content-Length`.
* **Aislamiento Multihilo en DAP**: Ejecución del intérprete en un hilo secundario con pila expandida a 64 MB para evitar bloqueos del editor en recursión profunda.
* **Sistema de Auto-Actualización**: Subcomando `yaguarete update` con verificación contra GitHub Releases y caché local para optimizar peticiones de red.
* **IntelliSense Prioritario**: Proveedores de autocompletado en VS Code que priorizan miembros de estructuras al escribir `.` sobre identificadores genéricos.

### [v0.1.0-beta.1] - 2026-10-17
* **Migración Arquitectónica al Motor Rust (ADR-001)**: Reescritura del núcleo en Rust, reduciendo el tiempo de arranque de 185 ms a menos de 1 ms y acelerando Quicksort más de 70x.
* **Consola Interactiva REPL**: Implementación del comando `yaguarete repl` con buffer multilínea y metacomandos `:variables`, `:ayuda`, `:limpiar`, `:cancel`, `:salir`.
* **Subcomando `check` y `lsp-check`**: Verificación estática rápida y emisión de diagnósticos en formato JSON para editores.

### [v0.1.0-alpha.1] - 2026-10-17
* **Operadores `//` y `%` en Índices Matriciales**: Desambiguación léxica de `//` para permitir divisiones enteras dentro de índices como `matriz[p // columnas][p % columnas]` sin conflicto con comentarios de línea.
* **Iconos Multiplataforma**: Generación de activos de icono en formatos `.ico`, `.png` y `.svg` en resoluciones de hasta 1024x1024 píxeles.
