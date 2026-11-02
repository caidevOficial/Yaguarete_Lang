# 📁 Manejo de Archivos (`ARCHIVO`)

El lenguaje **YAGUARETÉ** cuenta con soporte nativo para persistencia y manipulación secuencial de archivos en disco a través del tipo de datos **`ARCHIVO`**.

---

## 🔑 Apertura de Archivos: `ABRIR_ARCHIVO`

Para interactuar con un archivo físico, se invoca la función integrada `ABRIR_ARCHIVO`:

```yaguarete
ARCHIVO descriptor = ABRIR_ARCHIVO(ruta: CADENA, modo: CADENA)
```

### Modos de Apertura Admitidos

| Modo | Alias Abreviado | Comportamiento |
| :--- | :---: | :--- |
| `"lectura"` | `"r"` | Abre un archivo existente para lectura secuencial. Si el archivo no existe, emite un error de ejecución. |
| `"escritura"` | `"w"` | Abre el archivo para escritura. Si el archivo no existe, lo crea; si ya existe, trunca su contenido previo a 0 bytes. |
| `"anexo"` | `"a"` | Abre el archivo en modo *append*. Agrega nuevas líneas al final sin borrar el contenido anterior. |

---

## 📖 Lectura Secuencial con Inferencia de Tipos

El método `.LEER_LINEA()` lee la siguiente línea disponible en el archivo y avanza el cursor de lectura.

### Inferencia Automática según la Variable Receptora

A diferencia de otros lenguajes donde la lectura de disco siempre devuelve cadenas que el programador debe convertir manualmente con `atoi` o `parse()`, en YAGUARETÉ **la conversión es automática y tipada** en base al tipo de la variable de destino:

```yaguarete
INICIO
    ARCHIVO f = ABRIR_ARCHIVO("datos.txt", "lectura")

    ENTERO numero = f.LEER_LINEA()      // Convierte texto a entero automáticamente
    REAL medicion = f.LEER_LINEA()      // Convierte texto a real (64-bit)
    BOOLEANO flag = f.LEER_LINEA()      // Interpreta "VERDADERO"/"FALSO" a booleano
    CADENA texto  = f.LEER_LINEA()      // Lee la línea como texto UTF-8

    f.CERRAR_ARCHIVO()

    IMPRIMIR("Número + 10: " + (numero + 10))
    IMPRIMIR("Medición: " + medicion)
FIN
```

---

## 🏁 Detección de Fin de Archivo: `.findearchivo`

La propiedad booleana de solo lectura `.findearchivo` evalúa si el cursor ha alcanzado el final del archivo (`EOF`). Es la herramienta fundamental para procesar archivos de longitud variable mediante un bucle `MIENTRAS`:

```yaguarete
INICIO
    ARCHIVO archivo_notas = ABRIR_ARCHIVO("notas.txt", "lectura")
    ENTERO suma = 0
    ENTERO total_alumnos = 0

    MIENTRAS (NO archivo_notas.findearchivo)
        ENTERO nota_alumno = archivo_notas.LEER_LINEA()
        suma += nota_alumno
        total_alumnos++
    FIN MIENTRAS

    archivo_notas.CERRAR_ARCHIVO()

    SI (total_alumnos > 0)
        REAL promedio = suma / total_alumnos
        IMPRIMIR("Promedio general del curso: " + promedio)
    FIN SI
FIN
```

---

## ✍️ Escritura en Disco: `.ESCRIBIR_LINEA`

El método `.ESCRIBIR_LINEA(valor)` convierte cualquier expresión recibida a su representación canónica en texto, le añade automáticamente un salto de línea (`\n`) y fuerza la sincronización en disco (`flush()`):

```yaguarete
INICIO
    ARCHIVO reporte = ABRIR_ARCHIVO("informe_salida.txt", "escritura")

    reporte.ESCRIBIR_LINEA("=== REPORTE GENERADO POR YAGUARETÉ ===")
    reporte.ESCRIBIR_LINEA("Fecha de emisión: 2026-11-01")
    reporte.ESCRIBIR_LINEA("Total de registros procesados: 150")
    reporte.ESCRIBIR_LINEA("Estado: COMPLETADO CON ÉXITO")

    reporte.CERRAR_ARCHIVO()
    IMPRIMIR("Informe guardado en disco exitosamente.")
FIN
```

---

## 🔒 Cierre Obligatorio: `.CERRAR_ARCHIVO`

Todo archivo abierto debe cerrarse al terminar su procesamiento mediante `.CERRAR_ARCHIVO()`. Esta operación:
1. Vacía los buffers pendientes en el sistema operativo.
2. Libera el descriptor de archivo del sistema.
3. Marca el manejador como cerrado para prevenir lecturas o escrituras posteriores sobre un descriptor inactivo.
