//! Comprehensive Dual-Engine Integration Test Suite: Struct Default NULO Initialization
//! Covers:
//! - Tier 1: Feature Coverage (F1 to F6, 30 tests)
//! - Tier 2: Boundary & Corner Cases (7 topics, 35 tests)
//! - Tier 3: Cross-Feature Combinations (8 tests)
//! Total: 73 dual-engine integration tests exercising AST Interpreter and Bytecode VM.

use yaguarete_core::{run_code, run_code_vm};

// ============================================================================
// Dual-Engine Assertion Helpers
// ============================================================================

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
    assert!(res_ast.is_err(), "AST expected error but succeeded for code:\n{}", code);
    let ast_err = res_ast.unwrap_err().to_string();
    assert!(
        ast_err.to_lowercase().contains(&expected_err_substring.to_lowercase()),
        "AST error '{}' does not contain expected substring '{}'",
        ast_err,
        expected_err_substring
    );

    // 2. Bytecode VM
    let res_vm = run_code_vm(code, false);
    assert!(res_vm.is_err(), "VM expected error but succeeded for code:\n{}", code);
    let vm_err = res_vm.unwrap_err().to_string();
    assert!(
        vm_err.to_lowercase().contains(&expected_err_substring.to_lowercase()),
        "VM error '{}' does not contain expected substring '{}'",
        vm_err,
        expected_err_substring
    );
}

// ============================================================================
// TIER 1: FEATURE COVERAGE (F1 to F6)
// ============================================================================

// --- F1: Default NULO for primitive attributes ---

#[test]
fn test_tier1_f1_entero_primitive_default_nulo() {
    let code = r#"
    ESTRUCTURA RegistroEntero:
        ENTERO contador
    FIN ESTRUCTURA

    INICIO
        RegistroEntero r
        SI (r.contador == NULO)
            IMPRIMIR("ENTERO_ES_NULO: VERDADERO")
        FIN SI
        IMPRIMIR("REPR: " + r)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "ENTERO_ES_NULO: VERDADERO",
        "REPR: RegistroEntero{contador: NULO}",
    ]);
}

#[test]
fn test_tier1_f1_real_and_flotante_primitive_default_nulo() {
    let code = r#"
    ESTRUCTURA Decimales:
        REAL precision_doble
        FLOTANTE precision_simple
    FIN ESTRUCTURA

    INICIO
        Decimales d
        SI (d.precision_doble == NULO)
            IMPRIMIR("REAL_ES_NULO: VERDADERO")
        FIN SI
        SI (d.precision_simple == NULO)
            IMPRIMIR("FLOTANTE_ES_NULO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "REAL_ES_NULO: VERDADERO",
        "FLOTANTE_ES_NULO: VERDADERO",
    ]);
}

#[test]
fn test_tier1_f1_cadena_primitive_default_nulo() {
    let code = r#"
    ESTRUCTURA TextoInfo:
        CADENA descripcion
    FIN ESTRUCTURA

    INICIO
        TextoInfo t
        SI (t.descripcion == NULO)
            IMPRIMIR("CADENA_ES_NULO: VERDADERO")
        FIN SI
        SI (t.descripcion != "")
            IMPRIMIR("NO_ES_CADENA_VACIA: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "CADENA_ES_NULO: VERDADERO",
        "NO_ES_CADENA_VACIA: VERDADERO",
    ]);
}

#[test]
fn test_tier1_f1_booleano_and_caracter_primitive_default_nulo() {
    let code = r#"
    ESTRUCTURA EstadoYSimbolo:
        BOOLEANO activo
        CARACTER simbolo
    FIN ESTRUCTURA

    INICIO
        EstadoYSimbolo e
        SI (e.activo == NULO)
            IMPRIMIR("BOOLEANO_ES_NULO: VERDADERO")
        FIN SI
        SI (e.simbolo == NULO)
            IMPRIMIR("CARACTER_ES_NULO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "BOOLEANO_ES_NULO: VERDADERO",
        "CARACTER_ES_NULO: VERDADERO",
    ]);
}

#[test]
fn test_tier1_f1_all_primitives_combined_default_nulo() {
    let code = r#"
    ESTRUCTURA TodosLosPrimitivos:
        ENTERO f_entero
        REAL f_real
        FLOTANTE f_flotante
        CADENA f_cadena
        CARACTER f_caracter
        BOOLEANO f_booleano
    FIN ESTRUCTURA

    INICIO
        TodosLosPrimitivos p
        BOOLEANO todo_nulo = (p.f_entero == NULO Y p.f_real == NULO Y p.f_flotante == NULO Y p.f_cadena == NULO Y p.f_caracter == NULO Y p.f_booleano == NULO)
        SI (todo_nulo)
            IMPRIMIR("TODOS_PRIMITIVOS_NULOS: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["TODOS_PRIMITIVOS_NULOS: VERDADERO"]);
}

// --- F2: Default NULO for nested & self-referential struct attributes ---

#[test]
fn test_tier1_f2_nested_struct_single_level_nulo() {
    let code = r#"
    ESTRUCTURA Motor:
        ENTERO potencia
    FIN ESTRUCTURA

    ESTRUCTURA Auto:
        CADENA modelo
        Motor motor
    FIN ESTRUCTURA

    INICIO
        Auto a
        SI (a.motor == NULO)
            IMPRIMIR("MOTOR_SUBSTRUCT_NULO: VERDADERO")
        FIN SI
        SI (a.motor != NULO)
            IMPRIMIR("ERROR: NO DEBE EJECUTARSE")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["MOTOR_SUBSTRUCT_NULO: VERDADERO"]);
}

#[test]
fn test_tier1_f2_self_referential_linked_node_nulo() {
    let code = r#"
    ESTRUCTURA Nodo:
        ENTERO valor
        Nodo siguiente
    FIN ESTRUCTURA

    INICIO
        Nodo n
        SI (n.valor == NULO)
            IMPRIMIR("VALOR_NULO: VERDADERO")
        FIN SI
        SI (n.siguiente == NULO)
            IMPRIMIR("SIGUIENTE_NULO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "VALOR_NULO: VERDADERO",
        "SIGUIENTE_NULO: VERDADERO",
    ]);
}

