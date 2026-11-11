# Registro de Cambios (Changelog) - YAGUARETÉ

Todos los cambios notables en el lenguaje de programación y compilador **YAGUARETÉ** serán documentados en este archivo.
El formato se basa en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/) y este proyecto se adhiere a [Semantic Versioning](https://semver.org/lang/es/).

---

## [Unreleased]

---

## [0.2.0] - 2026-11-11

### 🚀 Rendimiento Extremo y Motor de Máquina Virtual (Bytecode VM)
- **Nuevo Motor de Máquina Virtual (`crates/yaguarete_core/src/vm/`):**
  - Implementación de un compilador de bytecode (`Compiler`) y una máquina virtual basada en pila (`VM`) con conjunto de instrucciones compacto (`OpCode`) y pool de constantes (`Chunk`).
  - Resolución de variables locales estática a slots indexados en tiempo de compilación O(1), eliminando búsquedas por cadena en tablas hash durante bucles intensivos.
  - Soporte de saltos directos (`Jump`, `JumpIfFalse`, `Loop`), llamadas y retornos de funciones con marcos de ejecución aislados (`CallFrame`), recursión y estructuras autorreferenciales.
  - Modificación e indexación in-situ sin clonado (`SetIndexLocal`, `SetIndexGlobal`, `GetIndexLocal`, `GetIndexGlobal`, `LengthLocal`, `LengthGlobal`), eliminando completamente la asignación masiva de memoria en operaciones sobre vectores.
  - Selección de motor en el CLI mediante el flag `--engine <vm|ast>` (habilitado `vm` por defecto).
- **Optimizaciones de Nivel 1 y 2 en el Intérprete AST (`eval.rs`):**
  - Adopción de `rustc-hash` (`FxHashMap`) para tablas de símbolos y asignación unificada de variables y tipos en un único `VariableEntry`.
  - Reutilización de ámbitos en memoria mediante pool interno (`scope_pool`) para evitar reasignaciones en el heap durante iteraciones de bucles `PARA`, `MIENTRAS` y `HACER MIENTRAS`.
  - Atajos (*fast-paths*) para mutación directa en memoria en operadores de asignación (`+=`, `-=`) e incremento/decremento (`i++`, `i--`).
  - Evaluación de condiciones relacionales en registros de CPU sin creación de instancias intermedias de `Value::Booleano`.
- **Resultados de Benchmark (Selection Sort de 10.000 elementos, ~50 millones de iteraciones):**
  - Baseline original: `39.459,87 ms` (~39,5 segundos).
  - Intérprete AST optimizado: `9.068,74 ms` (~9,0 segundos) — **4,35x más rápido**.
  - Máquina Virtual Bytecode (`--engine vm`): `6.901,29 ms` (~6,9 segundos) — **5,72x más rápido** (**82,5% de reducción del tiempo de ejecución total**).

### ⚡ Motor de Ejecución y Lenguaje (`yaguarete_core`)
- **Operador de potencia `^`** y funciones matemáticas incorporadas: `POTENCIA(base, exponente)`, `RAIZCUADRADA(numero)`, `ABS(numero)` y `ALEATORIO(min, max)`; constante `PI`.
- **Entrada:** `LEER()` / `LEER(mensaje)` con autoparseo (`BOOLEANO`, `ENTERO`, `REAL` o `CADENA`); el mensaje se imprime sin salto de línea. No disponible en el depurador DAP.
- **Cadenas:** `MAYUSCULAS(texto)`, `MINUSCULAS(texto)`, `RECORTAR(texto)` y `DIVIDIR_CADENA(texto, separador): CADENA[]` (separador no vacío).
- **Conversión:** `A_ENTERO`, `A_REAL`, `A_FLOTANTE`, `A_CADENA`, `A_BOOLEANO`.
- **Matemáticas:** `MIN(a, b)`, `MAX(a, b)` y `REDONDEAR(numero)` / `REDONDEAR(numero, decimales)` (half-away-from-zero: `2.5` → `3`, `-2.5` → `-3`).

### 🎨 Extensión de VS Code
- Autocompletado, hover y exclusión de "ir a definición" para todas las funciones nuevas; `LEER` ahora se completa como `LEER(mensaje)`.
- Ayuda de firma (*signature help*) para todas las built-ins (nuevas y existentes: `RAIZCUADRADA`, `ABS`, `POTENCIA`, `ALEATORIO`, `MOD`, `ESPAR`, `ESIMPAR`, `ABRIR_ARCHIVO`).
- Gramática TextMate: nuevos grupos `support.function.builtin.string` y `support.function.builtin.conversion`; `MIN`, `MAX`, `REDONDEAR` en `math`; los nombres built-in ya no se confunden con tipos de estructura.

### 📚 Documentación
- `README.md` (sección Built-ins) y `estructuras.txt` actualizados con las nuevas funciones.

---

## [0.1.2] - 2026-11-05

### ⚡ Motor de Ejecución y Lenguaje (`yaguarete_core`)
- **Evaluación en Cortocircuito para Operadores Lógicos (`Y`, `O`):**
  - Implementación de evaluación en cortocircuito (*short-circuit evaluation*) en el evaluador de expresiones binarias (`eval.rs`).
  - Para el operador `Y`: si el operando izquierdo es falso, el operando derecho no se evalúa, retornando `FALSO` de forma inmediata.
  - Para el operador `O`: si el operando izquierdo es verdadero, el operando derecho no se evalúa, retornando `VERDADERO` de forma inmediata.
  - Habilita patrones idiomáticos de guarda y comprobación segura en algoritmos (evita desbordamientos de índice como `MIENTRAS(j >= 0 Y vector[j] > key)` y división por cero o errores en evaluación de punteros/nulos).
  - Incorporación de tests unitarios de regresión en `crates/yaguarete_core/tests/test_operators.rs` (`test_cortocircuito_operador_y` y `test_cortocircuito_operador_o`).

### ⚙️ Sincronización Integral de Versiones a v0.1.2
- Sincronización de versión a `0.1.2` en todos los crates de Rust (`crates/yaguarete_core`, `crates/yaguarete_cli`, `crates/yaguarete_installer`).
- Actualización de metadatos de versión en cabecera PE de Windows (`FileVersion` y `ProductVersion` a `0.1.2.0`) en `build.rs` y archivos descriptores.
- Actualización de tests del CLI en `crates/yaguarete_cli/tests/test_cli.rs` para validar el banner `YAGUARETÉ (v0.1.2)`.

---

## [0.1.1] - 2026-10-30

### 🐆 Primer Release Oficial de YAGUARETÉ: Identidad "Jaguar Gamer" & Electric Blue
- **Transición desde el Prototipo UCABA hacia la Identidad Oficial `YAGUARETÉ` (`.yaguarete`):**
  - Consolidación del lenguaje bajo la identidad oficial **YAGUARETÉ** (`v0.1.1`) como evolución directa a partir del prototipo inicial UCABA, adoptando la extensión obligatoria `.yaguarete`, el comando `yaguarete` y el paquete `yaguarete-language-support`.
- **Nuevo Isotipo Oficial "Jaguar Gamer":**
  - Implementación del nuevo isotipo oficial de YAGUARETÉ con diseño vectorial afilado estilo *esports* / *gaming*, proyectando velocidad, agilidad y alto rendimiento.
  - Abandono definitivo del magenta institucional clásico en favor de una paleta cibernética de alto impacto: **Electric Blue & Cyber Cyan** (`#0066FF`, `#00D4FF`, `#38BDF8`).
- **Modernización de Iconos de Windows y Assets de Alta Definición:**
  - Nuevos binarios `.ico` multirresolución de alta definición (`yaguarete_icon.ico` y `yaguarete_icon_standard.ico`) con capas optimizadas para pantallas High-DPI desde 16x16 hasta 256x256 píxeles.
  - Actualización del asset `yaguarete_logo_header.png` y cabeceras gráficas con la estética Jaguar Gamer Electric Blue.

### 🖼️ Modernización del Instalador Gráfico (`installer_gui.py`)
- **Interfaz Gráfica Renovada en Identidad YAGUARETÉ v0.1.1:**
  - Rediseño completo de la interfaz visual del instalador gráfico en Python (`installer_gui.py`) y Rust (`crates/yaguarete_installer`).
  - Adopción de la paleta Dark Electric Blue (`#080c14` de fondo base, `#0f172a` en paneles, `#0066ff` en acento primario y `#00d4ff` en detalles neón).
  - Componentes estéticos modernizados: badges estilizadas para autoría y versión, botones redondeados de alta visibilidad con efectos hover fluidos, y consola de registro integrada con resaltado cyan neón.

### 🧩 Actualización del Tema de Iconos y Extensión de VS Code
- **Regeneración de Iconos SVG y PNG:**
  - Regeneración completa de los iconos en `extension-vscode/icons/` (`yaguarete-file.svg`, `yaguarete-file.png`) adaptados al nuevo isotipo Jaguar Gamer con estética azul eléctrica.
  - Actualización del tema de iconos `yaguarete-icon-theme.json` e icono oficial de la extensión (`extension-vscode/icon.png`).
- **Empaquetado y Distribución VSIX v0.1.1:**
  - Generación y empaquetado del nuevo archivo oficial `extension-vscode/yaguarete-language-support-0.1.1.vsix`.
  - Actualización de metadatos, descripciones y configuración de empaquetado en `extension-vscode/package.json`.

### ⚙️ Sincronización Integral de Versiones y Toolchain Rust
- **Crates de Rust y Binarios Autónomos:**
  - Sincronización rigurosa de versión `0.1.1` en `crates/yaguarete_core/Cargo.toml`, `crates/yaguarete_cli/Cargo.toml` y `crates/yaguarete_installer/Cargo.toml`.
  - Actualización de la constante global `VERSION` a `0.1.1` en `crates/yaguarete_core/src/lib.rs`.
  - Actualización de metadatos PE de Windows (FileVersion y ProductVersion a `0.1.1.0`) en `crates/yaguarete_cli/build.rs`, `crates/yaguarete_installer/build.rs`, `version_info_installer.txt` y `version_info_yaguarete.txt`.
  - Actualización de tests de integración del CLI en `crates/yaguarete_cli/tests/test_cli.rs` para validar el nuevo banner `YAGUARETÉ (v0.1.1)`.
  - Regeneración y validación de `Cargo.lock` para garantizar compilación y comprobación limpia en todo el workspace.
- **Herramientas Auxiliares de Empaquetado:**
  - Sincronización de versión `0.1.1` en scripts de empaquetado y distribución (`build_linux_package.py`, `install_extension.py`, `pseudocode/__init__.py`).

## [0.1.0-rc.3] - 2026-10-26

### ⚙️ Motor Nativo Rust (`yaguarete_core` / `crates/yaguarete_core`)
- **Trazabilidad Precisa de Errores en Tiempo de Ejecución (Línea y Columna):**
  - Se extendió el nodo `Statement::Located` del AST para almacenar la columna (`column: usize`) además de la línea de origen del código fuente.
  - El analizador sintáctico (`parser.rs`) ahora propaga línea y columna exactas para todas las sentencias y sentencias de actualización en bucles `PARA`.
  - El intérprete (`eval.rs`) ahora mantiene `current_column` y rastrea con precisión la ubicación del error en desbordamientos de índices de vectores/matrices, accesos inválidos a miembros de estructuras, llamadas a métodos o funciones no compatibles, y operaciones aritméticas inválidas o división por cero.
  - Se añadieron `call_line` y `call_column` al marco de llamadas (`CallFrame`) para preservar y restaurar la ubicación original al retornar o fallar dentro de funciones invocadas.
  - Se corrigió el reporte que anteriormente mostraba `Error de ejecución en línea 0, columna 0`, mostrando ahora la línea y columna exactas donde se originó el fallo.
  - Incorporación de test de regresión automatizado `test_runtime_error_line_and_column` en `crates/yaguarete_core/tests/test_engine.rs`.

### 🧩 Extensión y Empaquetado Oficial (v0.1.0-rc.3)
- Empaquetado y actualización del paquete VSIX pre-release: `extension-vscode/yaguarete-language-support-0.1.0-rc.3.vsix`.
- Sincronización de metadatos de versión `0.1.0-rc.3` en todo el workspace (`crates/yaguarete_core`, `crates/yaguarete_cli`, `crates/yaguarete_installer`, `extension-vscode/package.json`, instaladores y scripts auxiliares).

## [0.1.0-rc.2] - 2026-10-23

### 📚 Documentación Integral y Mejoras en Estructuras de Datos (`pilas.yaguarete` y `colas.yaguarete`)
- **Documentación Paso a Paso de Algoritmos Clave:**
  - Explicación exhaustiva, detallada y estructurada para cada paso de las operaciones de pilas y colas:
    - `agregar_elemento_pila`: Algoritmo *push* sobre lista doblemente enlazada (LIFO), detallando enlace bidireccional y actualización de cima y base en tiempo constante $O(1)$.
    - `sacar_elemento_pila`: Algoritmo *pop* con desconexión completa y aislamiento de punteros del nodo extraído.
    - `agregar_elemento_cola`: Algoritmo *enqueue* sobre lista simplemente enlazada (FIFO), asegurando que el nuevo nodo sea terminal y actualizando extremos en $O(1)$.
    - `sacar_elemento_cola`: Algoritmo *dequeue* con extracción del frente y sincronización de final.
  - Documentación de cabecera en todas las funciones y estructuras secundarias: constructores (`instanciar_nodo_cliente`, `instanciar_pila`, `instanciar_cola`), utilitarios de inserción por lote (`agregar_elementos_pila`, `agregar_elementos_cola`) y rutinas de diagnóstico (`mostrar_info_pila`, `mostrar_info_cola`).
- **Corrección de Caso Límite Crítico en `sacar_elemento_pila`:**
  - Se corrigió el error en tiempo de ejecución al desapilar el último nodo restante: la asignación `pila.procesar_primero.anterior_cliente = NULO` se ejecutaba cuando `procesar_primero` ya era `NULO`. Ahora se valida condicionalmente `SI (pila.procesar_primero != NULO)` y se sincroniza adecuadamente `pila.procesar_ultimo = NULO`.
- **Blindaje y Aislamiento de Memoria:**
  - Validación defensiva de punteros contra referencias nulas (`nuevo_nodo == NULO`) en `agregar_elemento_pila` y `agregar_elemento_cola`.
  - Eliminación de instanciaciones redundantes de nodos auxiliares antes de verificar si la estructura estaba vacía, optimizando el uso de memoria.

### ⚙️ Motor Nativo Rust (`yaguarete_core` / `crates/yaguarete_core`)
- **Corrección de Sintaxis de Retorno de Estructuras en Funciones (`parser.rs`):**
  - Se corrigió la regla sintáctica en `parse_function` para reconocer identificadores de estructuras de usuario como tipos de retorno válidos (`FUNCION f(): MiEstructura`) en la misma línea aun cuando la siguiente línea comience con comentarios o bloques no declarativos (`SI`, `MIENTRAS`, etc.).
  - Incorporación de test de regresión automatizado en `crates/yaguarete_core/tests/test_estructuras.rs`.

### 🧩 Extensión y Empaquetado Oficial (v0.1.0-rc.2)
- Compilación y actualización del paquete VSIX pre-release: `extension-vscode/yaguarete-language-support-0.1.0-rc.2.vsix`.
- Actualización de metadatos de versión en `crates/yaguarete_core`, `crates/yaguarete_cli`, `crates/yaguarete_installer`, `extension-vscode/package.json` y scripts de empaquetado.

## [0.1.0-rc.1] - 2026-10-23

### 🧩 Extensión Oficial de Visual Studio Code y Antigravity IDE (v0.1.0-rc.1)
- **Autocompletado Inteligente de Funciones y Procedimientos (IntelliSense de Alta Prioridad):**
  - Al comenzar a escribir el nombre de cualquier función o procedimiento (`FUNCION` o `PROCEDIMIENTO`), la extensión muestra inmediatamente el nombre completo de la función en la lista de sugerencias con icono de función/método, firma de parámetros y tipo de retorno (ej. `(cola: Cola, cantidad: ENTERO)`), idéntico al comportamiento de autocompletado de Python en VS Code.
  - Ordenamiento prioritario (`sortText = '0_' + fn.name`) para garantizar que las funciones de usuario aparezcan en el primer lugar de sugerencias de autocompletado frente a snippets y palabras clave.
  - Recolección multi-módulo: indexación y autocompletado de funciones y procedimientos tanto del archivo activo como de todos los documentos `.yaguarete` abiertos en el workspace.
- **Autocompletado de Variables y Parámetros en Ámbito:**
  - Inclusión automática en IntelliSense de variables locales, parámetros de funciones y variables de iteración (`PARA`) declaradas en el ámbito actual (`sortText = '1_' + var.name`).
- **Soporte de Ayuda de Parámetros en Tiempo Real (`SignatureHelpProvider`):**
  - Registro de proveedor de ayuda de firma activado por los caracteres `(` y `,`. Al escribir argumentos de una función o procedimiento, se despliega dinámicamente la firma completa resaltando el parámetro actual.
- **Robustecimiento del Analizador de Cabeceras de Función (`FN_HEADER_RE`):**
  - Soporte completo para tipos de retorno de estructuras personalizadas y arreglos (`[A-Za-z_][A-Za-z0-9_]*(?:\s*\[\s*\])*`), permitiendo tipos alfanuméricos con guiones bajos y dimensiones como `NodoCliente` y `TIPO[]`.
- **Coloreado de Sintaxis TextMate para Estructuras en Parámetros y Retornos:**
  - Actualización de `yaguarete.tmLanguage.json` para reconocer y resaltar con la clase de token de estructura (`entity.name.type.struct.yaguarete`, color verde menta/turquesa) los tipos de dato estructura tanto en los parámetros de funciones y procedimientos (`nodo_pila: Pila`) como en los tipos de retorno (`instanciar_nodo_cliente(): NodoCliente`), preservando al mismo tiempo el coloreado azul distintivo de los tipos primitivos (`ENTERO`, `CADENA`, etc.).
  - Soporte ampliado en declaraciones de variables con dimensiones de vectores/matrices de estructuras (`Pila[]`, `NodoCliente[10]`).
- **Nuevo Paquete VSIX Pre-release v0.1.0-rc.1:**
  - Compilación y empaquetado de `yaguarete-language-support-0.1.0-rc.1.vsix` e incrustación en el instalador gráfico nativo de Windows.

### 🔄 Verificación y Notificación Automática de Actualizaciones en CLI Nativo (Rust)
- **Comprobación Real contra GitHub Releases (`updater.rs`):**
  - Implementación en Rust de la consulta directa a la API de GitHub (`/repos/caidevOficial/Yaguarete_Lang/releases/latest`) para comparar versiones semánticas reales.
  - Caché local de 1 hora en `%LOCALAPPDATA%\Yaguarete\version_cache.json` compatible con versiones históricas.
  - Subcomando `yaguarete update --check` funcional: detecta con precisión si hay una versión superior disponible en GitHub Releases.
  - Comprobación en segundo plano no bloqueante al ejecutar archivos (`run`, `check`) y al iniciar la consola interactiva REPL, emitiendo avisos con estilo pip cuando hay actualizaciones.
  - Subcomando `yaguarete update` / `upgrade`: descarga y ejecución automática del asistente oficial de instalación (`Instalador_Yaguarete.exe`).

### 🐞 Depurador Nativo Interactivo DAP (*Debug Adapter Protocol*)
- **Soporte de Breakpoints en VS Code y Antigravity:**
  - Registro de capacidades de depuración en `extension-vscode`: puntos de interrupción en margen izquierdo (`breakpoints`), configuración automática de lanzamiento (`initialConfigurations`) y comando directo `yaguarete.debugFile`.
  - Servidor de depuración nativo en Rust invocado con `yaguarete dap`, comunicándose por `stdin`/`stdout` mediante el protocolo estándar DAP (*Debug Adapter Protocol*).
- **Control de Ejecución Paso a Paso:**
  - Soporte de acciones: *Continuar* (`continue`), *Paso por procedimientos* (`next` / step over), *Paso por instrucciones* (`stepIn`) y *Paso para salir* (`stepOut`), gestionando la profundidad de llamadas en funciones y procedimientos.
  - Parada opcional en el inicio (`stopOnEntry`) y pausa interactiva (`pause`).
- **Inspección de Pila de Llamadas y Variables:**
  - Desglose jerárquico de ámbitos y marcos de ejecución (*Call Stack*).
  - Inspección en vivo de variables locales, arreglos, matrices y campos de estructuras de datos personalizadas con protección contra ciclos y recursión infinita.
  - Evaluación de expresiones en consola de depuración y hovers interactivos (`x`, `p.campo`, `v[0]`).
- **Anotación de Líneas en AST (`Statement::Located`):**
  - Envoltorio de sentencias con número de línea original preservado sin alterar el rendimiento ni la compatibilidad del intérprete central.
  - Redirección transparente de `IMPRIMIR` mediante `print_sink` a eventos `output` de depuración sin contaminar el canal del protocolo.

## [0.1.0-beta.1] - 2026-10-17

### 🚀 Nuevo Motor Nativo en Rust (Workspace Cargo)
- **Migración Arquitectónica Completa:** Creación de un workspace en Rust compuesto por:
  - `crates/yaguarete_core`: Motor central que contiene el lexer (`lexer.rs`), AST tipado (`ast.rs`), parser recursivo descendente (`parser.rs`), comprobación estricta de tipos (`eval.rs`), e intérprete de ejecución directa sin runtime externo.
  - `crates/yaguarete_cli`: CLI nativo multiplataforma (`yaguarete`) con subcomandos `run`, `repl`, `check`, `lsp-check` y flags de tiempo `--time` y `--time-full`.
  - `crates/yaguarete_installer`: Asistente GUI autónomo para Windows desarrollado con `eframe`/`egui` y soporte High-DPI.
- **Rendimiento Revolucionario:**
  - Tiempo de arranque en frío (*Cold Start*): **<1 ms** (~250x más rápido).
  - Ejecución de algoritmos: QuickSort ordenando arreglos en **1 ms** frente a **3.240 ms** en Python (~300x más rápido).
  - Huella de memoria mínima: **< 5 MB RAM** (90% menor consumo).
  - Binario autónomo: **0.83 MB** sin DLLs ni runtime externo (90% más liviano que los 7.8 MB previos).

### 💻 Consola Interactiva REPL (`yaguarete repl`)
- **Evaluación Didáctica en Tiempo Real:** Subcomando `yaguarete repl` (y ejecución por defecto al invocar `yaguarete` sin argumentos) con evaluación de expresiones instantánea (`=> <valor>`).
- **Buffer Multilínea Inteligente:** Función `is_input_complete` con stack de bloques léxicos para admitir sentencias multilínea (`FUNCION`, `SI-SINO`, bucles) y prompt alternado (`yaguarete> ` / `...    `).
- **Metacomandos:** `:ayuda`, `:variables` (tabla ASCII estilizada con anchos dinámicos), `:limpiar`, `:cancel` y `:salir`.

### 🧩 Extensión Oficial de Visual Studio Code y Antigravity IDE (v0.1.0-beta.1)
- **Diagnósticos Sub-milisegundo:** Consumo directo del comando `yaguarete lsp-check --json --stdin`, eliminando toda dependencia de Python para el linter en tiempo real.
- **Comando REPL Integrado:** Nuevo comando `YAGUARETÉ: Iniciar consola interactiva REPL` (`yaguarete.startRepl`) accesible desde paleta de comandos y botones de navegación.
- **Detección Automática de Binarios:** Detección de binarios en workspace local (`target/release/yaguarete`), `%LOCALAPPDATA%` en Windows y `/usr/local/bin` en Linux/macOS.
- **Resaltado de Sintaxis Completo:** Gramática TextMate actualizada para reconocer tipos personalizados de `ESTRUCTURA` (`entity.name.type.struct.yaguarete`), operadores `++` y `--`, palabras reservadas y literales.
- **Inspección y Hover Sin Fallos:** Reparación del parser de contexto léxico en `isInCommentOrString` admitiendo comentarios estilo `#`, habilitando la inspección hover de variables y funciones en cualquier contexto de código.
- **Tema de Iconos Integrado:** Nuevo tema de iconos oficial (`yaguarete-icon-theme.json`) aportado vía `iconThemes` y activación por workspace (`workspaceContains:**/*.yaguarete`), dotando a los módulos `.yaguarete` de su icono en el árbol de archivos.
- **Nuevo Paquete VSIX y Despliegue Robusto:** Publicación de `yaguarete-language-support-0.1.0-beta.1.vsix` embebido en el instalador; actualización de `install_extension.py` para depurar `.obsolete` y registrar formalmente la extensión en `extensions.json` en VS Code, Cursor y Antigravity IDE.

### 🎨 Iconos y Asociación de Archivos Multiplataforma
- **Windows:** Registro dual de ProgIDs (`YaguareteFile` y `Yaguarete.Program`) con clave `DefaultIcon`, sincronización con `OpenWithProgids` en el registro de Windows y emisión de `SHCNE_ASSOCCHANGED` para refresco instantáneo del explorador.
- **Linux:** Adición de iconos MIME en `icons/hicolor/256x256/mimetypes/` (`application-x-yaguarete.png` y `text-x-yaguarete.png`) en paquetes `.deb` y script `install.sh`.
- **Publicación Pre-release:** Automatización de GitHub Actions configurada expresamente para publicar la release `v0.1.0-beta.1` como pre-release (`prerelease: true`).

### 🧪 Suite Exhaustiva de Paridad de Pruebas (72 Tests en Rust)
- **100% de Paridad Certificada:** Porting y ampliación de todos los casos de prueba de Python a suites nativas en Rust (`test_engine.rs`, `test_operators.rs`, `test_estructuras.rs`, `test_interpreter.rs`, `test_cli.rs`, `yaguarete_installer::tests`).
- **Verificación Estricta de Tipos:** Validación estática en tiempo de ejecución de asignaciones y retornos, precisión simple de 32 bits para `FLOTANTE` vs doble de 64 bits para `REAL`.
- **Auditoría de Código:** `cargo clippy --workspace -- -D warnings` con **0 advertencias**.

### 🖼️ Asistente de Instalación Gráfico Nativo (`Instalador_Yaguarete.exe`)
- **GUI Moderna y Autónoma en Rust:** Reemplazo completo de la interfaz basada en Tkinter/PyInstaller por una aplicación nativa escrita en Rust (`eframe`/`egui`).
- **Reducción de Tamaño Masiva:** El ejecutable del instalador pasa de ~18 MB a **5.05 MB** (72% más liviano) incluyendo todos los recursos embebidos en el binario: `yaguarete.exe` compilado, `yaguarete_icon.ico` y la extensión VSIX.
- **Migración Fluida y Resiliente desde Python/PyInstaller:** Ruptura de herencia de entorno y handles (`_MEIPASS2`) para evitar avisos de borrado de temporales, manejo resiliente contra bloqueos de archivos en Windows (`os error 32`) con reintentos y reemplazo atómico (*Move-then-Replace*), y purga automática de carpetas temporales residuales `_MEI*` en `%TEMP%`.
- **Instalación Desatendida:** Soporte para flag `--silent` para automatización en entornos de laboratorio o CI/CD.
- **Metadatos e Iconos Nativos Windows PE:** Integración con `winres`, incrustando el icono institucional y metadatos de versión en el recurso PE de Windows.

---

## [0.1.0-alpha.1] - 2026-10-17

### 📖 Documentación y Gramática Base del Lenguaje
- **Operadores aritméticos (`//` y `%`)**:
  - Soporte completo del operador infijo de **división entera (`//`)** y **módulo (`%`)** en el analizador léxico, sintáctico, verificador de tipos e intérprete, retornando siempre `ENTERO` con control de división por cero.
  - Desambiguación léxica inteligente: expresiones como `matriz[p // columnas][p % columnas] = valor` o `v[n // 2]` conviven de forma transparente con los comentarios de línea `//` y `#`.
- **Estructuras de Datos Heterogéneas (`ESTRUCTURA ... FIN ESTRUCTURA`) y Tipo `NULO`**:
  - Definición de tipos compuestos por el usuario con múltiples atributos tipados, vectores/matrices de estructuras (`Libro estante[10]`) y serialización directa.
  - Soporte de estructuras autorreferenciales (nodos para listas enlazadas, pilas y colas) y literal universal `NULO` (`null` / `None`) con protección contra referencias nulas.
- **Rendimiento y Optimización de Algoritmos**:
  - Implementación de Quicksort ($O(N \log N)$) con selección de pivote central y partición balanceada de 3 vías (menores, iguales y mayores).
- **Medición de Tiempo en CLI (`--time`, `--time ms`, `--time full`, `--time-full`)**:
  - Desglose con precisión en horas, minutos, segundos y milisegundos con concordancia singular/plural, admitiendo `--time` en cualquier posición de la línea de comandos.
- **Soporte Oficial y Empaquetado para GNU/Linux**:
  - Generación de paquete nativo Debian/Ubuntu (`.deb`) con registro MIME `application/x-yaguarete` y tarball portable universal (`.tar.gz`) con scripts `install.sh` y `uninstall.sh`.
- **Autocompletado por Punto (`.`) e Iconos de Archivos `.yaguarete`**:
  - Despliegue de atributos de estructuras y métodos de `ARCHIVO` (`LEER_LINEA()`, `ESCRIBIR_LINEA()`, `CERRAR_ARCHIVO()`, `findearchivo`) al escribir `.` en VS Code / Antigravity / Cursor.
  - Asociación de iconos `.yaguarete` en Windows (`SHChangeNotify`), Linux (`gtk-update-icon-cache`) y editores compatibles.
