# Security Policy / Política de Seguridad

El equipo de **YAGUARETÉ** se toma muy en serio la seguridad de sus usuarios y desarrolladores. Este documento describe la política oficial de seguridad, el modelo de confianza, las versiones respaldadas y el procedimiento de divulgación responsable para el lenguaje de programación YAGUARETÉ, sus herramientas CLI, depurador DAP, instalador oficial y extensiones de entorno de desarrollo.

---

## 📌 Supported Versions / Versiones Soportadas

Únicamente la versión estable oficial más reciente construida sobre el motor nativo en Rust recibe parches de seguridad activos y correcciones críticas:

| Versión / Version | Soportada / Supported | Arquitectura / Engine | Estado / Details |
| :--- | :---: | :--- | :--- |
| **0.1.1** | :white_check_mark: | **Rust Nativo (Core + CLI + DAP + Installer)** | **Versión Oficial Activa.** Recibe parches de seguridad, mitigaciones de vulnerabilidades y correcciones críticas. |
| `0.1.0` | :x: | Rust Nativo (Alpha inicial) | **No soportada.** Superada por 0.1.1. Se requiere actualizar inmediatamente. |
| `< 0.1.1` (versiones legacy) | :x: | Motor Python Histórico | **Fin de Ciclo de Vida (EOL).** Arquitectura discontinuada y sin mantenimiento. |
| Ramas de desarrollo (`main`, `dev`) | :x: | Commits Nightly / No etiquetados | **No soportada.** Sin garantías de estabilidad ni SLA de seguridad. |

### Cómo verificar y actualizar tu versión:

Para comprobar qué versión tienes instalada actualmente:
```bash
yaguarete --version
```

