import io
import unittest
from contextlib import redirect_stdout

from pseudocode import run_code
from pseudocode.builtins import (
    builtin_leer, builtin_mayusculas, builtin_minusculas, builtin_recortar,
    builtin_dividir_cadena, builtin_a_entero, builtin_a_real, builtin_a_flotante,
    builtin_a_cadena, builtin_a_booleano, builtin_min, builtin_max, builtin_redondear,
)
from pseudocode.errors import RuntimeError as PseudoRuntimeError, TypeError as PseudoTypeError
from pseudocode.interpreter import Interpreter


def run(body):
    out = []
    run_code("INICIO\n" + body + "\nFIN", output_collector=out)
    return out


def run_with_input(body, lines):
    it = iter(lines)

    def fn():
        try:
            return next(it)
        except StopIteration:
            raise EOFError

    import pseudocode
    from pseudocode.lexer import Lexer
    from pseudocode.parser import Parser
    from pseudocode.type_checker import TypeChecker
    src = "INICIO\n" + body + "\nFIN"
    program = Parser(Lexer(src).tokenize(), src).parse()
    TypeChecker(src).check(program)
    out = []
    interp = Interpreter(src, output_collector=out, input_fn=fn)
    interp.run(program)
    return out


class TestLeer(unittest.TestCase):
    def test_tipos(self):
        cases = [("42", 42), ("3.5", 3.5), ("verdadero", True), ("FALSE", False),
                 ("hola", "hola"), ("", "")]
        for text, exp in cases:
            r = builtin_leer(input_fn=lambda t=text: t)
            self.assertEqual(r, exp)
            self.assertIs(type(r), type(exp))

    def test_eof(self):
        def fn():
            raise EOFError
        self.assertEqual(builtin_leer(input_fn=fn), "")

    def test_mensaje(self):
        buf = io.StringIO()
        with redirect_stdout(buf):
            r = builtin_leer("Edad: ", input_fn=lambda: "7")
        self.assertEqual(buf.getvalue(), "Edad: ")
        self.assertEqual(r, 7)

    def test_sin_mensaje_no_imprime(self):
        buf = io.StringIO()
        with redirect_stdout(buf):
            builtin_leer(input_fn=lambda: "x")
        self.assertEqual(buf.getvalue(), "")

    def test_interprete_default_input(self):
        self.assertIs(Interpreter("").input_fn, input)

    def test_en_programa(self):
        buf = io.StringIO()
        with redirect_stdout(buf):
            out = run_with_input(
                'ENTERO n = LEER("n? ")\nREAL r = LEER()\nCADENA s = LEER()\n'
                'BOOLEANO b = LEER()\nCADENA e = LEER()\n'
                'IMPRIMIR(n + 1)\nIMPRIMIR(r)\nIMPRIMIR(s)\nIMPRIMIR(b)\nIMPRIMIR("[" + e + "]")',
                ["5", "2.5", "hola", "verdadero"])
        self.assertEqual(out, ["6", "2.5", "hola", "VERDADERO", "[]"])
        self.assertIn("n? ", buf.getvalue())

    def test_aridad_tipo(self):
        with self.assertRaises(PseudoTypeError):
            run('ENTERO x = LEER(1, 2)')


class TestTexto(unittest.TestCase):
    def test_basicas(self):
        self.assertEqual(builtin_mayusculas("hOla"), "HOLA")
        self.assertEqual(builtin_minusculas("HoLa"), "hola")
        self.assertEqual(builtin_recortar("  hi \n"), "hi")

    def test_en_programa(self):
        out = run('IMPRIMIR(MAYUSCULAS("ab"))\nIMPRIMIR(minusculas("AB"))\nIMPRIMIR("[" + RECORTAR("  x ") + "]")')
        self.assertEqual(out, ["AB", "ab", "[x]"])

    def test_tipo_invalido(self):
        with self.assertRaises(PseudoTypeError):
            run('IMPRIMIR(MAYUSCULAS(5))')
        with self.assertRaises(PseudoRuntimeError):
            builtin_recortar(5)

    def test_dividir(self):
        self.assertEqual(builtin_dividir_cadena("a,b,,c", ","), ["a", "b", "", "c"])
        with self.assertRaises(PseudoRuntimeError) as cm:
            builtin_dividir_cadena("abc", "")
        self.assertEqual(cm.exception.message, "El separador no puede estar vacío.")

    def test_dividir_programa(self):
        out = run('CADENA partes[] = DIVIDIR_CADENA("a,b,,c", ",")\n'
                  'IMPRIMIR(partes.largo)\nIMPRIMIR(partes[0])\nIMPRIMIR(partes[3])')
        self.assertEqual(out, ["4", "a", "c"])

    def test_dividir_error_programa(self):
        with self.assertRaises(PseudoRuntimeError) as cm:
            run('CADENA p[] = DIVIDIR_CADENA("abc", "")')
        self.assertIn("El separador no puede estar vacío.", str(cm.exception))


