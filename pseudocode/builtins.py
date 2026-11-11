# GNU General Public License v3.0 or later (GPL-3.0-or-later)
#
# Copyright (c) 2026 [Facu Falcone](a.facundo.falcone@gmail.com) All rights reserved.
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

"""
Manejadores de archivos y funciones incorporadas (Built-ins) para el pseudocódigo.
Incluye ArchivoHandle con soporte para .findearchivo, .LEER_LINEA(), .ESCRIBIR_LINEA() y .CERRAR_ARCHIVO(),
además de ESPAR(), ESIMPAR(), MOD() y ABRIR_ARCHIVO().
"""

import math
import os
import random
from typing import List, Any, Optional, Union
from pseudocode.errors import RuntimeError


class ArchivoHandle:
    def __init__(self, path: str, mode: str):
        self.path = path
        self.mode = mode.lower()
        self.lines: List[str] = []
        self.cursor = 0
        self.is_closed = False
        self.read_during_iteration = False  # Para evitar bucle infinito si omiten LEER_LINEA()

        # Normalizar modos
        if self.mode in ("lectura", "r"):
            self.python_mode = "r"
            if not os.path.exists(path):
                raise RuntimeError(f"El archivo '{path}' no existe para lectura.")
            with open(path, "r", encoding="utf-8", errors="replace") as f:
                # Conservar las líneas sin el salto de línea al final
                self.lines = [line.rstrip("\r\n") for line in f]
        elif self.mode in ("escritura", "w"):
            self.python_mode = "w"
            self.file_writer = open(path, "w", encoding="utf-8")
        elif self.mode in ("anexo", "a"):
            self.python_mode = "a"
            self.file_writer = open(path, "a", encoding="utf-8")
        else:
            raise RuntimeError(f"Modo de archivo no reconocido: '{mode}'. Use 'lectura' o 'escritura'.")

    @property
    def findearchivo(self) -> bool:
        if self.is_closed:
            return True
        if self.python_mode == "r":
            return self.cursor >= len(self.lines)
        return True

    def LEER_LINEA(self) -> Any:
        if self.is_closed:
            raise RuntimeError("Intento de leer de un archivo ya cerrado.")
        if self.python_mode != "r":
            raise RuntimeError("El archivo no está abierto en modo lectura.")

        self.read_during_iteration = True
        if self.cursor >= len(self.lines):
            return ""
        line = self.lines[self.cursor]
        self.cursor += 1
        return parse_auto_value(line)

    def ESCRIBIR_LINEA(self, content: Any):
        if self.is_closed:
            raise RuntimeError("Intento de escribir en un archivo ya cerrado.")
        if self.python_mode not in ("w", "a"):
            raise RuntimeError("El archivo no está abierto en modo escritura.")

        text = stringify_value(content)
        self.file_writer.write(text + "\n")
        self.file_writer.flush()

    def CERRAR_ARCHIVO(self):
        if not self.is_closed:
            if hasattr(self, "file_writer") and self.file_writer:
                self.file_writer.close()
            self.is_closed = True

    def notify_loop_step(self):
        """
        Si un ciclo MIENTRAS NO file.findearchivo se ejecuta sin llamar a LEER_LINEA(),
        avanza el cursor en 1 para evitar bucles infinitos en algoritmos que cuentan líneas.
        """
        if self.python_mode == "r":
            if not self.read_during_iteration:
                if self.cursor < len(self.lines):
                    self.cursor += 1
            self.read_during_iteration = False

    def __repr__(self) -> str:
        return f"<ARCHIVO '{self.path}' modo='{self.mode}' cursor={self.cursor}/{len(self.lines)}>"


def parse_auto_value(text: str) -> Any:
    """
    Parsea automáticamente una línea de texto a su tipo primitivo correspondiente:
    - BOOLEANO: si es 'VERDADERO', 'FALSO', 'TRUE', 'FALSE'
    - ENTERO: si representa un número entero (ej. 3519, -400)
    - REAL / FLOTANTE: si representa un número decimal con punto o notación científica (ej. 35.19, -0.5)
    - CADENA: cualquier otro texto
    """
    if not isinstance(text, str):
        return text

    stripped = text.strip()
    if not stripped:
        return text

    # 1. Booleano
    upper = stripped.upper()
    if upper in ("VERDADERO", "TRUE"):
        return True
    if upper in ("FALSO", "FALSE"):
        return False

    # 2. Entero (ej. 1234, -450, +10)
    if (stripped.isdigit() or 
        (stripped[0] in ('+', '-') and len(stripped) > 1 and stripped[1:].isdigit())):
        try:
            return int(stripped)
        except ValueError:
            pass

    # 3. Flotante / Real (ej. 12.34, -0.5, 1e5)
    if "." in stripped or "e" in stripped.lower():
        try:
            return float(stripped)
        except ValueError:
            pass

    # 4. Cadena (texto por defecto)
    return text


