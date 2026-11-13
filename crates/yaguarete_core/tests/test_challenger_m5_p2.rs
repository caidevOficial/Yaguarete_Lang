//! Adversarial Challenger Test Suite — Milestone 5: Phase 2 (Tier 5 Hardening)
//! Focus: White-box adversarial stress tests on AST Interpreter & Bytecode VM
//! Objectives:
//! 1. Self-referential structs and deep mutual references without auto-instantiation.
//! 2. Assignment of NULO back into previously non-null fields (primitives and structs).
//! 3. Distinguishing NULO from primitive defaults (0, 0.0, "", FALSO, " ").
//! 4. Null dereference behavior: read & write on NULO must raise controlled runtime errors, never panics.
//! 5. Boundary structs: 0-field empty struct and 64-field massive struct.
//! 6. Dynamic comparisons: obj.attr == NULO, NULO == obj.attr, truthiness, short-circuiting.

use yaguarete_core::{run_code, run_code_vm};

fn assert_dual_engine_ok(code: &str, expected_substrings: &[&str]) {
    // 1. AST Interpreter
    let out_ast = run_code(code, false).unwrap_or_else(|e| {
        panic!("AST interpreter failed for code:\n{}\nError: {:?}", code, e);
    });
    for exp in expected_substrings {
        assert!(
            out_ast.iter().any(|line| line.contains(exp)),
            "AST output did not contain '{}'. Full AST output:\n{:?}",
            exp,
            out_ast
        );
    }

    // 2. Bytecode VM
    let out_vm = run_code_vm(code, false).unwrap_or_else(|e| {
        panic!("Bytecode VM failed for code:\n{}\nError: {:?}", code, e);
    });
    for exp in expected_substrings {
        assert!(
            out_vm.iter().any(|line| line.contains(exp)),
            "VM output did not contain '{}'. Full VM output:\n{:?}",
            exp,
            out_vm
        );
    }
}

fn assert_dual_engine_err(code: &str, expected_err_substring: &str) {
    // 1. AST Interpreter
    let res_ast = run_code(code, false);
    assert!(res_ast.is_err(), "AST unexpectedly succeeded for code:\n{}", code);
    let err_ast = res_ast.unwrap_err().to_string();
    assert!(
        err_ast.contains(expected_err_substring),
        "AST error did not contain '{}'. Actual error:\n{}",
        expected_err_substring,
        err_ast
    );

    // 2. Bytecode VM
    let res_vm = run_code_vm(code, false);
    assert!(res_vm.is_err(), "Bytecode VM unexpectedly succeeded for code:\n{}", code);
    let err_vm = res_vm.unwrap_err().to_string();
    assert!(
        err_vm.contains(expected_err_substring),
        "VM error did not contain '{}'. Actual error:\n{}",
        expected_err_substring,
        err_vm
    );
}

// ============================================================================
// Objective 1: Self-referential & Deep Mutual References
// ============================================================================

#[test]
fn test_adv_obj1_deep_mutual_reference_cycle_and_traversal() {
    let code = r#"
    ESTRUCTURA Alfa:
        Beta b
        CADENA id
    FIN ESTRUCTURA

    ESTRUCTURA Beta:
        Gamma g
        CADENA id
    FIN ESTRUCTURA

    ESTRUCTURA Gamma:
        Alfa a
        CADENA id
    FIN ESTRUCTURA

    INICIO
        Alfa a
        Beta b
        Gamma g

        // Verify initial state: all nested pointers default to NULO
        SI (a.b == NULO Y b.g == NULO Y g.a == NULO)
            IMPRIMIR("INIT_MUTUAL_NULO: VERDADERO")
        FIN SI

        // Connect mutual reference cycle
        a.id = "A_NODE"
        b.id = "B_NODE"
        g.id = "G_NODE"

        a.b = b
        b.g = g
        g.a = a

        // Deep cycle traversal
        IMPRIMIR("TRAV_1: " + a.b.id)
        IMPRIMIR("TRAV_2: " + a.b.g.id)
        IMPRIMIR("TRAV_3: " + a.b.g.a.id)
        IMPRIMIR("TRAV_6: " + a.b.g.a.b.g.id)

        // Verify cycle printing does not crash or infinite loop
        IMPRIMIR("CANONICAL_ALFA: " + a)
    FIN
    "#;

    assert_dual_engine_ok(code, &[
        "INIT_MUTUAL_NULO: VERDADERO",
        "TRAV_1: B_NODE",
        "TRAV_2: G_NODE",
        "TRAV_3: A_NODE",
        "TRAV_6: G_NODE",
        "CANONICAL_ALFA: Alfa{",
    ]);
}

