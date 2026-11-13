//! Adversarial Stress & Parity Test Suite for Milestone 5 Phase 2 (Challenger 2)
//!
//! Objectives:
//! 1. White-box inspect `OpCode::NewStruct` in `vm.rs`:
//!    - All fields default to `Value::Nulo`.
//!    - No implicit/automatic sub-struct allocations.
//!    - Stack balance & operand integrity across chained access and loops.
//!    - Controlled runtime errors on null dereference (no panic / segfault).
//! 2. Dual-Engine Parity (AST vs VM):
//!    - Bytecode null comparison symmetry (`val == NULO` vs `NULO == val`).
//!    - Reassignment of fields and entire structs back to `NULO`.
//!    - Passing uninitialized struct fields to functions and returning uninitialized structs.
//!    - Vector of structs with null elements.

use yaguarete_core::{run_code, run_code_vm};

fn assert_dual_engine_parity(code: &str, expected_stdout: &[&str]) {
    let ast_res = run_code(code, false).expect("AST execution failed");
    let vm_res = run_code_vm(code, false).expect("VM execution failed");

    // 1. Both outputs must match each other
    assert_eq!(
        ast_res, vm_res,
        "Engine divergence between AST and VM!\nAST: {:?}\nVM:  {:?}",
        ast_res, vm_res
    );

    // 2. Output must contain all expected substrings
    let combined = vm_res.join("\n");
    for &expected in expected_stdout {
        assert!(
            combined.contains(expected),
            "Expected output substring '{}' not found in:\n{}",
            expected,
            combined
        );
    }
}

fn assert_dual_engine_err_parity(code: &str, expected_err_fragment: &str) {
    let ast_res = run_code(code, false);
    let vm_res = run_code_vm(code, false);

    assert!(
        ast_res.is_err(),
        "AST unexpectedly succeeded when error was expected: {:?}",
        ast_res.unwrap()
    );
    assert!(
        vm_res.is_err(),
        "VM unexpectedly succeeded when error was expected: {:?}",
        vm_res.unwrap()
    );

    let ast_err = ast_res.unwrap_err().to_string();
    let vm_err = vm_res.unwrap_err().to_string();

    assert!(
        ast_err.contains(expected_err_fragment),
        "AST error '{}' does not contain '{}'",
        ast_err,
        expected_err_fragment
    );
    assert!(
        vm_err.contains(expected_err_fragment),
        "VM error '{}' does not contain '{}'",
        vm_err,
        expected_err_fragment
    );
}

// -----------------------------------------------------------------------------
// 1. White-box OpCode::NewStruct: All primitive and struct fields initialize to NULO
// -----------------------------------------------------------------------------

#[test]
fn test_adv_vm_newstruct_all_primitive_and_struct_fields_nulo() {
    let code = r#"
    ESTRUCTURA SubTipo:
        ENTERO sub_id
    FIN ESTRUCTURA

    ESTRUCTURA BigStruct:
        ENTERO f_int
        REAL f_real
        FLOTANTE f_flot
        CADENA f_str
        CARACTER f_chr
        BOOLEANO f_bool
        SubTipo f_sub
    FIN ESTRUCTURA

    INICIO
        BigStruct s
        SI (s.f_int == NULO)
            IMPRIMIR("INT_IS_NULO")
        FIN SI
        SI (s.f_real == NULO)
            IMPRIMIR("REAL_IS_NULO")
        FIN SI
        SI (s.f_flot == NULO)
            IMPRIMIR("FLOT_IS_NULO")
        FIN SI
        SI (s.f_str == NULO)
            IMPRIMIR("STR_IS_NULO")
        FIN SI
        SI (s.f_chr == NULO)
            IMPRIMIR("CHR_IS_NULO")
        FIN SI
        SI (s.f_bool == NULO)
            IMPRIMIR("BOOL_IS_NULO")
        FIN SI
        SI (s.f_sub == NULO)
            IMPRIMIR("SUB_IS_NULO")
        FIN SI
    FIN
    "#;

    assert_dual_engine_parity(
        code,
        &[
            "INT_IS_NULO",
            "REAL_IS_NULO",
            "FLOT_IS_NULO",
            "STR_IS_NULO",
            "CHR_IS_NULO",
            "BOOL_IS_NULO",
            "SUB_IS_NULO",
        ],
    );
}

