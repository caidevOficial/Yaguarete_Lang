# 🖥️ Interfaz de Línea de Comandos (`yaguarete`)

El compilador e intérprete de **YAGUARETÉ** se distribuye como un ejecutable binario único de alto rendimiento llamado `yaguarete` (o `yaguarete.exe` en Windows), construido sobre la biblioteca `clap` de Rust.

---

## 📖 Sintaxis General

```bash
yaguarete [OPCIONES] [SUBCOMANDO] [ARCHIVO.yaguarete]
```

Si se invoca sin argumentos ni subcomandos, inicia automáticamente la [[Consola Interactiva REPL|Consola-Interactiva-REPL]].

---

## 🛠️ Subcomandos Disponibles

### 1. `run` (Ejecución de Programas)
Ejecuta un archivo fuente con extensión obligatoria `.yaguarete`:

```bash
yaguarete run mi_programa.yaguarete
# También admite ejecución directa sin la palabra 'run':
yaguarete mi_programa.yaguarete
```

#### Medición de Tiempos de Ejecución:
* **`--time`**: Muestra el tiempo transcurrido en milisegundos de alta precisión:
  ```bash
  yaguarete run algoritmo.yaguarete --time
  # Salida: [Tiempo total: 1.42 ms]
  ```
* **`--time-full`**: Muestra el desglose temporal legible en horas, minutos, segundos y milisegundos:
  ```bash
  yaguarete run ordenamiento.yaguarete --time-full
  # Salida: [Tiempo total: 0 Horas, 0 Minutos, 0 Segundos, 1 Milisegundo]
  ```

#### Modo Permisivo (`--lenient` o `-l`):
Habilita el analizador flexible para pseudocódigo informal, permitiendo omitir bloques `INICIO/FIN` estrictos o aceptar puntos y comas opcionales:
```bash
yaguarete run borrador.yaguarete --lenient
```

---

### 2. `check` (Verificación Estática de Sintaxis y Tipos)
Analiza el archivo fuente, construye el Árbol de Sintaxis Abstracta (AST) y valida los tipos de datos **sin ejecutar** el programa:

```bash
yaguarete check codigo.yaguarete
```

* **Sin errores**: Retorna código de salida `0` y el mensaje:
  ```text
  [OK] Sin errores de sintaxis ni tipos detectados.
  ```
* **Con errores**: Retorna código de salida `1` y describe la causa exacta con indicación de línea y columna. Ideal para hooks de Git pre-commit y pipelines de CI/CD.

---

### 3. `lsp-check` (Diagnósticos Estructurados en JSON)
Diseñado para la integración con servidores de lenguaje (LSP), plugins de editores e IDEs (VS Code, Antigravity, Sublime Text, Neovim):

```bash
# Lee el código fuente directamente desde stdin
cat script.yaguarete | yaguarete lsp-check --stdin --lenient
```

Emite un informe JSON directamente en `stdout`:
```json
{
  "diagnostics": [
    {
      "line": 14,
      "character": 9,
      "length": 1,
      "message": "Se esperaba ';' o ':' tras condición del bucle 'PARA'",
      "severity": "error",
      "source": "yaguarete-parser"
    }
  ]
}
```

---

### 4. `dap` (Servidor de Depuración Nativo)
Inicia el servidor DAP (*Debug Adapter Protocol*) que se comunica a través de `stdin`/`stdout` mediante tramas HTTP-like:

```bash
yaguarete dap
```
Este comando es invocado automáticamente por la extensión oficial de Visual Studio Code al presionar `F5`.

---

### 5. `update` / `upgrade` (Gestión de Actualizaciones)
Consulta el repositorio oficial en GitHub Releases para comprobar si existen nuevas versiones:

```bash
# Solo comprobar si hay actualización disponible sin instalar:
yaguarete update --check

# Descargar e iniciar automáticamente el instalador oficial:
yaguarete update
```

El cliente cuenta con un sistema de caché de 1 hora en `%LOCALAPPDATA%\Yaguarete\version_cache.json` para no sobrecargar el límite de peticiones de GitHub API.

---

## 🌐 Variables de Entorno del Sistema

Puedes personalizar el comportamiento de la CLI mediante las siguientes variables de entorno:

| Variable | Valor | Efecto |
| :--- | :---: | :--- |
| **`YAGUARETE_HIDE_BANNER`** | `1` | Suprime el encabezado gráfico del jaguar al iniciar la consola o ejecutar scripts. |
| **`YAGUARETE_NO_UPDATE_CHECK`** | `1` | Deshabilita la verificación automática en segundo plano de nuevas versiones. |
| **`NO_COLOR`** o **`YAGUARETE_NO_COLOR`** | `1` | Desactiva las secuencias de escape de color ANSI (salida en texto monocromático). |

---

## 🚦 Códigos de Salida del Proceso (*Exit Codes*)

* **`0`**: Ejecución o verificación completada exitosamente sin anomalías.
* **`1`**: Error léxico, de análisis sintáctico, de tipo o error durante la ejecución en tiempo de corrida.