#[test]
fn test_adv_obj1_quad_mutual_reference_network() {
    let code = r#"
    ESTRUCTURA Nodo4:
        ENTERO id
        Nodo4 sig
        Nodo4 alt
    FIN ESTRUCTURA

    INICIO
        Nodo4 n1
        Nodo4 n2
        Nodo4 n3
        Nodo4 n4

        // Check uninitialized pointers
        SI (n1.sig == NULO Y n1.alt == NULO Y n4.sig == NULO)
            IMPRIMIR("QUAD_UNINIT_OK: VERDADERO")
        FIN SI

        n1.id = 1
        n2.id = 2
        n3.id = 3
        n4.id = 4

        n1.sig = n2
        n1.alt = n3
        n2.sig = n3
        n2.alt = n4
        n3.sig = n4
        n3.alt = n1
        n4.sig = n1
        n4.alt = n2

        IMPRIMIR("PATH_1: " + n1.sig.sig.sig.sig.id)
        IMPRIMIR("PATH_ALT: " + n1.alt.alt.alt.id)
    FIN
    "#;

    assert_dual_engine_ok(code, &[
        "QUAD_UNINIT_OK: VERDADERO",
        "PATH_1: 1",
        "PATH_ALT: 3",
    ]);
}

// ============================================================================
// Objective 2: Reassigning NULO back into fields
// ============================================================================

#[test]
fn test_adv_obj2_reassign_nulo_all_primitive_types() {
    let code = r#"
    ESTRUCTURA Primitivos:
        ENTERO e
        REAL r
        CADENA c
        BOOLEANO b
        CARACTER k
    FIN ESTRUCTURA

    INICIO
        Primitivos p
        // 1. Populate
        p.e = 42
        p.r = 3.14
        p.c = "Hola"
        p.b = VERDADERO
        p.k = "X"

        IMPRIMIR("POPULATED: " + p.e + " " + p.r + " " + p.c + " " + p.b + " " + p.k)

        // 2. Reassign all back to NULO
        p.e = NULO
        p.r = NULO
        p.c = NULO
        p.b = NULO
        p.k = NULO

        SI (p.e == NULO Y p.r == NULO Y p.c == NULO Y p.b == NULO Y p.k == NULO)
            IMPRIMIR("ALL_REASSIGNED_NULO: VERDADERO")
        FIN SI

        SI (p.e != NULO O p.r != NULO O p.c != NULO O p.b != NULO O p.k != NULO)
            IMPRIMIR("ERROR: No deberia ser distinto de nulo")
        SINO
            IMPRIMIR("ALL_NOT_EQUAL_NULO_CHECK: FALSO")
        FIN SI

        // 3. Reassign back to new values
        p.e = 100
        p.c = "Reasignado"
        IMPRIMIR("RE_POPULATED: " + p.e + " " + p.c)
    FIN
    "#;

    assert_dual_engine_ok(code, &[
        "POPULATED: 42 3.14 Hola VERDADERO X",
        "ALL_REASSIGNED_NULO: VERDADERO",
        "ALL_NOT_EQUAL_NULO_CHECK: FALSO",
        "RE_POPULATED: 100 Reasignado",
    ]);
}