// -----------------------------------------------------------------------------
// 2. White-box OpCode::NewStruct: No automatic child struct allocation
// -----------------------------------------------------------------------------

#[test]
fn test_adv_vm_newstruct_no_implicit_child_allocation() {
    let code = r#"
    ESTRUCTURA Nivel3:
        ENTERO val3
    FIN ESTRUCTURA

    ESTRUCTURA Nivel2:
        Nivel3 n3
    FIN ESTRUCTURA

    ESTRUCTURA Nivel1:
        Nivel2 n2
    FIN ESTRUCTURA

    INICIO
        Nivel1 root
        SI (root.n2 == NULO)
            IMPRIMIR("NIVEL2_IS_NULO_VERIFIED")
        FIN SI
    FIN
    "#;

    assert_dual_engine_parity(code, &["NIVEL2_IS_NULO_VERIFIED"]);
}

// -----------------------------------------------------------------------------
// 3. Controlled runtime errors on null read & write dereference (no panic/segfault)
// -----------------------------------------------------------------------------

#[test]
fn test_adv_vm_controlled_null_read_dereference_error() {
    let code = r#"
    ESTRUCTURA Nodo:
        ENTERO val
        Nodo sig
    FIN ESTRUCTURA

    INICIO
        Nodo n
        IMPRIMIR(n.sig.val)
    FIN
    "#;

    assert_dual_engine_err_parity(
        code,
        "Intento de acceder al campo 'val' en una referencia nula (NULO)",
    );
}

#[test]
fn test_adv_vm_controlled_null_write_dereference_error() {
    let code = r#"
    ESTRUCTURA Nodo:
        ENTERO val
        Nodo sig
    FIN ESTRUCTURA

    INICIO
        Nodo n
        n.sig.val = 42
    FIN
    "#;

    assert_dual_engine_err_parity(
        code,
        "Intento de asignar el campo 'val' en una referencia nula (NULO)",
    );
}

// -----------------------------------------------------------------------------
// 4. Bytecode comparison symmetry (a.val == NULO vs NULO == a.val)
// -----------------------------------------------------------------------------

#[test]
fn test_adv_vm_null_comparison_symmetry() {
    let code = r#"
    ESTRUCTURA Item:
        CADENA nombre
    FIN ESTRUCTURA

    INICIO
        Item it
        // 1. Uninitialized comparison symmetry
        SI (it.nombre == NULO Y NULO == it.nombre)
            IMPRIMIR("SYM_EQ_NULO: VERDADERO")
        FIN SI
        SI (NO (it.nombre != NULO) Y NO (NULO != it.nombre))
            IMPRIMIR("SYM_NEQ_NULO: FALSO")
        FIN SI

        // 2. Assigned comparison symmetry
        it.nombre = "Hardware"
        SI (NO (it.nombre == NULO) Y NO (NULO == it.nombre))
            IMPRIMIR("SYM_ASSIGNED_EQ_NULO: FALSO")
        FIN SI
        SI (it.nombre != NULO Y NULO != it.nombre)
            IMPRIMIR("SYM_ASSIGNED_NEQ_NULO: VERDADERO")
        FIN SI

        // 3. Reassigned back to NULO symmetry
        it.nombre = NULO
        SI (it.nombre == NULO Y NULO == it.nombre)
            IMPRIMIR("SYM_REASSIGNED_EQ_NULO: VERDADERO")
        FIN SI
    FIN
    "#;

    assert_dual_engine_parity(
        code,
        &[
            "SYM_EQ_NULO: VERDADERO",
            "SYM_NEQ_NULO: FALSO",
            "SYM_ASSIGNED_EQ_NULO: FALSO",
            "SYM_ASSIGNED_NEQ_NULO: VERDADERO",
            "SYM_REASSIGNED_EQ_NULO: VERDADERO",
        ],
    );
}