def stringify_value(val: Any) -> str:
    """Convierte un valor de runtime a su representación canónica en pseudocódigo."""
    if isinstance(val, bool):
        return "VERDADERO" if val else "FALSO"
    if val is None:
        return "NULO"
    if isinstance(val, float):
        # Si es un número entero con formato flotante (ej 12.0), mostrar 12.0 o limpio
        if val.is_integer():
            return f"{val:.1f}"
        return str(val)
    if isinstance(val, list):
        return "[" + ", ".join(stringify_value(x) for x in val) + "]"
    return str(val)


# Builtins matemáticos y utilitarios
def builtin_abrir_archivo(path: str, mode: str) -> ArchivoHandle:
    return ArchivoHandle(path, mode)


def builtin_es_par(number: Any) -> bool:
    if not isinstance(number, (int, float)):
        raise RuntimeError(f"ESPAR requiere un número, se obtuvo {type(number).__name__}")
    return int(number) % 2 == 0


def builtin_es_impar(number: Any) -> bool:
    if not isinstance(number, (int, float)):
        raise RuntimeError(f"ESIMPAR requiere un número, se obtuvo {type(number).__name__}")
    return int(number) % 2 != 0


def builtin_mod(a: Any, b: Any) -> int:
    if not isinstance(a, (int, float)) or not isinstance(b, (int, float)):
        raise RuntimeError(f"MOD requiere dos operandos numéricos")
    return int(a) % int(b)


def builtin_raiz_cuadrada(number: Any) -> float:
    if not isinstance(number, (int, float)) or isinstance(number, bool):
        raise RuntimeError(f"RAIZCUADRADA requiere un número, se obtuvo {type(number).__name__}")
    if number < 0:
        raise RuntimeError("No se puede calcular la raíz cuadrada de un número negativo.")
    return math.sqrt(float(number))


def builtin_abs(number: Any) -> Union[int, float]:
    if not isinstance(number, (int, float)) or isinstance(number, bool):
        raise RuntimeError(f"ABS requiere un número, se obtuvo {type(number).__name__}")
    if isinstance(number, int):
        return abs(int(number))
    return abs(float(number))


def builtin_potencia(base: Any, exp: Any) -> float:
    if (not isinstance(base, (int, float)) or isinstance(base, bool) or
            not isinstance(exp, (int, float)) or isinstance(exp, bool)):
        raise RuntimeError("POTENCIA requiere dos operandos numéricos.")
    if base == 0 and exp < 0:
        raise RuntimeError("División por cero: no se puede elevar 0 a un exponente negativo.")
    if base < 0 and isinstance(exp, float) and not exp.is_integer():
        raise RuntimeError("No se puede elevar un número negativo a un exponente fraccionario real.")
    return float(math.pow(float(base), float(exp)))


def builtin_aleatorio(min_val: Any, max_val: Any) -> int:
    if (not isinstance(min_val, (int, float)) or isinstance(min_val, bool) or
            not isinstance(max_val, (int, float)) or isinstance(max_val, bool)):
        raise RuntimeError("ALEATORIO requiere dos operandos numéricos.")
    min_i = int(min_val)
    max_i = int(max_val)
    if min_i > max_i:
        raise RuntimeError("En ALEATORIO(min, max), 'min' no puede ser mayor que 'max'.")
    return random.randint(min_i, max_i)


# ---------------------------------------------------------------------------
# Builtins de texto, entrada, conversión y numéricos adicionales
# ---------------------------------------------------------------------------
def builtin_leer(prompt: Any = None, input_fn=input) -> Any:
    """LEER() / LEER(mensaje): lee una línea y la autoparsea."""
    import sys
    if prompt is not None:
        sys.stdout.write(stringify_value(prompt))
        sys.stdout.flush()
    try:
        line = input_fn()
    except EOFError:
        return ""
    if line is None or line == "":
        return ""
    return parse_auto_value(line)