#[test]
fn test_tier1_f2_binary_tree_two_pointers_nulo() {
    let code = r#"
    ESTRUCTURA NodoArbol:
        ENTERO clave
        NodoArbol izq
        NodoArbol der
    FIN ESTRUCTURA

    INICIO
        NodoArbol raiz
        SI (raiz.izq == NULO Y raiz.der == NULO)
            IMPRIMIR("ARBOL_PUNTEROS_NULOS: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["ARBOL_PUNTEROS_NULOS: VERDADERO"]);
}

#[test]
fn test_tier1_f2_multiple_nested_struct_types_nulo() {
    let code = r#"
    ESTRUCTURA Direccion:
        CADENA calle
    FIN ESTRUCTURA

    ESTRUCTURA Contacto:
        CADENA email
    FIN ESTRUCTURA

    ESTRUCTURA Persona:
        CADENA nombre
        Direccion dir
        Contacto contacto
    FIN ESTRUCTURA

    INICIO
        Persona p
        SI (p.dir == NULO Y p.contacto == NULO)
            IMPRIMIR("MULTIPLES_ANIDADOS_NULOS: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["MULTIPLES_ANIDADOS_NULOS: VERDADERO"]);
}

#[test]
fn test_tier1_f2_mutual_reference_struct_types_nulo() {
    let code = r#"
    ESTRUCTURA NodoA:
        ENTERO id
        NodoB vinculo_b
    FIN ESTRUCTURA

    ESTRUCTURA NodoB:
        ENTERO id
        NodoA vinculo_a
    FIN ESTRUCTURA

    INICIO
        NodoA a
        SI (a.vinculo_b == NULO)
            IMPRIMIR("VINCULO_MUTUO_A_NULO: VERDADERO")
        FIN SI
        NodoB b
        SI (b.vinculo_a == NULO)
            IMPRIMIR("VINCULO_MUTUO_B_NULO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "VINCULO_MUTUO_A_NULO: VERDADERO",
        "VINCULO_MUTUO_B_NULO: VERDADERO",
    ]);
}

// --- F3: Accurate NULO equality (== NULO, == null, == None) on uninitialized fields ---

#[test]
fn test_tier1_f3_equality_canonical_nulo_keyword() {
    let code = r#"
    ESTRUCTURA Sensor:
        REAL lectura
    FIN ESTRUCTURA

    INICIO
        Sensor s
        SI (s.lectura == NULO)
            IMPRIMIR("IGUALDAD_NULO_CANONICO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["IGUALDAD_NULO_CANONICO: VERDADERO"]);
}

#[test]
fn test_tier1_f3_equality_lowercase_null_keyword() {
    let code = r#"
    ESTRUCTURA Sensor:
        REAL lectura
    FIN ESTRUCTURA

    INICIO
        Sensor s
        SI (s.lectura == null)
            IMPRIMIR("IGUALDAD_NULL_MINUSCULA: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["IGUALDAD_NULL_MINUSCULA: VERDADERO"]);
}

#[test]
fn test_tier1_f3_equality_mixedcase_none_keyword() {
    let code = r#"
    ESTRUCTURA Sensor:
        REAL lectura
    FIN ESTRUCTURA

    INICIO
        Sensor s
        SI (s.lectura == None)
            IMPRIMIR("IGUALDAD_NONE_KEYWORD: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["IGUALDAD_NONE_KEYWORD: VERDADERO"]);
}

#[test]
fn test_tier1_f3_equality_in_conditional_si_branching() {
    let code = r#"
    ESTRUCTURA Cuenta:
        ENTERO balance
    FIN ESTRUCTURA

    INICIO
        Cuenta c
        SI (c.balance == NULO)
            IMPRIMIR("RAMA_THEN_CORRECTA: VERDADERO")
        SINO
            IMPRIMIR("RAMA_SINO_INCORRECTA: FALSO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["RAMA_THEN_CORRECTA: VERDADERO"]);
}

#[test]
fn test_tier1_f3_equality_assigned_to_boolean_variable() {
    let code = r#"
    ESTRUCTURA Tarea:
        CADENA titulo
    FIN ESTRUCTURA

    INICIO
        Tarea t
        BOOLEANO es_nulo = (t.titulo == NULO)
        IMPRIMIR("ASIGNADO_A_BOOLEANO: " + es_nulo)
    FIN
    "#;
    assert_dual_engine_ok(code, &["ASIGNADO_A_BOOLEANO: VERDADERO"]);
}

// --- F4: Accurate NULO inequality (!= NULO, != null, != None) on uninitialized fields ---

#[test]
fn test_tier1_f4_inequality_canonical_nulo_evaluates_falso() {
    let code = r#"
    ESTRUCTURA Sensor:
        REAL lectura
    FIN ESTRUCTURA

    INICIO
        Sensor s
        BOOLEANO res = (s.lectura != NULO)
        IMPRIMIR("DESIGUALDAD_NULO_CANONICO: " + res)
    FIN
    "#;
    assert_dual_engine_ok(code, &["DESIGUALDAD_NULO_CANONICO: FALSO"]);
}

#[test]
fn test_tier1_f4_inequality_lowercase_null_evaluates_falso() {
    let code = r#"
    ESTRUCTURA Sensor:
        REAL lectura
    FIN ESTRUCTURA

    INICIO
        Sensor s
        BOOLEANO res = (s.lectura != null)
        IMPRIMIR("DESIGUALDAD_NULL_MINUSCULA: " + res)
    FIN
    "#;
    assert_dual_engine_ok(code, &["DESIGUALDAD_NULL_MINUSCULA: FALSO"]);
}

#[test]
fn test_tier1_f4_inequality_mixedcase_none_evaluates_falso() {
    let code = r#"
    ESTRUCTURA Sensor:
        REAL lectura
    FIN ESTRUCTURA

    INICIO
        Sensor s
        BOOLEANO res = (s.lectura != None)
        IMPRIMIR("DESIGUALDAD_NONE_KEYWORD: " + res)
    FIN
    "#;
    assert_dual_engine_ok(code, &["DESIGUALDAD_NONE_KEYWORD: FALSO"]);
}

#[test]
fn test_tier1_f4_inequality_conditional_sino_branching() {
    let code = r#"
    ESTRUCTURA Item:
        ENTERO precio
    FIN ESTRUCTURA

    INICIO
        Item it
        SI (it.precio != NULO)
            IMPRIMIR("RAMA_THEN_INCORRECTA")
        SINO
            IMPRIMIR("RAMA_SINO_CORRECTA: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["RAMA_SINO_CORRECTA: VERDADERO"]);
}

#[test]
fn test_tier1_f4_inequality_negation_with_no_operator() {
    let code = r#"
    ESTRUCTURA Item:
        CADENA etiqueta
    FIN ESTRUCTURA

    INICIO
        Item it
        SI (NO (it.etiqueta != NULO))
            IMPRIMIR("NEGACION_DESIGUALDAD_ES_VERDADERO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["NEGACION_DESIGUALDAD_ES_VERDADERO: VERDADERO"]);
}

// --- F5: Subsequent assignment of values and struct instances over NULO attributes ---

#[test]
fn test_tier1_f5_assign_primitives_entero_and_real_over_nulo() {
    let code = r#"
    ESTRUCTURA Metrica:
        ENTERO cantidad
        REAL total
    FIN ESTRUCTURA

    INICIO
        Metrica m
        SI (m.cantidad == NULO Y m.total == NULO)
            IMPRIMIR("INICIALMENTE_NULOS: VERDADERO")
        FIN SI
        m.cantidad = 42
        m.total = 123.45
        SI (m.cantidad != NULO Y m.total != NULO)
            IMPRIMIR("ASIGNADOS_NO_NULOS: VERDADERO")
        FIN SI
        IMPRIMIR("VALORES: " + m.cantidad + " Y " + m.total)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "INICIALMENTE_NULOS: VERDADERO",
        "ASIGNADOS_NO_NULOS: VERDADERO",
        "VALORES: 42 Y 123.45",
    ]);
}

#[test]
fn test_tier1_f5_assign_primitives_cadena_and_booleano_over_nulo() {
    let code = r#"
    ESTRUCTURA Perfil:
        CADENA usuario
        BOOLEANO verificado
    FIN ESTRUCTURA

    INICIO
        Perfil p
        p.usuario = "admin"
        p.verificado = VERDADERO
        IMPRIMIR("USUARIO: " + p.usuario)
        IMPRIMIR("VERIFICADO: " + p.verificado)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "USUARIO: admin",
        "VERIFICADO: VERDADERO",
    ]);
}

#[test]
fn test_tier1_f5_assign_child_struct_instance_over_nulo() {
    let code = r#"
    ESTRUCTURA Dimension:
        REAL ancho
        REAL alto
    FIN ESTRUCTURA

    ESTRUCTURA Cuadro:
        CADENA titulo
        Dimension dim
    FIN ESTRUCTURA

    INICIO
        Cuadro c
        SI (c.dim == NULO)
            IMPRIMIR("DIM_INICIAL_NULO: VERDADERO")
        FIN SI

        Dimension d
        d.ancho = 80.5
        d.alto = 60.0
        c.dim = d

        SI (c.dim != NULO)
            IMPRIMIR("DIM_ASIGNADA: VERDADERO")
            IMPRIMIR("ANCHO: " + c.dim.ancho)
            IMPRIMIR("ALTO: " + c.dim.alto)
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "DIM_INICIAL_NULO: VERDADERO",
        "DIM_ASIGNADA: VERDADERO",
        "ANCHO: 80.5",
        "ALTO: 60.0",
    ]);
}

#[test]
fn test_tier1_f5_assign_self_referential_struct_instance() {
    let code = r#"
    ESTRUCTURA Elemento:
        CADENA etiqueta
        Elemento enlace
    FIN ESTRUCTURA

    INICIO
        Elemento e1
        Elemento e2
        e1.etiqueta = "Primero"
        e2.etiqueta = "Segundo"
        e1.enlace = e2

        IMPRIMIR("E1: " + e1.etiqueta)
        IMPRIMIR("E1_SIGUIENTE: " + e1.enlace.etiqueta)
        SI (e2.enlace == NULO)
            IMPRIMIR("E2_TERMINAL_NULO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "E1: Primero",
        "E1_SIGUIENTE: Segundo",
        "E2_TERMINAL_NULO: VERDADERO",
    ]);
}