// -----------------------------------------------------------------------------
// 5. Stack balance & memory integrity: Chained access and rapid loop instantiation
// -----------------------------------------------------------------------------

#[test]
fn test_adv_vm_stack_balance_and_loop_instantiation() {
    let code = r#"
    ESTRUCTURA Contador:
        ENTERO paso
    FIN ESTRUCTURA

    FUNCION contar_iteraciones(): ENTERO
        ENTERO total = 0
        PARA (ENTERO i = 0; i < 200; i++)
            Contador c
            c.paso = 1
            total = total + c.paso
        FIN PARA
        RETORNAR total
    FIN FUNCION

    INICIO
        ENTERO res = contar_iteraciones()
        IMPRIMIR("TOTAL_ITERACIONES:", res)
    FIN
    "#;

    assert_dual_engine_parity(code, &["TOTAL_ITERACIONES: 200"]);
}

// -----------------------------------------------------------------------------
// 6. Reassignment of entire struct variable back to NULO and subsequent dereference
// -----------------------------------------------------------------------------

#[test]
fn test_adv_vm_reassign_struct_variable_to_nulo_then_deref() {
    let code = r#"
    ESTRUCTURA Cuenta:
        ENTERO saldo
    FIN ESTRUCTURA

    INICIO
        Cuenta c
        c.saldo = 1000
        IMPRIMIR("SALDO_INICIAL:", c.saldo)

        // Reassign variable to NULO
        c = NULO
        SI (c == NULO)
            IMPRIMIR("CUENTA_ES_NULO: VERDADERO")
        FIN SI

        // Subsequent dereference must fail gracefully
        IMPRIMIR(c.saldo)
    FIN
    "#;

    assert_dual_engine_err_parity(
        code,
        "Intento de acceder al campo 'saldo' en una referencia nula (NULO)",
    );
}

// -----------------------------------------------------------------------------
// 7. Functions: Passing uninitialized struct fields & returning uninitialized structs
// -----------------------------------------------------------------------------

#[test]
fn test_adv_vm_functions_uninitialized_fields_and_returns() {
    let code = r#"
    ESTRUCTURA Registro:
        ENTERO clave
        CADENA etiqueta
    FIN ESTRUCTURA

    FUNCION verificar_registro(r: Registro): CADENA
        SI (r.clave == NULO)
            RETORNAR "CLAVE_ES_NULO"
        SINO
            RETORNAR "CLAVE_TIENE_VALOR"
        FIN SI
    FIN FUNCION

    FUNCION crear_registro_vacio(): Registro
        Registro r
        RETORNAR r
    FIN FUNCION

    INICIO
        Registro reg
        CADENA est1 = verificar_registro(reg)
        IMPRIMIR("EST1:", est1)

        reg.clave = 777
        CADENA est2 = verificar_registro(reg)
        IMPRIMIR("EST2:", est2)

        Registro fab = crear_registro_vacio()
        SI (fab.clave == NULO Y fab.etiqueta == NULO)
            IMPRIMIR("FABRICADO_VACIO: VERDADERO")
        FIN SI
        fab.etiqueta = "Activo"
        IMPRIMIR("FAB_ETIQUETA:", fab.etiqueta)
    FIN
    "#;

    assert_dual_engine_parity(
        code,
        &[
            "EST1: CLAVE_ES_NULO",
            "EST2: CLAVE_TIENE_VALOR",
            "FABRICADO_VACIO: VERDADERO",
            "FAB_ETIQUETA: Activo",
        ],
    );
}

// -----------------------------------------------------------------------------
// 8. Vectors/Arrays of structs with uninitialized and re-nulled elements
// -----------------------------------------------------------------------------