#[test]
fn test_adv_obj2_reassign_nested_struct_and_deref_behavior() {
    let code = r#"
    ESTRUCTURA SubDato:
        ENTERO clave
    FIN ESTRUCTURA

    ESTRUCTURA Contenedor:
        SubDato sub
    FIN ESTRUCTURA

    INICIO
        Contenedor c
        SubDato sd
        sd.clave = 777
        c.sub = sd

        IMPRIMIR("CLAVE_INIT: " + c.sub.clave)

        // Reassign back to NULO
        c.sub = NULO
        SI (c.sub == NULO)
            IMPRIMIR("SUB_IS_NULO: VERDADERO")
        FIN SI
    FIN
    "#;

    assert_dual_engine_ok(code, &[
        "CLAVE_INIT: 777",
        "SUB_IS_NULO: VERDADERO",
    ]);
}

#[test]
fn test_adv_obj2_reassign_nested_struct_deref_fails_ast_and_vm() {
    let code = r#"
    ESTRUCTURA SubDato:
        ENTERO clave
    FIN ESTRUCTURA

    ESTRUCTURA Contenedor:
        SubDato sub
    FIN ESTRUCTURA

    INICIO
        Contenedor c
        SubDato sd
        sd.clave = 777
        c.sub = sd
        c.sub = NULO
        // Dereferencing after reassigning to NULO must fail with controlled error
        ENTERO k = c.sub.clave
    FIN
    "#;

    assert_dual_engine_err(code, "referencia nula (NULO)");
}

#[test]
fn test_adv_obj2_stress_loop_alternating_nulo_assignments() {
    let code = r#"
    ESTRUCTURA Buffer:
        ENTERO idx
        CADENA tag
    FIN ESTRUCTURA

    INICIO
        Buffer b
        ENTERO nulo_count = 0
        ENTERO val_count = 0

        PARA (ENTERO i = 0; i < 30; i++)
            b.idx = i
            b.tag = "iter_" + i
            SI (b.idx != NULO Y b.tag != NULO)
                val_count++
            FIN SI

            b.idx = NULO
            b.tag = NULO
            SI (b.idx == NULO Y b.tag == NULO)
                nulo_count++
            FIN SI
        FIN PARA

        IMPRIMIR("VAL_COUNT: " + val_count)
        IMPRIMIR("NULO_COUNT: " + nulo_count)
    FIN
    "#;

    assert_dual_engine_ok(code, &[
        "VAL_COUNT: 30",
        "NULO_COUNT: 30",
    ]);
}

// ============================================================================
// Objective 3: Distinguishing NULO from Primitive Defaults
// ============================================================================