#[test]
fn test_tier1_f5_reassign_multiple_times_over_nulo() {
    let code = r#"
    ESTRUCTURA Variable:
        ENTERO valor
    FIN ESTRUCTURA

    INICIO
        Variable v
        v.valor = 10
        IMPRIMIR("PASO1: " + v.valor)
        v.valor = 20
        IMPRIMIR("PASO2: " + v.valor)
        v.valor = 30
        IMPRIMIR("PASO3: " + v.valor)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "PASO1: 10",
        "PASO2: 20",
        "PASO3: 30",
    ]);
}

// --- F6: Controlled runtime error on null dereference ---

#[test]
fn test_tier1_f6_err_read_field_on_uninitialized_nested_struct() {
    let code = r#"
    ESTRUCTURA Sub:
        ENTERO dato
    FIN ESTRUCTURA

    ESTRUCTURA Principal:
        Sub sub
    FIN ESTRUCTURA

    INICIO
        Principal p
        ENTERO x = p.sub.dato
    FIN
    "#;
    assert_dual_engine_err(code, "Intento de acceder al campo 'dato' en una referencia nula (NULO)");
}

#[test]
fn test_tier1_f6_err_write_field_on_uninitialized_nested_struct() {
    let code = r#"
    ESTRUCTURA Sub:
        ENTERO dato
    FIN ESTRUCTURA

    ESTRUCTURA Principal:
        Sub sub
    FIN ESTRUCTURA

    INICIO
        Principal p
        p.sub.dato = 55
    FIN
    "#;
    assert_dual_engine_err(code, "Intento de asignar el campo 'dato' en una referencia nula (NULO)");
}

#[test]
fn test_tier1_f6_err_read_field_on_uninitialized_self_ref() {
    let code = r#"
    ESTRUCTURA Nodo:
        ENTERO val
        Nodo sig
    FIN ESTRUCTURA

    INICIO
        Nodo n
        ENTERO v = n.sig.val
    FIN
    "#;
    assert_dual_engine_err(code, "Intento de acceder al campo 'val' en una referencia nula (NULO)");
}

#[test]
fn test_tier1_f6_err_write_field_on_uninitialized_self_ref() {
    let code = r#"
    ESTRUCTURA Nodo:
        ENTERO val
        Nodo sig
    FIN ESTRUCTURA

    INICIO
        Nodo n
        n.sig.val = 99
    FIN
    "#;
    assert_dual_engine_err(code, "Intento de asignar el campo 'val' en una referencia nula (NULO)");
}

#[test]
fn test_tier1_f6_err_direct_null_variable_field_access() {
    let code = r#"
    ESTRUCTURA Nodo:
        ENTERO val
    FIN ESTRUCTURA

    INICIO
        Nodo n = NULO
        ENTERO x = n.val
    FIN
    "#;
    assert_dual_engine_err(code, "Intento de acceder al campo 'val' en una referencia nula (NULO)");
}

// ============================================================================
// TIER 2: BOUNDARY & CORNER CASES (7 topics, 35 tests)
// ============================================================================

// --- Topic 1: Struct with 0 fields or only primitive fields ---

