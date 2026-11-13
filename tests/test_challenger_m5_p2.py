# GNU General Public License v3.0 or later (GPL-3.0-or-later)
#
# Copyright (c) 2026 [Facu Falcone](a.facundo.falcone@gmail.com) All rights reserved.

"""
Adversarial Challenger Test Suite — Milestone 5: Phase 2 (Tier 5)
White-box stress testing of Python compatibility runtime (types_system.py & interpreter.py)
Covering Objectives 1-6.
"""

import unittest
from pseudocode.types_system import (
    DataType, StructType, StructInstance,
    TYPE_ENTERO, TYPE_REAL, TYPE_FLOTANTE, TYPE_CADENA,
    TYPE_BOOLEANO, TYPE_CARACTER, TYPE_NULO
)
from pseudocode import run_code


class TestChallengerM5P2PythonRuntime(unittest.TestCase):

    # =========================================================================
    # Objective 1: Self-referential & Deep Mutual References
    # =========================================================================

    def test_obj1_types_system_self_ref_no_autoinstantiation(self):
        node_type = StructType("Nodo", {"val": TYPE_ENTERO, "sig": StructType("Nodo")})
        n = StructInstance(node_type)
        # Verify uninitialized fields default to None (NULO)
        self.assertIsNone(n.get_field("val"))
        self.assertIsNone(n.get_field("sig"))
        self.assertEqual(str(n), "Nodo{val: NULO, sig: NULO}")

    def test_obj1_types_system_mutual_cycle_representation(self):
        t_a = StructType("A", {"id": TYPE_CADENA, "b": StructType("B")})
        t_b = StructType("B", {"id": TYPE_CADENA, "a": StructType("A")})

        inst_a = StructInstance(t_a)
        inst_b = StructInstance(t_b)

        inst_a.set_field("id", "NodeA")
        inst_b.set_field("id", "NodeB")
        inst_a.set_field("b", inst_b)
        inst_b.set_field("a", inst_a)

        # Cycle traversal
        self.assertEqual(inst_a.get_field("b").get_field("a").get_field("id"), "NodeA")
        # Cycle representation should handle recursion without stack overflow
        repr_a = str(inst_a)
        self.assertIn("<A ...>", repr_a)

    def test_obj1_interpreter_deep_mutual_reference_execution(self):
        code = """
        ESTRUCTURA Alfa:
            Beta b
            CADENA id
        FIN ESTRUCTURA

        ESTRUCTURA Beta:
            Alfa a
            CADENA id
        FIN ESTRUCTURA

        INICIO
            Alfa a
            Beta b
            SI (a.b == NULO Y b.a == NULO)
                IMPRIMIR("INIT_MUTUAL_NULO: VERDADERO")
            FIN SI
            a.id = "ALFA_OK"
            b.id = "BETA_OK"
            a.b = b
            b.a = a
            IMPRIMIR("TRAV: " + a.b.a.id)
        FIN
        """
        out = []
        run_code(code, output_collector=out)
        self.assertIn("INIT_MUTUAL_NULO: VERDADERO", out)
        self.assertIn("TRAV: ALFA_OK", out)

    # =========================================================================
    # Objective 2: Assignment of NULO back into fields
    # =========================================================================

    def test_obj2_types_system_reassign_nulo_all_primitives(self):
        st = StructType("Prims", {
            "e": TYPE_ENTERO,
            "r": TYPE_REAL,
            "c": TYPE_CADENA,
            "b": TYPE_BOOLEANO,
        })
        inst = StructInstance(st)
        inst.set_field("e", 42)
        inst.set_field("r", 3.14)
        inst.set_field("c", "Test")
        inst.set_field("b", True)

        self.assertEqual(inst.get_field("e"), 42)

        # Reassign to None
        inst.set_field("e", None)
        inst.set_field("r", None)
        inst.set_field("c", None)
        inst.set_field("b", None)

        self.assertIsNone(inst.get_field("e"))
        self.assertIsNone(inst.get_field("r"))
        self.assertIsNone(inst.get_field("c"))
        self.assertIsNone(inst.get_field("b"))

    def test_obj2_interpreter_reassign_nulo_flow(self):
        code = """
        ESTRUCTURA SubDato:
            ENTERO clave
        FIN ESTRUCTURA

        ESTRUCTURA Item:
            CADENA tag
            SubDato sub
        FIN ESTRUCTURA

        INICIO
            Item it
            SubDato sd
            sd.clave = 99
            it.sub = sd
            it.tag = "Z"

            it.tag = NULO
            it.sub = NULO
            SI (it.tag == NULO Y it.sub == NULO)
                IMPRIMIR("REASSIGN_OK: VERDADERO")
            FIN SI
        FIN
        """
        out = []
        run_code(code, output_collector=out)
        self.assertIn("REASSIGN_OK: VERDADERO", out)

    def test_obj2_interpreter_reassign_primitive_int_nulo_lenient(self):
        code = """
        ESTRUCTURA Item:
            ENTERO stock
        FIN ESTRUCTURA

        INICIO
            Item it
            it.stock = 50
            it.stock = NULO
            SI (it.stock == NULO)
                IMPRIMIR("INT_REASSIGN_NULO: VERDADERO")
            FIN SI
        FIN
        """
        out = []
        run_code(code, output_collector=out, lenient=True)
        self.assertIn("INT_REASSIGN_NULO: VERDADERO", out)

    # =========================================================================
    # Objective 3: Distinguishing NULO from primitive defaults
    # =========================================================================

    def test_obj3_interpreter_distinguish_nulo_from_primitives(self):
        code = """
        ESTRUCTURA Reg:
            ENTERO e
            REAL r
            CADENA c
            BOOLEANO b
        FIN ESTRUCTURA

        INICIO
            Reg x
            SI (x.e != 0 Y 0 != x.e)
                IMPRIMIR("NOT_0: OK")
            FIN SI
            SI (x.r != 0.0 Y 0.0 != x.r)
                IMPRIMIR("NOT_0_0: OK")
            FIN SI
            SI (x.c != "" Y "" != x.c)
                IMPRIMIR("NOT_EMPTY: OK")
            FIN SI
            SI (x.b != FALSO Y FALSO != x.b)
                IMPRIMIR("NOT_FALSE: OK")
            FIN SI
            SI (x.e == NULO Y x.r == NULO Y x.c == NULO Y x.b == NULO)
                IMPRIMIR("ALL_GENUINE_NULO: OK")
            FIN SI
        FIN
        """
        out = []
        run_code(code, output_collector=out)
        self.assertIn("NOT_0: OK", out)
        self.assertIn("NOT_0_0: OK", out)
        self.assertIn("NOT_EMPTY: OK", out)
        self.assertIn("NOT_FALSE: OK", out)
        self.assertIn("ALL_GENUINE_NULO: OK", out)

    # =========================================================================
    # Objective 4: Null Dereference Behavior
    # =========================================================================

    def test_obj4_types_system_nonexistent_field_raises_attribute_error(self):
        st = StructType("Vacia", {})
        inst = StructInstance(st)
        with self.assertRaises(AttributeError):
            inst.get_field("fantasma")
        with self.assertRaises(AttributeError):
            inst.set_field("fantasma", 123)

    def test_obj4_interpreter_null_deref_raises_runtime_error(self):
        code = """
        ESTRUCTURA Sub:
            ENTERO x
        FIN ESTRUCTURA

        ESTRUCTURA Main:
            Sub s
        FIN ESTRUCTURA

        INICIO
            Main m
            ENTERO val = m.s.x
        FIN
        """
        out = []
        with self.assertRaises(Exception):
            run_code(code, output_collector=out)

    # =========================================================================
    # Objective 5: Boundary Structs (0 Fields & 64 Fields)
    # =========================================================================

    def test_obj5_types_system_empty_struct(self):
        st = StructType("Vacia", {})
        inst = StructInstance(st)
        self.assertEqual(inst.fields, {})
        self.assertEqual(str(inst), "Vacia{}")

    def test_obj5_types_system_massive_struct_64_fields(self):
        fields = {f"f{i:02d}": TYPE_ENTERO for i in range(1, 65)}
        st = StructType("Massive", fields)
        inst = StructInstance(st)
        self.assertEqual(len(inst.fields), 64)
        for i in range(1, 65):
            self.assertIsNone(inst.get_field(f"f{i:02d}"))
        inst.set_field("f32", 999)
        self.assertEqual(inst.get_field("f32"), 999)
        self.assertIsNone(inst.get_field("f01"))
        self.assertIsNone(inst.get_field("f64"))

    # =========================================================================
    # Objective 6: Dynamic Comparisons
    # =========================================================================

    def test_obj6_interpreter_dynamic_comparisons(self):
        code = """
        ESTRUCTURA Item:
            ENTERO code
        FIN ESTRUCTURA

        INICIO
            Item it
            BOOLEANO b1 = (it.code == NULO)
            BOOLEANO b2 = (NULO == it.code)
            BOOLEANO b3 = (it.code != NULO)
            BOOLEANO b4 = (NULO != it.code)

            IMPRIMIR("B1: " + b1)
            IMPRIMIR("B2: " + b2)
            IMPRIMIR("B3: " + b3)
            IMPRIMIR("B4: " + b4)
        FIN
        """
        out = []
        run_code(code, output_collector=out)
        self.assertIn("B1: VERDADERO", out)
        self.assertIn("B2: VERDADERO", out)
        self.assertIn("B3: FALSO", out)
        self.assertIn("B4: FALSO", out)


if __name__ == "__main__":
    unittest.main()