#[test]
fn test_adv_obj3_distinguish_nulo_from_primitives_strict_symmetry() {
    let code = r#"
    ESTRUCTURA Medidas:
        ENTERO contador
        REAL porcentaje
        CADENA nombre
        BOOLEANO flag
        CARACTER inicial
    FIN ESTRUCTURA

    INICIO
        Medidas m

        // 1. Uninitialized != 0 / 0.0 / "" / FALSO / " "
        SI (m.contador != 0 Y 0 != m.contador)
            IMPRIMIR("DIFF_INT_0: VERDADERO")
        FIN SI
        SI (m.porcentaje != 0.0 Y 0.0 != m.porcentaje)
            IMPRIMIR("DIFF_REAL_0: VERDADERO")
        FIN SI
        SI (m.nombre != "" Y "" != m.nombre)
            IMPRIMIR("DIFF_CADENA_EMPTY: VERDADERO")
        FIN SI
        SI (m.flag != FALSO Y FALSO != m.flag)
            IMPRIMIR("DIFF_BOOL_FALSE: VERDADERO")
        FIN SI
        SI (m.flag != VERDADERO Y VERDADERO != m.flag)
            IMPRIMIR("DIFF_BOOL_TRUE: VERDADERO")
        FIN SI
        SI (m.inicial != " " Y " " != m.inicial)
            IMPRIMIR("DIFF_CHAR_SPACE: VERDADERO")
        FIN SI

        // 2. Equality to primitive defaults must evaluate to FALSO
        SI (m.contador == 0 O 0 == m.contador)
            IMPRIMIR("FAIL: contador no es 0")
        SINO
            IMPRIMIR("EQ_INT_0: FALSO")
        FIN SI
        SI (m.porcentaje == 0.0 O 0.0 == m.porcentaje)
            IMPRIMIR("FAIL: porcentaje no es 0.0")
        SINO
            IMPRIMIR("EQ_REAL_0: FALSO")
        FIN SI
        SI (m.nombre == "" O "" == m.nombre)
            IMPRIMIR("FAIL: nombre no es vacio")
        SINO
            IMPRIMIR("EQ_CADENA_EMPTY: FALSO")
        FIN SI
        SI (m.flag == FALSO O FALSO == m.flag)
            IMPRIMIR("FAIL: flag no es falso")
        SINO
            IMPRIMIR("EQ_BOOL_FALSE: FALSO")
        FIN SI

        // 3. String comparison with literal "NULO" text
        SI (m.nombre == "NULO")
            IMPRIMIR("FAIL: nombre no es la cadena literal NULO")
        SINO
            IMPRIMIR("EQ_CADENA_LITERAL_NULO: FALSO")
        FIN SI

        // 4. Genuine NULO equality
        SI (m.contador == NULO Y NULO == m.contador)
            IMPRIMIR("GENUINE_NULO_INT: VERDADERO")
        FIN SI
        SI (m.nombre == NULO Y NULO == m.nombre)
            IMPRIMIR("GENUINE_NULO_CADENA: VERDADERO")
        FIN SI
    FIN
    "#;

    assert_dual_engine_ok(code, &[
        "DIFF_INT_0: VERDADERO",
        "DIFF_REAL_0: VERDADERO",
        "DIFF_CADENA_EMPTY: VERDADERO",
        "DIFF_BOOL_FALSE: VERDADERO",
        "DIFF_BOOL_TRUE: VERDADERO",
        "DIFF_CHAR_SPACE: VERDADERO",
        "EQ_INT_0: FALSO",
        "EQ_REAL_0: FALSO",
        "EQ_CADENA_EMPTY: FALSO",
        "EQ_BOOL_FALSE: FALSO",
        "EQ_CADENA_LITERAL_NULO: FALSO",
        "GENUINE_NULO_INT: VERDADERO",
        "GENUINE_NULO_CADENA: VERDADERO",
    ]);
}

// ============================================================================
// Objective 4: Null Dereference Behavior (Controlled Errors)
// ============================================================================

#[test]
fn test_adv_obj4_err_read_on_uninitialized_nested_field() {
    let code = r#"
    ESTRUCTURA Interior:
        ENTERO secreto
    FIN ESTRUCTURA

    ESTRUCTURA Exterior:
        Interior in
    FIN ESTRUCTURA

    INICIO
        Exterior ex
        ENTERO s = ex.in.secreto
    FIN
    "#;

    assert_dual_engine_err(code, "Intento de acceder al campo 'secreto' en una referencia nula (NULO)");
}

#[test]
fn test_adv_obj4_err_write_on_uninitialized_nested_field() {
    let code = r#"
    ESTRUCTURA Interior:
        ENTERO secreto
    FIN ESTRUCTURA

    ESTRUCTURA Exterior:
        Interior in
    FIN ESTRUCTURA

    INICIO
        Exterior ex
        ex.in.secreto = 999
    FIN
    "#;

    assert_dual_engine_err(code, "Intento de asignar el campo 'secreto' en una referencia nula (NULO)");
}

#[test]
fn test_adv_obj4_err_vector_element_null_deref() {
    let code = r#"
    ESTRUCTURA Celda:
        ENTERO ocupante
    FIN ESTRUCTURA

    INICIO
        Celda matriz[2]
        matriz[0] = NULO
        ENTERO val_o = matriz[0].ocupante
    FIN
    "#;

    assert_dual_engine_err(code, "referencia nula (NULO)");
}

#[test]
fn test_adv_obj4_short_circuit_prevents_null_dereference() {
    let code = r#"
    ESTRUCTURA Nodo:
        ENTERO val
        Nodo sig
    FIN ESTRUCTURA

    INICIO
        Nodo n
        // Logical AND short circuit: n.sig == NULO -> second operand must not be evaluated
        SI (n.sig != NULO Y n.sig.val == 10)
            IMPRIMIR("AND_BRANCH: VERDADERO")
        SINO
            IMPRIMIR("AND_SHORT_CIRCUIT_SAFE: VERDADERO")
        FIN SI

        // Logical OR short circuit: n.sig == NULO is true -> second operand must not be evaluated
        SI (n.sig == NULO O n.sig.val == 10)
            IMPRIMIR("OR_SHORT_CIRCUIT_SAFE: VERDADERO")
        FIN SI
    FIN
    "#;

    assert_dual_engine_ok(code, &[
        "AND_SHORT_CIRCUIT_SAFE: VERDADERO",
        "OR_SHORT_CIRCUIT_SAFE: VERDADERO",
    ]);
}