#[test]
fn test_tier2_b1_empty_struct_definition_and_instantiation() {
    let code = r#"
    ESTRUCTURA Vacia:
    FIN ESTRUCTURA

    INICIO
        Vacia v
        IMPRIMIR("VACIA_CREADA: " + v)
    FIN
    "#;
    assert_dual_engine_ok(code, &["VACIA_CREADA: Vacia{}"]);
}

#[test]
fn test_tier2_b1_struct_single_primitive_entero() {
    let code = r#"
    ESTRUCTURA UnicoEntero:
        ENTERO solo_entero
    FIN ESTRUCTURA

    INICIO
        UnicoEntero u
        SI (u.solo_entero == NULO)
            IMPRIMIR("SOLO_ENTERO_NULO: VERDADERO")
        FIN SI
        u.solo_entero = 1001
        IMPRIMIR("SOLO_ENTERO_VALOR: " + u.solo_entero)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "SOLO_ENTERO_NULO: VERDADERO",
        "SOLO_ENTERO_VALOR: 1001",
    ]);
}

#[test]
fn test_tier2_b1_struct_single_primitive_cadena() {
    let code = r#"
    ESTRUCTURA UnicaCadena:
        CADENA solo_texto
    FIN ESTRUCTURA

    INICIO
        UnicaCadena u
        SI (u.solo_texto == NULO)
            IMPRIMIR("SOLO_TEXTO_NULO: VERDADERO")
        FIN SI
        u.solo_texto = "Yaguareté 0.2.1"
        IMPRIMIR("SOLO_TEXTO_VALOR: " + u.solo_texto)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "SOLO_TEXTO_NULO: VERDADERO",
        "SOLO_TEXTO_VALOR: Yaguareté 0.2.1",
    ]);
}

#[test]
fn test_tier2_b1_struct_single_primitive_booleano() {
    let code = r#"
    ESTRUCTURA UnicoBool:
        BOOLEANO solo_bool
    FIN ESTRUCTURA

    INICIO
        UnicoBool u
        SI (u.solo_bool == NULO)
            IMPRIMIR("SOLO_BOOL_NULO: VERDADERO")
        FIN SI
        u.solo_bool = VERDADERO
        IMPRIMIR("SOLO_BOOL_VALOR: " + u.solo_bool)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "SOLO_BOOL_NULO: VERDADERO",
        "SOLO_BOOL_VALOR: VERDADERO",
    ]);
}

#[test]
fn test_tier2_b1_struct_only_primitives_partial_assignment() {
    let code = r#"
    ESTRUCTURA DatosTrio:
        ENTERO a
        REAL b
        CADENA c
    FIN ESTRUCTURA

    INICIO
        DatosTrio d
        d.b = 3.1416
        SI (d.a == NULO Y d.c == NULO)
            IMPRIMIR("RESTANTES_NULOS: VERDADERO")
        FIN SI
        SI (d.b != NULO)
            IMPRIMIR("ASIGNADO_VALIDO: " + d.b)
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "RESTANTES_NULOS: VERDADERO",
        "ASIGNADO_VALIDO: 3.1416",
    ]);
}

// --- Topic 2: Deeply nested structs (3+ levels: A -> B -> C -> D) ---

#[test]
fn test_tier2_b2_deep_nesting_4_levels_all_nulo_initial() {
    let code = r#"
    ESTRUCTURA NivelD:
        ENTERO valor_final
    FIN ESTRUCTURA

    ESTRUCTURA NivelC:
        NivelD d
    FIN ESTRUCTURA

    ESTRUCTURA NivelB:
        NivelC c
    FIN ESTRUCTURA

    ESTRUCTURA NivelA:
        NivelB b
    FIN ESTRUCTURA

    INICIO
        NivelA a
        SI (a.b == NULO)
            IMPRIMIR("NIVEL_B_INICIAL_NULO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["NIVEL_B_INICIAL_NULO: VERDADERO"]);
}

#[test]
fn test_tier2_b2_deep_nesting_instantiate_level_by_level() {
    let code = r#"
    ESTRUCTURA NivelD:
        ENTERO valor_final
    FIN ESTRUCTURA

    ESTRUCTURA NivelC:
        NivelD d
    FIN ESTRUCTURA

    ESTRUCTURA NivelB:
        NivelC c
    FIN ESTRUCTURA

    ESTRUCTURA NivelA:
        NivelB b
    FIN ESTRUCTURA

    INICIO
        NivelA a
        NivelB b
        a.b = b
        SI (a.b != NULO Y a.b.c == NULO)
            IMPRIMIR("NIVEL_B_ASIGNADO_PERO_C_NULO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["NIVEL_B_ASIGNADO_PERO_C_NULO: VERDADERO"]);
}

#[test]
fn test_tier2_b2_deep_nesting_full_path_population() {
    let code = r#"
    ESTRUCTURA NivelD:
        ENTERO valor_final
    FIN ESTRUCTURA

    ESTRUCTURA NivelC:
        NivelD d
    FIN ESTRUCTURA

    ESTRUCTURA NivelB:
        NivelC c
    FIN ESTRUCTURA

    ESTRUCTURA NivelA:
        NivelB b
    FIN ESTRUCTURA

    INICIO
        NivelA a
        NivelB b
        NivelC c
        NivelD d
        d.valor_final = 777
        c.d = d
        b.c = c
        a.b = b

        IMPRIMIR("VALOR_PROFUNDO: " + a.b.c.d.valor_final)
    FIN
    "#;
    assert_dual_engine_ok(code, &["VALOR_PROFUNDO: 777"]);
}

#[test]
fn test_tier2_b2_deep_nesting_partial_leaves_subfields_nulo() {
    let code = r#"
    ESTRUCTURA NivelD:
        ENTERO val_d
    FIN ESTRUCTURA

    ESTRUCTURA NivelC:
        NivelD d
        CADENA etiqueta_c
    FIN ESTRUCTURA

    ESTRUCTURA NivelB:
        NivelC c
    FIN ESTRUCTURA

    INICIO
        NivelB b
        NivelC c
        c.etiqueta_c = "Intermedio"
        b.c = c

        SI (b.c.etiqueta_c != NULO Y b.c.d == NULO)
            IMPRIMIR("ETIQUETA_PRESENTE_D_NULO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["ETIQUETA_PRESENTE_D_NULO: VERDADERO"]);
}

#[test]
fn test_tier2_b2_deep_nesting_deref_at_level_3_fails() {
    let code = r#"
    ESTRUCTURA NivelD:
        ENTERO val_d
    FIN ESTRUCTURA

    ESTRUCTURA NivelC:
        NivelD d
    FIN ESTRUCTURA

    ESTRUCTURA NivelB:
        NivelC c
    FIN ESTRUCTURA

    INICIO
        NivelB b
        NivelC c
        b.c = c
        // c.d sigue siendo NULO, acceder a c.d.val_d debe fallar
        ENTERO x = b.c.d.val_d
    FIN
    "#;
    assert_dual_engine_err(code, "Intento de acceder al campo 'val_d' en una referencia nula (NULO)");
}

// --- Topic 3: Vectors/arrays of structs with unassigned elements ---

#[test]
fn test_tier2_b3_vector_struct_explicit_null_elements() {
    let code = r#"
    ESTRUCTURA Auto:
        CADENA marca
        ENTERO anio
    FIN ESTRUCTURA

    INICIO
        Auto vec[3] = [NULO, NULO, NULO]
        SI (vec[0] == NULO Y vec[1] == NULO Y vec[2] == NULO)
            IMPRIMIR("TODOS_ELEMENTOS_NULOS: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["TODOS_ELEMENTOS_NULOS: VERDADERO"]);
}

#[test]
fn test_tier2_b3_vector_struct_populate_single_element() {
    let code = r#"
    ESTRUCTURA Auto:
        CADENA marca
    FIN ESTRUCTURA

    INICIO
        Auto vec[2] = [NULO, NULO]
        Auto a
        a.marca = "Toyota"
        vec[0] = a

        SI (vec[0] != NULO Y vec[1] == NULO)
            IMPRIMIR("VEC_0_POPULADO_VEC_1_NULO: VERDADERO")
            IMPRIMIR("MARCA: " + vec[0].marca)
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "VEC_0_POPULADO_VEC_1_NULO: VERDADERO",
        "MARCA: Toyota",
    ]);
}

#[test]
fn test_tier2_b3_vector_struct_loop_population() {
    let code = r#"
    ESTRUCTURA NodoDato:
        ENTERO id
        CADENA texto
    FIN ESTRUCTURA

    INICIO
        NodoDato lista[3] = [NULO, NULO, NULO]
        PARA (ENTERO i = 0; i < 3; i++)
            NodoDato n
            n.id = i + 1
            lista[i] = n
        FIN PARA

        SI (lista[0].id == 1 Y lista[0].texto == NULO)
            IMPRIMIR("ELEMENTO_0_VALIDO: VERDADERO")
        FIN SI
        SI (lista[2].id == 3 Y lista[2].texto == NULO)
            IMPRIMIR("ELEMENTO_2_VALIDO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "ELEMENTO_0_VALIDO: VERDADERO",
        "ELEMENTO_2_VALIDO: VERDADERO",
    ]);
}

#[test]
fn test_tier2_b3_vector_struct_mutate_element_fields() {
    let code = r#"
    ESTRUCTURA Casilla:
        ENTERO numero
        CADENA valor
    FIN ESTRUCTURA

    INICIO
        Casilla tablero[2] = [NULO, NULO]
        Casilla c0
        Casilla c1
        c0.numero = 10
        c1.numero = 20
        tablero[0] = c0
        tablero[1] = c1

        tablero[0].valor = "Ficha Roja"
        tablero[1].valor = "Ficha Azul"

        IMPRIMIR("C0: " + tablero[0].numero + " " + tablero[0].valor)
        IMPRIMIR("C1: " + tablero[1].numero + " " + tablero[1].valor)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "C0: 10 Ficha Roja",
        "C1: 20 Ficha Azul",
    ]);
}