class TestConversiones(unittest.TestCase):
    def test_a_entero(self):
        self.assertEqual(builtin_a_entero(" 12 "), 12)
        self.assertEqual(builtin_a_entero("3.9"), 3)
        self.assertEqual(builtin_a_entero("-3.9"), -3)
        self.assertEqual(builtin_a_entero(-2.7), -2)
        self.assertEqual(builtin_a_entero(True), 1)
        self.assertEqual(builtin_a_entero(False), 0)
        with self.assertRaises(PseudoRuntimeError) as cm:
            builtin_a_entero("abc")
        self.assertEqual(cm.exception.message, "No se puede convertir 'abc' a ENTERO")

    def test_a_real_flotante(self):
        self.assertEqual(builtin_a_real("2.5"), 2.5)
        self.assertIsInstance(builtin_a_real(3), float)
        self.assertIsInstance(builtin_a_flotante(3), float)
        with self.assertRaises(PseudoRuntimeError):
            builtin_a_real("x")
        with self.assertRaises(PseudoRuntimeError):
            builtin_a_flotante("x")

    def test_a_cadena(self):
        self.assertEqual(builtin_a_cadena(True), "VERDADERO")
        self.assertEqual(builtin_a_cadena(12), "12")
        self.assertEqual(builtin_a_cadena(1.0), "1.0")

    def test_a_booleano(self):
        self.assertIs(builtin_a_booleano(" verdadero "), True)
        self.assertIs(builtin_a_booleano("False"), False)
        self.assertIs(builtin_a_booleano(0), False)
        self.assertIs(builtin_a_booleano(2.5), True)
        self.assertIs(builtin_a_booleano(True), True)
        with self.assertRaises(PseudoRuntimeError):
            builtin_a_booleano("quizas")

    def test_en_programa_tipos(self):
        out = run('ENTERO a = A_ENTERO("7")\nREAL b = A_REAL("1.5")\nFLOTANTE c = A_FLOTANTE(2)\n'
                  'CADENA d = A_CADENA(9)\nBOOLEANO e = A_BOOLEANO("true")\n'
                  'IMPRIMIR(a)\nIMPRIMIR(b)\nIMPRIMIR(d)\nIMPRIMIR(e)')
        self.assertEqual(out, ["7", "1.5", "9", "VERDADERO"])

    def test_error_programa(self):
        with self.assertRaises(PseudoRuntimeError) as cm:
            run('ENTERO a = A_ENTERO("zz")')
        self.assertIn("No se puede convertir 'zz' a ENTERO", str(cm.exception))


class TestMinMaxRedondear(unittest.TestCase):
    def test_min_max(self):
        self.assertEqual(builtin_min(2, 3), 2)
        self.assertIsInstance(builtin_min(2, 3), int)
        self.assertEqual(builtin_max(2, 3.5), 3.5)
        self.assertIsInstance(builtin_max(5, 3.5), float)
        self.assertIsInstance(builtin_min(1, 2.0), float)
        with self.assertRaises(PseudoRuntimeError):
            builtin_min(True, 2)
        with self.assertRaises(PseudoRuntimeError):
            builtin_max("a", 2)

    def test_min_max_programa(self):
        out = run('ENTERO m = MIN(4, 2)\nREAL x = MAX(1, 2.5)\nIMPRIMIR(m)\nIMPRIMIR(x)')
        self.assertEqual(out, ["2", "2.5"])
        with self.assertRaises(PseudoTypeError):
            run('IMPRIMIR(MIN(1))')
        with self.assertRaises(PseudoTypeError):
            run('IMPRIMIR(MAX("a", 1))')

    def test_redondear(self):
        self.assertEqual(builtin_redondear(2.5), 3)
        self.assertEqual(builtin_redondear(-2.5), -3)
        self.assertEqual(builtin_redondear(3.4), 3)
        self.assertEqual(builtin_redondear(7), 7)
        self.assertIsInstance(builtin_redondear(2.5), int)
        self.assertEqual(builtin_redondear(3.14159, 2), 3.14)
        self.assertEqual(builtin_redondear(2.675, 2), 2.68)
        self.assertEqual(builtin_redondear(-1.005, 2), -1.01)
        self.assertIsInstance(builtin_redondear(3, 2), float)
        self.assertEqual(builtin_redondear(1.5, 0), 2.0)

    def test_redondear_errores(self):
        with self.assertRaises(PseudoRuntimeError):
            builtin_redondear(1.5, -1)
        with self.assertRaises(PseudoRuntimeError):
            builtin_redondear(1.5, 1.5)
        with self.assertRaises(PseudoRuntimeError):
            builtin_redondear("a")

    def test_redondear_programa(self):
        out = run('ENTERO a = REDONDEAR(2.5)\nREAL b = REDONDEAR(3.14159, 2)\nIMPRIMIR(a)\nIMPRIMIR(b)')
        self.assertEqual(out, ["3", "3.14"])


if __name__ == "__main__":
    unittest.main()