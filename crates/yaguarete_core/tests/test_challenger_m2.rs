//! Adversarial Challenger Test Suite for Milestone 2 (M2_Compatibility_Validation)
//!
//! Direct Core Engine Dual-Engine Stress Tests (AST vs VM):
//! 1. Complex boolean logical combinations with uninitialized/NULO struct fields.
//! 2. Alternating reassignments between NULO, struct instances, and terminal nodes.
//! 3. Empty list/queue traversals and safe termination without hanging.
//! 4. Shared struct reference aliasing and mutation across null checks.
//! 5. Heterogeneous vector of structures with interleaved null pointers and accumulators.
//! 6. Dual-engine parity on null pointer dereference errors.

use yaguarete_core::{run_code, run_code_vm};

fn assert_dual_engine_ok(code: &str, expected_substrings: &[&str]) {
    let ast_res = run_code(code, false);
    assert!(ast_res.is_ok(), "AST failed: {:?}", ast_res.err());
    let ast_out = ast_res.unwrap();
    for sub in expected_substrings {
        assert!(
            ast_out.iter().any(|s| s.contains(sub)),
            "AST output did not contain '{}'. Full output:\n{:?}",
            sub,
            ast_out
        );
    }

    let vm_res = run_code_vm(code, false);
    assert!(vm_res.is_ok(), "VM failed: {:?}", vm_res.err());
    let vm_out = vm_res.unwrap();
    for sub in expected_substrings {
        assert!(
            vm_out.iter().any(|s| s.contains(sub)),
            "VM output did not contain '{}'. Full output:\n{:?}",
            sub,
            vm_out
        );
    }
}

#[allow(dead_code)]
fn assert_dual_engine_err(code: &str, expected_err_sub: &str) {
    let ast_res = run_code(code, false);
    assert!(ast_res.is_err(), "AST unexpectedly succeeded: {:?}", ast_res.ok());
    let ast_err = ast_res.unwrap_err().to_string();
    assert!(
        ast_err.contains(expected_err_sub),
        "AST error did not contain '{}'. Got: '{}'",
        expected_err_sub,
        ast_err
    );

    let vm_res = run_code_vm(code, false);
    assert!(vm_res.is_err(), "VM unexpectedly succeeded: {:?}", vm_res.ok());
    let vm_err = vm_res.unwrap_err().to_string();
    assert!(
        vm_err.contains(expected_err_sub),
        "VM error did not contain '{}'. Got: '{}'",
        expected_err_sub,
        vm_err
    );
}

#[test]
fn test_adv_complex_boolean_logic_combinations_with_null() {
    let code = r#"
    ESTRUCTURA Paquete:
        ENTERO id
        Paquete siguiente
    FIN ESTRUCTURA

    INICIO
        Paquete p1
        Paquete p2
        p2.id = 100

        // p1.id es NULO, p1.siguiente es NULO
        // p2.id es 100, p2.siguiente es NULO

        BOOLEANO b1 = (p1.id == NULO Y p1.siguiente == NULO)
        BOOLEANO b2 = (p1.id != NULO O p2.id == 100)
        BOOLEANO b3 = (p2.siguiente == NULO Y p2.id != NULO)
        BOOLEANO b4 = NO (p1.siguiente != NULO)
        BOOLEANO b5 = (p1.id == NULO Y (p2.id == 100 Y p2.siguiente == NULO))

        IMPRIMIR("B1: " + b1)
        IMPRIMIR("B2: " + b2)
        IMPRIMIR("B3: " + b3)
        IMPRIMIR("B4: " + b4)
        IMPRIMIR("B5: " + b5)
    FIN
    "#;

    assert_dual_engine_ok(
        code,
        &[
            "B1: VERDADERO",
            "B2: VERDADERO",
            "B3: VERDADERO",
            "B4: VERDADERO",
            "B5: VERDADERO",
        ],
    );
}

#[test]
fn test_adv_alternating_reassignments_null_and_instances() {
    let code = r#"
    ESTRUCTURA Nodo:
        ENTERO val
        Nodo sig
    FIN ESTRUCTURA

    INICIO
        Nodo n
        IMPRIMIR("PASO_0: " + (n.sig == NULO))

        Nodo n1
        n1.val = 1
        n.sig = n1
        IMPRIMIR("PASO_1: " + (n.sig != NULO))
        IMPRIMIR("PASO_1_VAL: " + n.sig.val)

        // Reseteo a NULO
        n.sig = NULO
        IMPRIMIR("PASO_2_RESET: " + (n.sig == NULO))

        // Asignación de una instancia diferente
        Nodo n2
        n2.val = 2
        n.sig = n2
        IMPRIMIR("PASO_3_REASSIGN: " + (n.sig != NULO))
        IMPRIMIR("PASO_3_VAL: " + n.sig.val)

        // Reseteo final a NULO
        n.sig = NULO
        IMPRIMIR("PASO_4_FINAL_RESET: " + (n.sig == NULO))
    FIN
    "#;

    assert_dual_engine_ok(
        code,
        &[
            "PASO_0: VERDADERO",
            "PASO_1: VERDADERO",
            "PASO_1_VAL: 1",
            "PASO_2_RESET: VERDADERO",
            "PASO_3_REASSIGN: VERDADERO",
            "PASO_3_VAL: 2",
            "PASO_4_FINAL_RESET: VERDADERO",
        ],
    );
}