#[test]
fn test_tier2_b3_vector_struct_reassign_element_to_null() {
    let code = r#"
    ESTRUCTURA Paquete:
        CADENA codigo
    FIN ESTRUCTURA

    INICIO
        Paquete p
        p.codigo = "PKT-99"
        Paquete vec[2] = [p, NULO]

        SI (vec[0] != NULO)
            IMPRIMIR("VEC_0_INICIAL: " + vec[0].codigo)
        FIN SI

        vec[0] = NULO
        SI (vec[0] == NULO)
            IMPRIMIR("VEC_0_REASIGNADO_NULO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "VEC_0_INICIAL: PKT-99",
        "VEC_0_REASIGNADO_NULO: VERDADERO",
    ]);
}

// --- Topic 4: Comparing uninitialized attribute with 0, "", false ---

#[test]
fn test_tier2_b4_compare_uninitialized_entero_with_zero() {
    let code = r#"
    ESTRUCTURA Cuenta:
        ENTERO balance
    FIN ESTRUCTURA

    INICIO
        Cuenta c
        SI (c.balance == 0)
            IMPRIMIR("ERROR: NO DEBE SER IGUAL A CERO")
        SINO
            IMPRIMIR("ENTERO_NULO_DISTINTO_DE_CERO: VERDADERO")
        FIN SI
        SI (c.balance != 0)
            IMPRIMIR("ENTERO_NULO_NO_ES_CERO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "ENTERO_NULO_DISTINTO_DE_CERO: VERDADERO",
        "ENTERO_NULO_NO_ES_CERO: VERDADERO",
    ]);
}

#[test]
fn test_tier2_b4_compare_uninitialized_real_with_zero_float() {
    let code = r#"
    ESTRUCTURA Medida:
        REAL temperatura
    FIN ESTRUCTURA

    INICIO
        Medida m
        SI (m.temperatura == 0.0)
            IMPRIMIR("ERROR: NO DEBE SER 0.0")
        SINO
            IMPRIMIR("REAL_NULO_DISTINTO_DE_CERO: VERDADERO")
        FIN SI
        SI (m.temperatura != 0.0)
            IMPRIMIR("REAL_NULO_NO_ES_CERO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "REAL_NULO_DISTINTO_DE_CERO: VERDADERO",
        "REAL_NULO_NO_ES_CERO: VERDADERO",
    ]);
}

#[test]
fn test_tier2_b4_compare_uninitialized_cadena_with_empty_string() {
    let code = r#"
    ESTRUCTURA Mensaje:
        CADENA cuerpo
    FIN ESTRUCTURA

    INICIO
        Mensaje m
        SI (m.cuerpo == "")
            IMPRIMIR("ERROR: NO DEBE SER CADENA VACIA")
        SINO
            IMPRIMIR("CADENA_NULO_DISTINTO_DE_VACIO: VERDADERO")
        FIN SI
        SI (m.cuerpo != "")
            IMPRIMIR("CADENA_NULO_NO_ES_VACIO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "CADENA_NULO_DISTINTO_DE_VACIO: VERDADERO",
        "CADENA_NULO_NO_ES_VACIO: VERDADERO",
    ]);
}

