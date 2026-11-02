# 🛡️ Política de Seguridad (SECURITY)

Bienvenido a la documentación oficial de la **Política de Seguridad** del lenguaje **YAGUARETÉ**.

El proyecto se compromete activamente con los más altos estándares de calidad, robustez y protección tanto para estudiantes que dan sus primeros pasos en algoritmos como para docentes e investigadores que utilizan nuestras herramientas.

---

## 📌 Matriz de Versiones y Soporte

Únicamente la versión estable oficial más reciente construida sobre el motor nativo en Rust recibe parches de seguridad activos y correcciones críticas:

| Versión | Arquitectura | Soporte de Seguridad | Observaciones |
| :--- | :--- | :---: | :--- |
| **0.1.1** | **Rust Nativo (Core, CLI, DAP, Installer)** | **Soportada Activada** | **Versión Oficial Actual.** Recibe parches continuos, mitigaciones y fixes. |
| `0.1.0` | Rust Nativo (Alpha) | No soportada | Versión superada; actualizar a 0.1.1. |
| `< 0.1.1` | Python Legacy | Fin de Vida (EOL) | Arquitectura descontinuada sin soporte. |
| Ramas `main`/`dev` | Git Nightly | No soportada | Sin garantías de estabilidad o SLA. |

---

## 🚨 Canales de Reporte Responsable

Si has descubierto una vulnerabilidad de seguridad en cualquier componente de YAGUARETÉ, te solicitamos **no divulgarla públicamente ni abrir un Issue de GitHub público**.

### 1. GitHub Private Vulnerability Reporting (Recomendado)
A través de la infraestructura nativa y cifrada de GitHub:
👉 **[Reportar Vulnerabilidad en YAGUARETÉ](https://github.com/caidevOficial/Yaguarete_Lang/security/advisories/new)**

### 2. Correo Electrónico Directo
Puedes contactar de forma directa al autor y mantenedor principal:
* **Email:** `a.facundo.falcone@gmail.com`
* **Asunto:** `[SEGURIDAD] Vulnerabilidad en YAGUARETÉ - <Breve descripción>`

---

## ⏱️ Acuerdos de Nivel de Servicio (SLA)

| Etapa | Plazo Máximo | Compromiso |
| :--- | :---: | :--- |
| **Acuse de Recibo** | **48 horas hábiles** | Confirmación formal de la recepción del reporte por el equipo. |
| **Triaje Técnico** | **7 días corridos** | Validación de reproducibilidad y asignación de severidad CVSS v3.1. |
| **Desarrollo del Parche** | **15 a 30 días** | Desarrollo y testeo del fix de seguridad en suite de integración. |
| **Divulgación Coordinada** | **30 a 90 días** | Publicación del parche, Security Advisory oficial y crédito al autor del hallazgo. |

---

## 🔍 Modelo de Confianza Arquitectónico

1. **Intérprete de Usuario:** El motor de YAGUARETÉ se ejecuta en espacio de usuario. No implementa un sandbox a nivel de SO, por lo que las operaciones del tipo nativo `ARCHIVO` interactúan con el sistema de archivos real con los permisos del usuario actual.
2. **Seguridad de Memoria en Rust:** Gracias al núcleo desarrollado en Rust (`Rc<RefCell<...>>`), el intérprete previene por diseño vulnerabilidades de corrupción de memoria como desbordamiento de búfer y Use-After-Free.
3. **Depurador DAP:** El subcomando `yaguarete dap` funciona únicamente sobre `stdin`/`stdout` y realiza lecturas de memoria pasivas (snapshots inmutables), evitando ejecución de código malicioso durante la inspección.
4. **Auto-actualizador:** Realiza peticiones HTTPS a la API pública de GitHub Releases y proporciona sumas SHA-256 en cada publicación.

---

## ⚖️ Política de Puerto Seguro (Safe Harbor)

Cualquier investigación de seguridad efectuada de buena fe que respete las directrices de este documento es considerada autorizada. El mantenedor se compromete a no emprender acciones legales contra investigadores que actúen conforme a esta política.