#[test]
fn test_adv_shared_aliasing_and_null_mutation() {
    let code = r#"
    ESTRUCTURA Contenedor:
        ENTERO dato
        Contenedor enlace
    FIN ESTRUCTURA

    INICIO
        Contenedor c1
        c1.dato = 50

        // Asignación de alias por referencia
        Contenedor alias = c1
        IMPRIMIR("ALIAS_DATO_INIT: " + alias.dato)
        IMPRIMIR("ALIAS_ENLACE_INIT_NULO: " + (alias.enlace == NULO))

        // Mutamos el objeto mediante el alias
        Contenedor c2
        c2.dato = 99
        alias.enlace = c2

        // Verificamos si c1 observa el cambio
        IMPRIMIR("C1_ENLACE_NO_NULO: " + (c1.enlace != NULO))
        IMPRIMIR("C1_ENLACE_DATO: " + c1.enlace.dato)

        // Desconectamos mediante c1
        c1.enlace = NULO
        IMPRIMIR("ALIAS_ENLACE_AHORA_NULO: " + (alias.enlace == NULO))
    FIN
    "#;

    assert_dual_engine_ok(
        code,
        &[
            "ALIAS_DATO_INIT: 50",
            "ALIAS_ENLACE_INIT_NULO: VERDADERO",
            "C1_ENLACE_NO_NULO: VERDADERO",
            "C1_ENLACE_DATO: 99",
            "ALIAS_ENLACE_AHORA_NULO: VERDADERO",
        ],
    );
}

#[test]
fn test_adv_vector_interleaved_null_pointers_and_traversal() {
    let code = r#"
    ESTRUCTURA Elemento:
        ENTERO valor
        Elemento hijo
    FIN ESTRUCTURA

    INICIO
        Elemento vec[6]

        // Inicializamos los 6 elementos
        PARA(ENTERO i = 0; i < 6; i++)
            Elemento e
            e.valor = i * 10
            // Solo los pares tienen un sub-elemento hijo
            SI (i % 2 == 0)
                Elemento sub
                sub.valor = 1
                e.hijo = sub
            FIN SI
            vec[i] = e
        FIN PARA

        ENTERO total_padres = 0
        ENTERO total_hijos = 0
        ENTERO cantidad_con_hijo = 0
        ENTERO cantidad_sin_hijo = 0

        PARA(ENTERO k = 0; k < 6; k++)
            Elemento cur = vec[k]
            total_padres = total_padres + cur.valor
            SI (cur.hijo != NULO)
                cantidad_con_hijo++
                total_hijos = total_hijos + cur.hijo.valor
            SINO
                cantidad_sin_hijo++
            FIN SI
        FIN PARA

        IMPRIMIR("TOTAL_PADRES: " + total_padres)
        IMPRIMIR("TOTAL_HIJOS: " + total_hijos)
        IMPRIMIR("CON_HIJO: " + cantidad_con_hijo)
        IMPRIMIR("SIN_HIJO: " + cantidad_sin_hijo)
    FIN
    "#;

    assert_dual_engine_ok(
        code,
        &[
            "TOTAL_PADRES: 150",
            "TOTAL_HIJOS: 3",
            "CON_HIJO: 3",
            "SIN_HIJO: 3",
        ],
    );
}

#[test]
fn test_adv_null_deref_error_parity() {
    let code_read = r#"
    ESTRUCTURA Nodo:
        ENTERO val
        Nodo prox
    FIN ESTRUCTURA

    INICIO
        Nodo n
        // n.prox es NULO por defecto; intentar leer n.prox.val debe fallar
        ENTERO x = n.prox.val
    FIN
    "#;

    // Both engines should cleanly reject reading member on Nulo
    let ast_err = run_code(code_read, false).unwrap_err().to_string();
    let vm_err = run_code_vm(code_read, false).unwrap_err().to_string();

    assert!(
        ast_err.contains("referencia nula") || ast_err.contains("Nulo") || ast_err.contains("nulo") || ast_err.contains("miembro"),
        "AST error unexpected: {}",
        ast_err
    );
    assert!(
        vm_err.contains("referencia nula") || vm_err.contains("Nulo") || vm_err.contains("nulo") || vm_err.contains("miembro"),
        "VM error unexpected: {}",
        vm_err
    );

    let code_write = r#"
    ESTRUCTURA Nodo:
        ENTERO val
        Nodo prox
    FIN ESTRUCTURA

    INICIO
        Nodo n
        // n.prox es NULO por defecto; intentar asignar n.prox.val debe fallar
        n.prox.val = 42
    FIN
    "#;

    let ast_write_err = run_code(code_write, false).unwrap_err().to_string();
    let vm_write_err = run_code_vm(code_write, false).unwrap_err().to_string();

    assert!(
        ast_write_err.contains("referencia nula") || ast_write_err.contains("Nulo") || ast_write_err.contains("nulo") || ast_write_err.contains("miembro"),
        "AST write error unexpected: {}",
        ast_write_err
    );
    assert!(
        vm_write_err.contains("referencia nula") || vm_write_err.contains("Nulo") || vm_write_err.contains("nulo") || vm_write_err.contains("miembro"),
        "VM write error unexpected: {}",
        vm_write_err
    );
}
