# 📜 Historial de Versiones (Changelog Oficial)

Este documento registra la evolución histórica del lenguaje **YAGUARETÉ**, comenzando por la versión oficial estable **0.1.1** y documentando los hitos de maduración y estabilización previos organizados como pre-releases semánticas (SemVer).

---

## 🌟 Versión Estable Oficial

### [v0.1.1] - 2026-10-30 (Primer Release Oficial)
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