#[test]
fn test_adv_vm_vector_of_structs_mutation_and_renulling() {
    let code = r#"
    ESTRUCTURA Paquete:
        ENTERO id
        CADENA destino
    FIN ESTRUCTURA

    INICIO
        Paquete envios[3] = [NULO, NULO, NULO]

        SI (envios[0] == NULO Y envios[1] == NULO Y envios[2] == NULO)
            IMPRIMIR("TODOS_SLOTS_NULOS: VERDADERO")
        FIN SI

        Paquete p1
        p1.id = 101
        p1.destino = "Asunción"
        envios[1] = p1

        SI (envios[0] == NULO Y envios[1] != NULO Y envios[2] == NULO)
            IMPRIMIR("SLOT1_ASIGNADO: VERDADERO")
            IMPRIMIR("P1_DESTINO:", envios[1].destino)
        FIN SI

        // Reassign slot back to NULO
        envios[1] = NULO
        SI (envios[1] == NULO)
            IMPRIMIR("SLOT1_RESET_NULO: VERDADERO")
        FIN SI
    FIN
    "#;

    assert_dual_engine_parity(
        code,
        &[
            "TODOS_SLOTS_NULOS: VERDADERO",
            "SLOT1_ASIGNADO: VERDADERO",
            "P1_DESTINO: Asunción",
            "SLOT1_RESET_NULO: VERDADERO",
        ],
    );
}

// -----------------------------------------------------------------------------
// 9. Uninitialized field return value comparison against NULO
// -----------------------------------------------------------------------------

#[test]
fn test_adv_vm_function_returning_uninitialized_field_direct_comparison() {
    let code = r#"
    ESTRUCTURA Caja:
        ENTERO contenido
    FIN ESTRUCTURA

    FUNCION extraer_contenido(c: Caja): ENTERO
        RETORNAR c.contenido
    FIN FUNCION

    INICIO
        Caja c
        SI (extraer_contenido(c) == NULO)
            IMPRIMIR("CONTENIDO_EXTRAIDO_ES_NULO")
        FIN SI

        c.contenido = 42
        SI (extraer_contenido(c) != NULO)
            IMPRIMIR("CONTENIDO_EXTRAIDO_TIENE_VALOR:", extraer_contenido(c))
        FIN SI
    FIN
    "#;

    assert_dual_engine_parity(
        code,
        &[
            "CONTENIDO_EXTRAIDO_ES_NULO",
            "CONTENIDO_EXTRAIDO_TIENE_VALOR: 42",
        ],
    );
}

// -----------------------------------------------------------------------------
// 10. Documented Divergence: Passing uninitialized primitive field to primitive param
// -----------------------------------------------------------------------------

#[test]
fn test_adv_divergence_uninitialized_primitive_field_passed_to_typed_param() {
    let code = r#"
    ESTRUCTURA Dato:
        ENTERO num
    FIN ESTRUCTURA

    FUNCION consumir_entero(val: ENTERO): CADENA
        SI (val == NULO)
            RETORNAR "VAL_NULO"
        SINO
            RETORNAR "VAL_PRESENTE"
        FIN SI
    FIN FUNCION

    INICIO
        Dato d
        CADENA res = consumir_entero(d.num)
        IMPRIMIR(res)
    FIN
    "#;

    // AST enforces typed variable declaration in function call frame -> Incompatibilidad de tipos
    let ast_res = run_code(code, false);
    assert!(ast_res.is_err());
    let ast_err = ast_res.unwrap_err().to_string();
    assert!(ast_err.contains("Incompatibilidad de tipos: no se puede asignar valor a variable de tipo 'ENTERO'"));

    // VM uses stack slot binding without runtime argument type enforcement -> receives NULO cleanly
    let vm_res = run_code_vm(code, false);
    assert!(vm_res.is_ok());
    assert_eq!(vm_res.unwrap(), vec!["VAL_NULO"]);
}