// ============================================================================
// Objective 5: Boundary Structs (0 Fields & 64 Fields)
// ============================================================================

#[test]
fn test_adv_obj5_empty_struct_definition_and_usage() {
    let code = r#"
    ESTRUCTURA Vacia:
    FIN ESTRUCTURA

    INICIO
        Vacia v1
        Vacia v2

        // Identity comparison
        SI (v1 == v1)
            IMPRIMIR("SELF_EQ: VERDADERO")
        FIN SI

        SI (v1 != v2)
            IMPRIMIR("DIFF_INSTANCES: VERDADERO")
        FIN SI

        IMPRIMIR("CANONICAL_EMPTY: " + v1)
    FIN
    "#;

    assert_dual_engine_ok(code, &[
        "SELF_EQ: VERDADERO",
        "DIFF_INSTANCES: VERDADERO",
        "CANONICAL_EMPTY: Vacia{}",
    ]);
}

#[test]
fn test_adv_obj5_empty_struct_nonexistent_field_read_fails() {
    let code = r#"
    ESTRUCTURA Vacia:
    FIN ESTRUCTURA

    INICIO
        Vacia v
        ENTERO f = v.fantasma
    FIN
    "#;

    assert_dual_engine_err(code, "no tiene el campo 'fantasma'");
}

/// Both AST interpreter and Bytecode VM enforce struct field schema on write,
/// cleanly rejecting writing to non-existent fields with a controlled runtime error.
#[test]
fn test_adv_obj5_nonexistent_field_write_fails_ast_and_vm() {
    let code = r#"
    ESTRUCTURA Vacia:
    FIN ESTRUCTURA

    INICIO
        Vacia v
        v.fantasma = 123
    FIN
    "#;

    assert_dual_engine_err(code, "no tiene el campo 'fantasma'");
}

#[test]
fn test_adv_obj5_massive_struct_64_fields() {
    // Generate massive struct with 64 fields
    let mut struct_defs = String::from("ESTRUCTURA MegaStruct:\n");
    for i in 1..=64 {
        if i % 3 == 0 {
            struct_defs.push_str(&format!("        CADENA f{:02}\n", i));
        } else if i % 2 == 0 {
            struct_defs.push_str(&format!("        REAL f{:02}\n", i));
        } else {
            struct_defs.push_str(&format!("        ENTERO f{:02}\n", i));
        }
    }
    struct_defs.push_str("    FIN ESTRUCTURA\n\n");

    let code = format!(
        r#"
        {}
        INICIO
            MegaStruct m

            // Check field 1, 32, 64 initial NULO
            SI (m.f01 == NULO Y m.f32 == NULO Y m.f64 == NULO)
                IMPRIMIR("MASSIVE_INIT_NULO: VERDADERO")
            FIN SI

            // Mutate boundaries
            m.f01 = 111
            m.f32 = 222.5
            m.f64 = 333.3

            IMPRIMIR("F01: " + m.f01)
            IMPRIMIR("F32: " + m.f32)
            IMPRIMIR("F64: " + m.f64)

            // Reassign middle back to NULO
            m.f32 = NULO
            SI (m.f32 == NULO)
                IMPRIMIR("F32_RESET: VERDADERO")
            FIN SI

            // Verify untouched field remains NULO
            SI (m.f15 == NULO Y m.f50 == NULO)
                IMPRIMIR("UNTOUCHED_NULO: VERDADERO")
            FIN SI
        FIN
        "#,
        struct_defs
    );

    assert_dual_engine_ok(&code, &[
        "MASSIVE_INIT_NULO: VERDADERO",
        "F01: 111",
        "F32: 222.5",
        "F64: 333.3",
        "F32_RESET: VERDADERO",
        "UNTOUCHED_NULO: VERDADERO",
    ]);
}