#[test]
fn test_tier2_b4_compare_uninitialized_booleano_with_falso() {
    let code = r#"
    ESTRUCTURA Bandera:
        BOOLEANO activa
    FIN ESTRUCTURA

    INICIO
        Bandera b
        SI (b.activa == FALSO)
            IMPRIMIR("ERROR: NO DEBE SER FALSO")
        SINO
            IMPRIMIR("BOOLEANO_NULO_DISTINTO_DE_FALSO: VERDADERO")
        FIN SI
        SI (b.activa != FALSO)
            IMPRIMIR("BOOLEANO_NULO_NO_ES_FALSO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "BOOLEANO_NULO_DISTINTO_DE_FALSO: VERDADERO",
        "BOOLEANO_NULO_NO_ES_FALSO: VERDADERO",
    ]);
}

#[test]
fn test_tier2_b4_compare_uninitialized_caracter_with_space_or_zero() {
    let code = r#"
    ESTRUCTURA Simbolo:
        CARACTER letra
    FIN ESTRUCTURA

    INICIO
        Simbolo s
        SI (s.letra == " ")
            IMPRIMIR("ERROR: NO DEBE SER ESPACIO")
        SINO
            IMPRIMIR("CARACTER_NULO_DISTINTO_ESPACIO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["CARACTER_NULO_DISTINTO_ESPACIO: VERDADERO"]);
}

// --- Topic 5: Reassigning field back to NULO (obj.campo = NULO) ---

#[test]
fn test_tier2_b5_reassign_primitive_int_back_to_nulo() {
    let code = r#"
    ESTRUCTURA Item:
        ENTERO stock
    FIN ESTRUCTURA

    INICIO
        Item it
        it.stock = 50
        IMPRIMIR("CON_STOCK: " + it.stock)
        it.stock = NULO
        SI (it.stock == NULO)
            IMPRIMIR("STOCK_REASIGNADO_NULO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "CON_STOCK: 50",
        "STOCK_REASIGNADO_NULO: VERDADERO",
    ]);
}

#[test]
fn test_tier2_b5_reassign_primitive_cadena_back_to_nulo() {
    let code = r#"
    ESTRUCTURA Nota:
        CADENA texto
    FIN ESTRUCTURA

    INICIO
        Nota n
        n.texto = "Importante"
        IMPRIMIR("TEXTO_PREVIO: " + n.texto)
        n.texto = NULO
        SI (n.texto == NULO)
            IMPRIMIR("TEXTO_REASIGNADO_NULO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "TEXTO_PREVIO: Importante",
        "TEXTO_REASIGNADO_NULO: VERDADERO",
    ]);
}

#[test]
fn test_tier2_b5_reassign_nested_struct_back_to_nulo() {
    let code = r#"
    ESTRUCTURA Hijo:
        ENTERO id
    FIN ESTRUCTURA

    ESTRUCTURA Padre:
        Hijo hijo
    FIN ESTRUCTURA

    INICIO
        Padre p
        Hijo h
        h.id = 10
        p.hijo = h
        IMPRIMIR("HIJO_ID: " + p.hijo.id)

        p.hijo = NULO
        SI (p.hijo == NULO)
            IMPRIMIR("HIJO_REASIGNADO_NULO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "HIJO_ID: 10",
        "HIJO_REASIGNADO_NULO: VERDADERO",
    ]);
}

#[test]
fn test_tier2_b5_reassign_back_to_nulo_then_deref_fails() {
    let code = r#"
    ESTRUCTURA Hijo:
        ENTERO id
    FIN ESTRUCTURA

    ESTRUCTURA Padre:
        Hijo hijo
    FIN ESTRUCTURA

    INICIO
        Padre p
        Hijo h
        h.id = 10
        p.hijo = h
        p.hijo = NULO
        // Acceder a p.hijo.id ahora debe fallar
        ENTERO x = p.hijo.id
    FIN
    "#;
    assert_dual_engine_err(code, "Intento de acceder al campo 'id' en una referencia nula (NULO)");
}

#[test]
fn test_tier2_b5_reassign_back_to_nulo_and_then_reassign_value() {
    let code = r#"
    ESTRUCTURA Contador:
        ENTERO valor
    FIN ESTRUCTURA

    INICIO
        Contador c
        c.valor = 100
        c.valor = NULO
        SI (c.valor == NULO)
            IMPRIMIR("INTERMEDIO_NULO: VERDADERO")
        FIN SI
        c.valor = 200
        IMPRIMIR("FINAL_VALOR: " + c.valor)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "INTERMEDIO_NULO: VERDADERO",
        "FINAL_VALOR: 200",
    ]);
}

// --- Topic 6: Independent instances mutating independently without sharing state ---

#[test]
fn test_tier2_b6_independent_instances_primitives() {
    let code = r#"
    ESTRUCTURA Coordenada:
        ENTERO coord_x
        ENTERO coord_y
    FIN ESTRUCTURA

    INICIO
        Coordenada p1
        Coordenada p2
        p1.coord_x = 10
        p1.coord_y = 20

        SI (p2.coord_x == NULO Y p2.coord_y == NULO)
            IMPRIMIR("P2_INTACTO_NULO: VERDADERO")
        FIN SI
        IMPRIMIR("P1: " + p1.coord_x + "," + p1.coord_y)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "P2_INTACTO_NULO: VERDADERO",
        "P1: 10,20",
    ]);
}

#[test]
fn test_tier2_b6_independent_instances_nested_struct() {
    let code = r#"
    ESTRUCTURA Motor:
        CADENA tipo
    FIN ESTRUCTURA

    ESTRUCTURA Auto:
        Motor motor
    FIN ESTRUCTURA

    INICIO
        Auto a1
        Auto a2
        Motor m
        m.tipo = "Electrico"
        a1.motor = m

        SI (a2.motor == NULO)
            IMPRIMIR("A2_MOTOR_SIGUE_NULO: VERDADERO")
        FIN SI
        IMPRIMIR("A1_MOTOR: " + a1.motor.tipo)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "A2_MOTOR_SIGUE_NULO: VERDADERO",
        "A1_MOTOR: Electrico",
    ]);
}

#[test]
fn test_tier2_b6_independent_instances_different_values() {
    let code = r#"
    ESTRUCTURA Contador:
        ENTERO n
    FIN ESTRUCTURA

    INICIO
        Contador c1
        Contador c2
        c1.n = 100
        c2.n = 200
        SI (c1.n != c2.n)
            IMPRIMIR("MUTACION_INDEPENDIENTE: VERDADERO")
            IMPRIMIR("C1: " + c1.n + " C2: " + c2.n)
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "MUTACION_INDEPENDIENTE: VERDADERO",
        "C1: 100 C2: 200",
    ]);
}

