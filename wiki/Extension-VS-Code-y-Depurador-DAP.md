# 💻 Extensión Oficial para Visual Studio Code y Depurador DAP

La extensión oficial **YAGUARETÉ** (`yaguarete-language-support` v0.1.1) transforma a Visual Studio Code y Antigravity IDE en un entorno de desarrollo integrado (IDE) completo, moderno y optimizado para la enseñanza de la programación.

---

## 🎨 Identidad Visual y Coloreado de Sintaxis TextMate

La extensión implementa una gramática sintáctica formal TextMate (`syntaxes/yaguarete.tmLanguage.json`) alineada con la paleta de colores estándar de **VS Code Dark+**:

* **Estructuras**: `#4EC9B0` (Turquesa / Teal) para declaraciones de `ESTRUCTURA`.
* **Palabras de Control**: `#C586C0` (Púrpura / Magenta) para `INICIO`, `FIN`, `SI`, `SINO`, `PARA`, `MIENTRAS`, `HACER`, `RETORNAR`.
* **Tipos de Datos**: `#569CD6` (Azul Eléctrico) para `ENTERO`, `FLOTANTE`, `REAL`, `CADENA`, `CARACTER`, `BOOLEANO`, `ARCHIVO`, `VOID`, `NULO`.
* **Funciones y Procedimientos**: `#DCDCAA` (Amarillo Claro) para nombres de funciones, subrutinas y métodos.
* **Variables y Parámetros**: `#9CDCFE` (Celeste Claro).
* **Cadenas de Texto**: `#CE9178` (Naranja Cálido) para literales entre comillas.
* **Comentarios**: `#6A9955` (Verde Bosque) para notas de línea `//` y bloques `/* ... */`.

### 🐆 Tema de Iconos Oficial *"Jaguar Gamer"*
La extensión incluye el tema de iconos `yaguarete-icons` que asigna a todos los archivos con extensión `.yaguarete` el icono oficial de la cabeza del jaguar en alta resolución.

---

## 🧠 Los 6 Proveedores de IntelliSense

La extensión registra seis servicios de lenguaje nativos en [`extension.js`](file:///extension-vscode/extension.js) que se comunican con el motor de Rust:

1. **Autocompletado Contextual (`CompletionItemProvider`)**:
   * Priorización inteligente: Sugiere miembros de estructuras al escribir `objeto.` mostrando solo los campos válidos para esa instancia.
   * Sugerencia automática de funciones nativas (`MOD`, `ESPAR`, `ESIMPAR`, `ABRIR_ARCHIVO`) y palabras clave.
2. **Documentación Flotante (`HoverProvider`)**:
   * Al colocar el cursor sobre cualquier variable o llamada, despliega una tarjeta con su tipo formal, firma y comentarios de documentación precedentes.
3. **Ayuda de Parámetros (`SignatureHelpProvider`)**:
   * Al abrir un paréntesis `(` en una función o procedimiento, resalta en negrita el parámetro activo actual y describe su tipo.
4. **Ir a la Definición (`DefinitionProvider` - F12)**:
   * Permite saltar instantáneamente desde el uso de una función, procedimiento o estructura hasta su línea de declaración original en el workspace.
5. **Buscar Todas las Referencias (`ReferenceProvider` - Shift+F12)**:
   * Rastrea todas las apariciones y llamadas de un identificador a lo largo del archivo abierto.
6. **Esquema de Símbolos (`DocumentSymbolProvider`)**:
   * Alimenta el panel lateral **Outline** y la barra superior de navegación (*Breadcrumbs*), organizando jerárquicamente estructuras, funciones y procedimientos.

---

## ⚡ Diagnósticos en Vivo (Linter en Tiempo Real)

Al editar un archivo `.yaguarete`, la extensión ejecuta en segundo plano el comando nativo `yaguarete lsp-check --stdin --lenient` con una técnica de *debouncing* de 250 ms.

Los errores sintácticos o de tipado se subrayan en rojo inmediatamente en el editor antes de guardar o ejecutar el archivo, informando la causa exacta en la pestaña **Problems** de VS Code.

---

## 🐞 Depurador Nativo DAP (*Debug Adapter Protocol*)

YAGUARETÉ incluye una implementación completa del protocolo DAP estándar de la industria (`crates/yaguarete_cli/src/dap.rs`), permitiendo depuración gráfica interactiva sin herramientas externas.

### Inicio Rápido con F5 (Zero Configuration)
1. Abre cualquier archivo `.yaguarete` en VS Code.
2. Coloca puntos de interrupción (*breakpoints*) haciendo clic en el margen izquierdo de cualquier línea ejecutable (aparecerá un punto rojo).
3. Presiona **`F5`** (o selecciona el comando *"YAGUARETÉ: Depurar archivo actual"*).
4. El motor iniciará el proceso y se detendrá con precisión milimétrica en el primer punto de interrupción.

### Controles de Ejecución Soportados

| Tecla | Acción | Descripción |
| :---: | :--- | :--- |
| **`F5`** | Continuar (*Continue*) | Reanuda la ejecución del programa hasta alcanzar el próximo punto de interrupción. |
| **`F10`** | Paso por Encima (*Step Over*) | Ejecuta la línea actual y se detiene en la siguiente sentencia del mismo ámbito. |
| **`F11`** | Paso al Interior (*Step In*) | Desciende al interior del cuerpo de una `FUNCION` o `PROCEDIMIENTO` invocado. |
| **`Shift + F11`** | Paso hacia Afuera (*Step Out*) | Ejecuta el resto de la función actual y se detiene en la línea llamadora. |
| **`Shift + F5`** | Detener (*Stop*) | Aborta la sesión de depuración y finaliza el proceso de forma limpia. |

### Paneles de Inspección Disponibles

* **Pila de Llamadas (*Call Stack*)**: Muestra el marco `<principal>` y el anidamiento de subrutinas activas con sus números de línea.
* **Panel de Variables (*Variables Pane*)**:
  * Muestra las variables locales y sus valores en tiempo real.
  * Permite desplegar estructuras complejas y listas enlazadas en árbol hasta 6 niveles de profundidad para examinar punteros (`proximo_cliente`, `cabeza`, etc.).
* **Evaluación al Pasar el Cursor (*Hover Evaluation*)**: Al posar el mouse sobre una variable o expresión durante una pausa, se muestra su valor actual.
* **Consola de Depuración (*Debug Console*)**: Las salidas generadas por `IMPRIMIR(...)` se canalizan directamente a la consola de depuración sin interferir con el protocolo binario.

### Arquitectura Segura del Depurador
Para evitar desbordamientos de pila (*stack overflow*) al depurar algoritmos recursivos profundos, el depurador lanza el intérprete en un hilo independiente con una pila expandida a **64 Megabytes** (`std::thread::Builder`), garantizando estabilidad absoluta incluso en recorridos masivos.