// ============================================================================
// Objective 6: Dynamic Comparisons & Truthiness
// ============================================================================

#[test]
fn test_adv_obj6_dynamic_comparisons_and_boolean_variables() {
    let code = r#"
    ESTRUCTURA Elemento:
        ENTERO codigo
        CADENA nombre
    FIN ESTRUCTURA

    INICIO
        Elemento el

        // Expressions stored in boolean variables
        BOOLEANO b_eq1 = (el.codigo == NULO)
        BOOLEANO b_eq2 = (NULO == el.codigo)
        BOOLEANO b_neq1 = (el.codigo != NULO)
        BOOLEANO b_neq2 = (NULO != el.codigo)

        IMPRIMIR("B_EQ1: " + b_eq1)
        IMPRIMIR("B_EQ2: " + b_eq2)
        IMPRIMIR("B_NEQ1: " + b_neq1)
        IMPRIMIR("B_NEQ2: " + b_neq2)

        // Null keyword variations (null, None)
        SI (el.nombre == null Y null == el.nombre)
            IMPRIMIR("LOWERCASE_NULL: VERDADERO")
        FIN SI
        SI (el.nombre == None Y None == el.nombre)
            IMPRIMIR("MIXEDCASE_NONE: VERDADERO")
        FIN SI
    FIN
    "#;

    assert_dual_engine_ok(code, &[
        "B_EQ1: VERDADERO",
        "B_EQ2: VERDADERO",
        "B_NEQ1: FALSO",
        "B_NEQ2: FALSO",
        "LOWERCASE_NULL: VERDADERO",
        "MIXEDCASE_NONE: VERDADERO",
    ]);
}

#[test]
fn test_adv_obj6_truthiness_semantics_of_nulo_attribute() {
    let code = r#"
    ESTRUCTURA Objeto:
        ENTERO dato
    FIN ESTRUCTURA

    INICIO
        Objeto obj

        // Implicit condition: NULO evaluates as falsy
        SI (obj.dato)
            IMPRIMIR("FAIL: nulo no es truthy")
        SINO
            IMPRIMIR("NULO_IS_FALSY: VERDADERO")
        FIN SI

        // Negation: NO NULO evaluates as truthy
        SI (NO obj.dato)
            IMPRIMIR("NOT_NULO_IS_TRUTHY: VERDADERO")
        FIN SI

        // When populated with non-zero, becomes truthy
        obj.dato = 42
        SI (obj.dato)
            IMPRIMIR("ASSIGNED_IS_TRUTHY: VERDADERO")
        FIN SI
    FIN
    "#;

    assert_dual_engine_ok(code, &[
        "NULO_IS_FALSY: VERDADERO",
        "NOT_NULO_IS_TRUTHY: VERDADERO",
        "ASSIGNED_IS_TRUTHY: VERDADERO",
    ]);
}

#[test]
fn test_adv_obj6_while_loop_dynamic_termination_on_nulo() {
    let code = r#"
    ESTRUCTURA NodoL:
        ENTERO v
        NodoL sig
    FIN ESTRUCTURA

    INICIO
        NodoL n1
        NodoL n2
        NodoL n3

        n1.v = 10
        n2.v = 20
        n3.v = 30

        n1.sig = n2
        n2.sig = n3
        // n3.sig is NULO

        NodoL curr = n1
        ENTERO suma = 0
        ENTERO pasos = 0

        MIENTRAS (curr != NULO)
            suma = suma + curr.v
            pasos++
            curr = curr.sig
        FIN MIENTRAS

        IMPRIMIR("PASOS: " + pasos)
        IMPRIMIR("SUMA: " + suma)
        SI (curr == NULO)
            IMPRIMIR("TERMINAL_IS_NULO: VERDADERO")
        FIN SI
    FIN
    "#;

    assert_dual_engine_ok(code, &[
        "PASOS: 3",
        "SUMA: 60",
        "TERMINAL_IS_NULO: VERDADERO",
    ]);
}