#[test]
fn test_tier2_b6_independent_instances_three_instances() {
    let code = r#"
    ESTRUCTURA Item:
        CADENA cod
    FIN ESTRUCTURA

    INICIO
        Item i1
        Item i2
        Item i3
        i2.cod = "ITEM-B"

        SI (i1.cod == NULO Y i3.cod == NULO)
            IMPRIMIR("I1_Y_I3_NULOS: VERDADERO")
        FIN SI
        IMPRIMIR("I2: " + i2.cod)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "I1_Y_I3_NULOS: VERDADERO",
        "I2: ITEM-B",
    ]);
}

#[test]
fn test_tier2_b6_independent_instances_reference_identity() {
    let code = r#"
    ESTRUCTURA Punto:
        ENTERO coord_x
    FIN ESTRUCTURA

    INICIO
        Punto p1
        Punto p2
        SI (p1 != p2)
            IMPRIMIR("INSTANCIAS_DISTINTAS: VERDADERO")
        FIN SI
        Punto p3 = p1
        SI (p1 == p3)
            IMPRIMIR("ALIAS_REFERENCIA_IGUALES: VERDADERO")
        FIN SI
        p3.coord_x = 99
        SI (p1.coord_x == 99 Y p2.coord_x == NULO)
            IMPRIMIR("MUTACION_COMPARTIDA_Y_P2_INTACTO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "INSTANCIAS_DISTINTAS: VERDADERO",
        "ALIAS_REFERENCIA_IGUALES: VERDADERO",
        "MUTACION_COMPARTIDA_Y_P2_INTACTO: VERDADERO",
    ]);
}

// --- Topic 7: Functions receiving uninitialized structs and returning structs ---

#[test]
fn test_tier2_b7_function_receives_uninitialized_struct_checks_null() {
    let code = r#"
    ESTRUCTURA Caja:
        ENTERO dato
    FIN ESTRUCTURA

    FUNCION esta_vacia(c: Caja): BOOLEANO
        RETORNAR (c.dato == NULO)
    FIN FUNCION

    INICIO
        Caja mi_caja
        BOOLEANO vacia = esta_vacia(mi_caja)
        IMPRIMIR("FUNCION_RECIBE_UNINIT: " + vacia)
    FIN
    "#;
    assert_dual_engine_ok(code, &["FUNCION_RECIBE_UNINIT: VERDADERO"]);
}

#[test]
fn test_tier2_b7_function_returns_uninitialized_struct() {
    let code = r#"
    ESTRUCTURA Caja:
        ENTERO dato
        CADENA etiqueta
    FIN ESTRUCTURA

    FUNCION crear_caja_vacia(): Caja
        Caja c
        RETORNAR c
    FIN FUNCION

    INICIO
        Caja ret = crear_caja_vacia()
        SI (ret.dato == NULO Y ret.etiqueta == NULO)
            IMPRIMIR("FUNCION_RETORNA_UNINIT: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &["FUNCION_RETORNA_UNINIT: VERDADERO"]);
}

#[test]
fn test_tier2_b7_function_factory_initializes_fields() {
    let code = r#"
    ESTRUCTURA Persona:
        CADENA nombre
        ENTERO edad
    FIN ESTRUCTURA

    FUNCION inicializar_persona(p: Persona, n: CADENA, e: ENTERO): Persona
        p.nombre = n
        p.edad = e
        RETORNAR p
    FIN FUNCION

    INICIO
        Persona uninit
        Persona lista = inicializar_persona(uninit, "Sofia", 28)
        IMPRIMIR("NOMBRE: " + lista.nombre)
        IMPRIMIR("EDAD: " + lista.edad)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "NOMBRE: Sofia",
        "EDAD: 28",
    ]);
}

#[test]
fn test_tier2_b7_function_chain_passing_struct() {
    let code = r#"
    ESTRUCTURA Documento:
        CADENA titulo
        ENTERO paginas
    FIN ESTRUCTURA

    FUNCION paso1(d: Documento): Documento
        SI (d.titulo == NULO)
            d.titulo = "Borrador"
        FIN SI
        RETORNAR d
    FIN FUNCION

    FUNCION paso2(d: Documento): Documento
        SI (d.paginas == NULO)
            d.paginas = 10
        FIN SI
        RETORNAR d
    FIN FUNCION

    INICIO
        Documento doc
        Documento doc1 = paso1(doc)
        Documento doc2 = paso2(doc1)
        IMPRIMIR("TITULO: " + doc2.titulo)
        IMPRIMIR("PAGINAS: " + doc2.paginas)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "TITULO: Borrador",
        "PAGINAS: 10",
    ]);
}

#[test]
fn test_tier2_b7_procedure_mutates_struct_fields() {
    let code = r#"
    ESTRUCTURA Contador:
        ENTERO cuenta
    FIN ESTRUCTURA

    PROCEDIMIENTO configurar(c: Contador, v: ENTERO)
        SI (c.cuenta == NULO)
            c.cuenta = v
        FIN SI
    FIN PROCEDIMIENTO

    INICIO
        Contador cont
        SI (cont.cuenta == NULO)
            IMPRIMIR("PREVIO_NULO: VERDADERO")
        FIN SI
        configurar(cont, 42)
        IMPRIMIR("POST_CUENTA: " + cont.cuenta)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "PREVIO_NULO: VERDADERO",
        "POST_CUENTA: 42",
    ]);
}

// ============================================================================
// TIER 3: CROSS-FEATURE COMBINATIONS (8 tests)
// ============================================================================

#[test]
fn test_tier3_c1_field_assignment_followed_by_nulo_check_and_branching() {
    let code = r#"
    ESTRUCTURA Usuario:
        CADENA nombre
        CADENA rol
    FIN ESTRUCTURA

    INICIO
        Usuario u
        u.nombre = "Martin"
        // u.rol permanece NULO
        SI (u.nombre != NULO Y u.rol == NULO)
            IMPRIMIR("USUARIO_PARCIAL_VALIDADO: VERDADERO")
        SINO
            IMPRIMIR("ERROR_ESTADO")
        FIN SI
        u.rol = "Invitado"
        SI (u.nombre != NULO Y u.rol != NULO)
            IMPRIMIR("USUARIO_COMPLETO_VALIDADO: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "USUARIO_PARCIAL_VALIDADO: VERDADERO",
        "USUARIO_COMPLETO_VALIDADO: VERDADERO",
    ]);
}

