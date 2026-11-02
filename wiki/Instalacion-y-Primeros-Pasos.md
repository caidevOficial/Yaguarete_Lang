# 🚀 Instalación y Primeros Pasos

En esta guía aprenderás a instalar el entorno oficial de ejecución de **YAGUARETÉ v0.1.1** en Windows y Linux, verificar la configuración del sistema y configurar tu editor de código.

---

## 💻 Requisitos del Sistema

* **Windows**: Windows 10 o Windows 11 (arquitectura de 64 bits `x86_64`). No requiere permisos de administrador para la instalación en el perfil de usuario.
* **Linux**: Distribuciones modernas con kernel Linux 4.x o superior y `glibc >= 2.31` (Ubuntu 20.04+, Debian 11+, Fedora 36+, Arch Linux).
* **Espacio en Disco**: Menos de 20 MB para el runtime nativo completo.
* **Opcional**: [Visual Studio Code](https://code.visualstudio.com/) o Antigravity IDE para una experiencia integrada con depuración gráfica y coloreado de sintaxis.

---

## 🪟 Instalación en Windows

YAGUARETÉ cuenta con un instalador integral con interfaz gráfica nativa diseñada con la identidad visual *"Jaguar Gamer" & Electric Blue*.

### Opción 1: Asistente Gráfico Todo-en-Uno (Recomendado)

1. Descarga el archivo ejecutable [`Instalador_Yaguarete.exe`](https://github.com/caidevOficial/Yaguarete_Lang/releases/tag/v0.1.1) desde los Releases oficiales de GitHub.
2. Ejecuta el archivo con doble clic. Se abrirá el asistente visual de instalación.
3. El instalador realizará automáticamente las siguientes acciones:
   * Desempaqueta el motor nativo `yaguarete.exe` en `%LOCALAPPDATA%\Programs\Yaguarete\bin`.
   * Registra automáticamente el directorio en la variable de entorno `PATH` del usuario actual.
   * Asocia la extensión de archivo `.yaguarete` con el icono oficial del jaguar y la acción de ejecución predeterminada.
   * Si detecta una instalación de Visual Studio Code o Antigravity IDE, registra la extensión oficial `yaguarete-language-support-0.1.1.vsix`.
   * Crea un acceso directo en el Menú Inicio para abrir la consola REPL directamente.

### Opción 2: Instalación Desatendida / Silenciosa (Para Laboratorios y Servidores)

Para desplegar YAGUARETÉ en múltiples computadoras de un laboratorio universitario o en scripts automatizados de CI/CD:

```powershell
.\Instalador_Yaguarete.exe --silent
```

### Verificación en Windows

Abre una nueva ventana de PowerShell o Símbolo del Sistema (`cmd.exe`) y escribe:

```powershell
yaguarete --version
```

Deberás ver la salida:
```text
YAGUARETÉ v0.1.1 (Native Rust Engine)
```

Si el comando no es reconocido, asegúrate de reiniciar tu terminal para que se carguen las nuevas variables de entorno de tu usuario.

---

## 🐧 Instalación en Linux

### Opción 1: Paquete Debian / Ubuntu (`.deb`)

Descarga el paquete `yaguarete-0.1.1-amd64.deb` y ejecútalo mediante `dpkg`:

```bash
sudo dpkg -i yaguarete-0.1.1-amd64.deb
```

El binario quedará disponible globalmente en `/usr/local/bin/yaguarete`.

### Opción 2: Script Automatizado `install.sh`

Si descargas el archivo comprimido `.tar.gz`:

```bash
tar -xzf yaguarete-0.1.1-linux-x86_64.tar.gz
cd yaguarete-0.1.1-linux-x86_64
chmod +x install.sh
./install.sh
```

El script verificará permisos, copiará los binarios a `~/.local/bin` o `/usr/local/bin` y añadirá la ruta a tu archivo `~/.bashrc` o `~/.zshrc`.

---

## 🧩 Instalación de la Extensión para Visual Studio Code

Para contar con autocompletado inteligente, resaltado de sintaxis TextMate y depuración gráfica con puntos de interrupción:

1. Descarga el paquete `yaguarete-language-support-0.1.1.vsix` del release.
2. Abre tu terminal e instala la extensión ejecutando:

```bash
code --install-extension yaguarete-language-support-0.1.1.vsix
```

También puedes instalarla desde la interfaz de VS Code:
1. Abre VS Code.
2. Ve a la barra lateral de **Extensiones** (`Ctrl + Shift + X`).
3. Haz clic en el menú contextual de tres puntos (`...`) en la esquina superior de la pestaña.
4. Selecciona **"Install from VSIX..."** y elige el archivo descargado.

---

## 🏃 Tu Primer Programa

Para verificar que todo funcione correctamente, crea un archivo de prueba llamado `prueba.yaguarete`:

```yaguarete
// Archivo: prueba.yaguarete
INICIO
    CADENA desarrollador = "Estudiante"
    ENTERO anio = 2026
    IMPRIMIR("Bienvenido a YAGUARETÉ, " + desarrollador + " (" + anio + ")")
    
    ENTERO total = 0
    PARA (ENTERO i = 1; i <= 10; i++)
        total = total + i
    FIN PARA
    
    IMPRIMIR("La suma de los primeros 10 números es: " + total)
FIN
```

Ejecuta el programa:

```bash
yaguarete run prueba.yaguarete
```

Salida esperada:
```text
[RUN] Ejecutando archivo: prueba.yaguarete (lenient=false) ...
Bienvenido a YAGUARETÉ, Estudiante (2026)
La suma de los primeros 10 números es: 55
```

¡Felicitaciones! Tu entorno de YAGUARETÉ se encuentra 100% operativo y listo para programar.