def _require_text(name: str, value: Any) -> str:
    if not isinstance(value, str):
        raise RuntimeError(f"{name} requiere una cadena, se obtuvo {type(value).__name__}")
    return value


def builtin_mayusculas(text: Any) -> str:
    return _require_text("MAYUSCULAS", text).upper()


def builtin_minusculas(text: Any) -> str:
    return _require_text("MINUSCULAS", text).lower()


def builtin_recortar(text: Any) -> str:
    return _require_text("RECORTAR", text).strip()


def builtin_dividir_cadena(text: Any, sep: Any) -> List[str]:
    _require_text("DIVIDIR_CADENA", text)
    _require_text("DIVIDIR_CADENA", sep)
    if sep == "":
        raise RuntimeError("El separador no puede estar vacío.")
    return text.split(sep)


def _is_number(v: Any) -> bool:
    return isinstance(v, (int, float)) and not isinstance(v, bool)


def builtin_a_entero(value: Any) -> int:
    err = RuntimeError(f"No se puede convertir '{stringify_value(value)}' a ENTERO")
    if isinstance(value, bool):
        return 1 if value else 0
    if isinstance(value, int):
        return value
    if isinstance(value, float):
        if math.isnan(value) or math.isinf(value):
            raise err
        return int(value)
    if isinstance(value, str):
        s = value.strip()
        try:
            return int(s)
        except ValueError:
            pass
        try:
            f = float(s)
            if math.isnan(f) or math.isinf(f):
                raise err
            return int(f)
        except ValueError:
            raise err
    raise err


def builtin_a_real(value: Any) -> float:
    if isinstance(value, bool):
        return 1.0 if value else 0.0
    if isinstance(value, (int, float)):
        return float(value)
    if isinstance(value, str):
        try:
            return float(value.strip())
        except ValueError:
            pass
    raise RuntimeError(f"No se puede convertir '{stringify_value(value)}' a REAL")


def builtin_a_flotante(value: Any) -> float:
    try:
        return builtin_a_real(value)
    except RuntimeError:
        raise RuntimeError(f"No se puede convertir '{stringify_value(value)}' a FLOTANTE")


def builtin_a_cadena(value: Any) -> str:
    return stringify_value(value)


def builtin_a_booleano(value: Any) -> bool:
    if isinstance(value, bool):
        return value
    if isinstance(value, (int, float)):
        return value != 0
    if isinstance(value, str):
        u = value.strip().upper()
        if u in ("VERDADERO", "TRUE"):
            return True
        if u in ("FALSO", "FALSE"):
            return False
    raise RuntimeError(f"No se puede convertir '{stringify_value(value)}' a BOOLEANO")


def builtin_min(a: Any, b: Any) -> Union[int, float]:
    if not _is_number(a) or not _is_number(b):
        raise RuntimeError("MIN requiere dos operandos numéricos.")
    r = min(a, b)
    return r if isinstance(a, int) and isinstance(b, int) else float(r)


def builtin_max(a: Any, b: Any) -> Union[int, float]:
    if not _is_number(a) or not _is_number(b):
        raise RuntimeError("MAX requiere dos operandos numéricos.")
    r = max(a, b)
    return r if isinstance(a, int) and isinstance(b, int) else float(r)


def builtin_redondear(num: Any, decimales: Any = None) -> Union[int, float]:
    from decimal import Decimal, ROUND_HALF_UP
    if not _is_number(num):
        raise RuntimeError("REDONDEAR requiere un número.")
    if isinstance(num, float) and (math.isnan(num) or math.isinf(num)):
        raise RuntimeError("REDONDEAR no puede operar sobre NaN o infinito.")
    d = Decimal(str(num))
    if decimales is None:
        return int(d.quantize(Decimal(1), rounding=ROUND_HALF_UP))
    if not isinstance(decimales, int) or isinstance(decimales, bool) or decimales < 0:
        raise RuntimeError("REDONDEAR requiere que 'decimales' sea un entero mayor o igual a 0.")
    q = Decimal(1).scaleb(-decimales)
    return float(d.quantize(q, rounding=ROUND_HALF_UP))