#[test]
fn test_tier3_c2_nested_struct_instantiation_inner_assignment_while_traversal() {
    let code = r#"
    ESTRUCTURA Nodo:
        ENTERO val
        Nodo sig
    FIN ESTRUCTURA

    INICIO
        Nodo n1
        Nodo n2
        Nodo n3
        n1.val = 10
        n2.val = 20
        n3.val = 30
        n1.sig = n2
        n2.sig = n3
        // n3.sig es NULO por defecto

        ENTERO suma = 0
        Nodo actual = n1
        MIENTRAS (actual != NULO)
            suma = suma + actual.val
            actual = actual.sig
        FIN MIENTRAS
        IMPRIMIR("SUMA_NODOS: " + suma)
    FIN
    "#;
    assert_dual_engine_ok(code, &["SUMA_NODOS: 60"]);
}

#[test]
fn test_tier3_c3_vector_of_structs_while_loop_populating_nulo() {
    let code = r#"
    ESTRUCTURA Slot:
        ENTERO id
        BOOLEANO asignado
    FIN ESTRUCTURA

    INICIO
        Slot slots[3] = [NULO, NULO, NULO]
        ENTERO idx = 0
        MIENTRAS (idx < 3)
            SI (slots[idx] == NULO)
                Slot s
                s.id = idx + 100
                s.asignado = VERDADERO
                slots[idx] = s
            FIN SI
            idx++
        FIN MIENTRAS

        IMPRIMIR("SLOT0: " + slots[0].id + " " + slots[0].asignado)
        IMPRIMIR("SLOT1: " + slots[1].id + " " + slots[1].asignado)
        IMPRIMIR("SLOT2: " + slots[2].id + " " + slots[2].asignado)
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "SLOT0: 100 VERDADERO",
        "SLOT1: 101 VERDADERO",
        "SLOT2: 102 VERDADERO",
    ]);
}

#[test]
fn test_tier3_c4_function_returning_uninitialized_struct_caller_assigns_and_verifies() {
    let code = r#"
    ESTRUCTURA Cliente:
        CADENA razon_social
        ENTERO cuit
    FIN ESTRUCTURA

    FUNCION factory_cliente(): Cliente
        Cliente c
        RETORNAR c
    FIN FUNCION

    INICIO
        Cliente nuevo = factory_cliente()
        SI (nuevo.razon_social == NULO Y nuevo.cuit == NULO)
            IMPRIMIR("CLIENTE_INICIAL_NULO: VERDADERO")
        FIN SI
        nuevo.razon_social = "Acme Corp"
        nuevo.cuit = 20304050
        SI (nuevo.razon_social != NULO Y nuevo.cuit != NULO)
            IMPRIMIR("CLIENTE_POPULADO: " + nuevo.razon_social + " " + nuevo.cuit)
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "CLIENTE_INICIAL_NULO: VERDADERO",
        "CLIENTE_POPULADO: Acme Corp 20304050",
    ]);
}

#[test]
fn test_tier3_c5_struct_equality_comparison_uninitialized_vs_assigned() {
    let code = r#"
    ESTRUCTURA Punto:
        ENTERO coord_x
        ENTERO coord_y
    FIN ESTRUCTURA

    INICIO
        Punto p1
        Punto p2
        // Dos instancias separadas son distintas por identidad de referencia
        SI (p1 != p2)
            IMPRIMIR("DISTINTAS_INSTANCIAS: VERDADERO")
        FIN SI

        // Comprobación de campos uninit
        SI (p1.coord_x == p2.coord_x Y p1.coord_y == p2.coord_y)
            IMPRIMIR("CAMPOS_AMBOS_NULOS: VERDADERO")
        FIN SI

        p1.coord_x = 5
        SI (p1.coord_x != p2.coord_x)
            IMPRIMIR("CAMPO_MUTADO_DESIGUAL: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "DISTINTAS_INSTANCIAS: VERDADERO",
        "CAMPOS_AMBOS_NULOS: VERDADERO",
        "CAMPO_MUTADO_DESIGUAL: VERDADERO",
    ]);
}

#[test]
fn test_tier3_c6_null_dereference_caught_inside_function() {
    let code = r#"
    ESTRUCTURA Interno:
        ENTERO id
    FIN ESTRUCTURA

    ESTRUCTURA Envoltorio:
        Interno interno
    FIN ESTRUCTURA

    FUNCION leer_id_interno(e: Envoltorio): ENTERO
        RETORNAR e.interno.id
    FIN FUNCION

    INICIO
        Envoltorio env
        ENTERO res = leer_id_interno(env)
    FIN
    "#;
    assert_dual_engine_err(code, "Intento de acceder al campo 'id' en una referencia nula (NULO)");
}

#[test]
fn test_tier3_c7_pairwise_reassign_nulo_and_branching_in_loop() {
    let code = r#"
    ESTRUCTURA Recurso:
        ENTERO usos
    FIN ESTRUCTURA

    INICIO
        Recurso r
        r.usos = 0
        ENTERO nulos_detectados = 0

        PARA (ENTERO vuelta = 1; vuelta <= 4; vuelta++)
            SI (vuelta % 2 == 0)
                r.usos = NULO
            SINO
                r.usos = vuelta
            FIN SI

            SI (r.usos == NULO)
                nulos_detectados++
            FIN SI
        FIN PARA

        IMPRIMIR("NULOS_EN_LOOP: " + nulos_detectados)
    FIN
    "#;
    assert_dual_engine_ok(code, &["NULOS_EN_LOOP: 2"]);
}

#[test]
fn test_tier3_c8_binary_search_tree_leaf_insertion_and_traversal() {
    let code = r#"
    ESTRUCTURA NodoArbol:
        ENTERO clave
        NodoArbol izq
        NodoArbol der
    FIN ESTRUCTURA

    INICIO
        NodoArbol raiz
        raiz.clave = 50

        // Inserción hoja izquierda condicional comprobando NULO
        SI (raiz.izq == NULO)
            NodoArbol hijo_izq
            hijo_izq.clave = 25
            raiz.izq = hijo_izq
        FIN SI

        // Inserción hoja derecha condicional comprobando NULO
        SI (raiz.der == NULO)
            NodoArbol hijo_der
            hijo_der.clave = 75
            raiz.der = hijo_der
        FIN SI

        IMPRIMIR("RAIZ: " + raiz.clave)
        IMPRIMIR("IZQ: " + raiz.izq.clave)
        IMPRIMIR("DER: " + raiz.der.clave)
        SI (raiz.izq.izq == NULO Y raiz.der.der == NULO)
            IMPRIMIR("HOJAS_TERMINALES_NULAS: VERDADERO")
        FIN SI
    FIN
    "#;
    assert_dual_engine_ok(code, &[
        "RAIZ: 50",
        "IZQ: 25",
        "DER: 75",
        "HOJAS_TERMINALES_NULAS: VERDADERO",
    ]);
}