Para actualizar a la última versión segura:
```bash
yaguarete update
```
O descarga el instalador oficial más reciente desde los [Releases de GitHub](https://github.com/caidevOficial/Yaguarete_Lang/releases/latest).

---

## 🚨 Reporting a Vulnerability / Reportar una Vulnerabilidad

Agradecemos enormemente la labor de la comunidad de investigación y desarrollo para mantener seguro el ecosistema de YAGUARETÉ. **Por favor, NO abras issues públicos para reportar vulnerabilidades de seguridad.**

### Canales Oficiales y Seguros de Reporte

1. **GitHub Private Vulnerability Reporting (Canal Preferido):**  
   Puedes enviar un reporte de seguridad privado, cifrado y confidencial directamente a los mantenedores:  
   👉 **[Reportar Vulnerabilidad en YAGUARETÉ (GitHub PVR)](https://github.com/caidevOficial/Yaguarete_Lang/security/advisories/new)**

2. **Correo Electrónico Directo del Autor:**  
   Si no cuentas con una cuenta en GitHub o requieres comunicación directa por correo electrónico:  
   📧 **`a.facundo.falcone@gmail.com`**  
   * **Asunto sugerido:** `[SEGURIDAD] Vulnerabilidad en YAGUARETÉ - <Breve descripción>`

### Información a Incluir en el Reporte

Para permitir un triaje y remediación expeditivos, incluye la mayor cantidad de detalles posible:
- **Componente Afectado:** Motor Core (`yaguarete_core`), CLI / REPL (`yaguarete_cli`), Servidor DAP (`dap.rs`), Auto-actualizador (`updater.rs`), Instalador GUI de Windows, Extensión de VS Code o Workflows de CI/CD.
- **Tipo de Vulnerabilidad:** Inyección de comandos, escalada de privilegios local, manipulación de archivos (Path Traversal), desbordamiento de pila / DoS, etc.
- **Prueba de Concepto (PoC):** Código `.yaguarete`, script o secuencia reproducible paso a paso.
- **Evaluación de Impacto:** Estimación de la severidad (CVSS v3.1 si está disponible) y escenarios de explotación realistas.
- **Entorno:** Sistema operativo (Windows / Linux), versión de YAGUARETÉ y arquitectura del procesador (x64).

---

## ⏱️ Response Times & Remediation SLAs / Tiempos de Respuesta

Nos comprometemos con los siguientes acuerdos de nivel de servicio (SLA) para reportes de seguridad:

| Etapa | Plazo Máximo | Descripción |
| :--- | :---: | :--- |
| **Acuse de Recibo Inicial** | **48 horas hábiles** | Confirmación formal de que el reporte fue recibido y asignado a revisión por el mantenedor. |
| **Triaje y Clasificación** | **7 días corridos** | Validación de reproducibilidad, análisis de severidad (CVSS) y determinación de prioridad. |
| **Desarrollo del Parche** | **15 a 30 días corridos** | Creación del parche correctivo, suite de regresión y validación en entornos limpios. |
| **Publicación Coordinada** | **30 a 90 días (Embargo)** | Liberación de la versión con el parche, publicación del GitHub Security Advisory y reconocimiento formal al investigador. |

---

## 🔍 Trust Model & Architecture / Modelo de Confianza Arquitectónico

Para evaluar correctamente el alcance de un reporte, ten en cuenta las siguientes premisas de diseño del lenguaje:

### 1. Ejecución de Código en el Intérprete (`.yaguarete`)
- **Espacio de Usuario:** El motor de YAGUARETÉ se ejecuta como un proceso estándar en espacio de usuario (`User-Mode OS Process`) con los mismos privilegios del usuario del sistema operativo que lo ejecuta.
- **Sin Sandbox Aislado:** El intérprete no implementa actualmente un sandbox restringido a nivel de sistema operativo (chroot/seccomp en Linux o AppContainer en Windows).
- **Manejo de Archivos (`ARCHIVO`):** Las rutinas `ABRIR`, `LEER_LINEA` y `ESCRIBIR_LINEA` operan sobre el sistema de archivos del usuario. **No ejecutes scripts no confiables con privilegios elevados de Administrador (`sudo` / `Run as Administrator`).**
- **Seguridad de Memoria:** El núcleo está implementado en Safe Rust, garantizando protección de fábrica contra Use-After-Free y Buffer Overflows.

### 2. Protocolo de Depuración (Debug Adapter Protocol - DAP)
- **Canal Local `stdin`/`stdout`:** El subcomando `yaguarete dap` se comunica exclusivamente por canal estándar con el IDE anfitrión (VS Code, Cursor, Antigravity). No abre puertos de red ni sockets TCP en segundo plano.
- **Inspección Segura (`evaluate`):** Las expresiones evaluadas en depuración consultan los estados de memoria capturados en snapshots inmutables; no ejecutan código arbitrario ni funciones durante la inspección interactiva.

### 3. Auto-Actualizador (`updater`)
- **Canal Cifrado (HTTPS):** Las consultas de nuevas versiones y descargas se realizan exclusivamente contra la API oficial de GitHub Releases en `https://api.github.com/repos/caidevOficial/Yaguarete_Lang/releases/latest`.
- **Integridad:** Cada Release oficial publica las sumas de comprobación SHA-256 de todos sus binarios e instaladores para verificación de integridad previa a la ejecución.

### 4. Instalador para Windows (`Instalador_Yaguarete.exe`)
- **Principio de Menor Privilegio:** Instala el binario por defecto en `%LOCALAPPDATA%\Programs\Yaguarete\bin` y configura el entorno de usuario en `HKEY_CURRENT_USER\Environment\PATH`, sin requerir permisos de Administrador de Windows.

### 5. Extensión para Visual Studio Code
- **Workspace Trust:** Se recomienda abrir repositorios de código de terceros bajo el modo restringido (*Restricted Mode*) de VS Code para evitar la ejecución no supervisada de herramientas locales en espacios de trabajo no confiables.

---

## ⚖️ Safe Harbor Policy / Política de Puerto Seguro

Apoyamos plenamente la investigación ética y responsable en ciberseguridad. Consideramos explícitamente autorizadas todas las actividades de investigación realizadas de buena fe que cumplan las siguientes pautas:

- **Sin Impacto Destructivo:** No comprometer la disponibilidad de servicios, no destruir datos ni acceder a información privada de otros usuarios.
- **Divulgación Coordinada:** Respetar los canales privados de reporte y otorgar al mantenedor el tiempo prudencial establecido en el SLA para publicar el parche antes de divulgar detalles técnicos públicamente.
- **Compromiso Legal:** El autor y mantenedores de YAGUARETÉ se comprometen a **no emprender acciones legales** (ni civiles ni penales) contra investigadores que actúen de buena fe en conformidad con esta política de Safe Harbor.

---

## 🛡️ Buenas Prácticas para Usuarios y Desarrolladores

1. **Verificar Sumas SHA-256:** Siempre verifica que el hash SHA-256 de tu instalador `Instalador_Yaguarete.exe` o binario `yaguarete.exe` coincida con el publicado en el Release oficial de GitHub.
2. **Entornos Aislados:** Si ejecutas o evalúas código de pseudocódigo provisto por terceros desconocidos, ejecútalo en una máquina virtual o contenedor sin acceso a secretos locales.
3. **Mantenerse Actualizado:** Mantén instalada la versión 0.1.1 o superior ejecutando periódicamente `yaguarete update`.

---

*Última actualización: Noviembre 2026 — Proyecto Oficial YAGUARETÉ (Facu Falcone).*
