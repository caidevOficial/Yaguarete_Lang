use yaguarete_core::{run_code, run_code_vm};

// Helper to assert both AST and VM produce exact same output lines
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

// Helper to assert both AST and VM fail with expected error substring
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
fn test_adv_deeply_nested_structures_initialization_and_linking() {
    let code = r#"
    ESTRUCTURA NivelD:
        ENTERO val_d
        CADENA tag_d
    FIN ESTRUCTURA

    ESTRUCTURA NivelC:
        NivelD d
        REAL val_c
    FIN ESTRUCTURA

    ESTRUCTURA NivelB:
        NivelC c
        BOOLEANO flag_b
    FIN ESTRUCTURA

    ESTRUCTURA NivelA:
        NivelB b
        CADENA tag_a
    FIN ESTRUCTURA

    INICIO
        NivelA a
        IMPRIMIR("A_TAG_NULO: " + (a.tag_a == NULO))
        IMPRIMIR("A_B_NULO: " + (a.b == NULO))
        IMPRIMIR("A_B_NO_NULO: " + (a.b != NULO))
        IMPRIMIR("A_CANONICA_INIT: " + a)

        // Asignación progresiva de niveles
        NivelB b_inst
        IMPRIMIR("B_C_PREVIO_NULO: " + (b_inst.c == NULO))
        a.b = b_inst
        IMPRIMIR("A_B_AHORA_VALIDO: " + (a.b != NULO))
        IMPRIMIR("A_B_C_NULO: " + (a.b.c == NULO))

        NivelC c_inst
        c_inst.val_c = 3.14
        a.b.c = c_inst
        IMPRIMIR("A_B_C_VAL: " + a.b.c.val_c)
        IMPRIMIR("A_B_C_D_NULO: " + (a.b.c.d == NULO))

        NivelD d_inst
        d_inst.val_d = 777
        d_inst.tag_d = "Fondo"
        a.b.c.d = d_inst
        IMPRIMIR("A_B_C_D_VAL: " + a.b.c.d.val_d)
        IMPRIMIR("A_B_C_D_TAG: " + a.b.c.d.tag_d)
        IMPRIMIR("A_CANONICA_FULL: " + a)
    FIN
    "#;

    assert_dual_engine_ok(
        code,
        &[
            "A_TAG_NULO: VERDADERO",
            "A_B_NULO: VERDADERO",
            "A_B_NO_NULO: FALSO",
            "A_CANONICA_INIT: NivelA{b: NULO, tag_a: NULO}",
            "B_C_PREVIO_NULO: VERDADERO",
            "A_B_AHORA_VALIDO: VERDADERO",
            "A_B_C_NULO: VERDADERO",
            "A_B_C_VAL: 3.14",
            "A_B_C_D_NULO: VERDADERO",
            "A_B_C_D_VAL: 777",
            "A_B_C_D_TAG: Fondo",
            "A_CANONICA_FULL: NivelA{b: NivelB{c: NivelC{d: NivelD{val_d: 777, tag_d: 'Fondo'}, val_c: 3.14}, flag_b: NULO}, tag_a: NULO}",
        ],
    );
}

#[test]
fn test_adv_deeply_nested_uninitialized_member_access_errors() {
    let code_read = r#"
    ESTRUCTURA NivelD:
        ENTERO val_d
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
        ENTERO x = a.b.c.d.val_d
    FIN
    "#;
    assert_dual_engine_err(code_read, "Intento de acceder al campo 'c' en una referencia nula (NULO)");

    let code_write = r#"
    ESTRUCTURA NivelD:
        ENTERO val_d
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
        a.b.c = NULO
    FIN
    "#;
    assert_dual_engine_err(code_write, "Intento de asignar el campo 'c' en una referencia nula (NULO)");
}

#[test]
fn test_adv_reassignment_cycles_canonical_string_and_break() {
    let code = r#"
    ESTRUCTURA Nodo:
        CADENA id
        Nodo sig
    FIN ESTRUCTURA

    INICIO
        Nodo n1
        Nodo n2
        Nodo n3
        n1.id = "N1"
        n2.id = "N2"
        n3.id = "N3"

        // Formar ciclo: n1 -> n2 -> n3 -> n1
        n1.sig = n2
        n2.sig = n3
        n3.sig = n1

        // Impresión canónica con ciclo: no debe desbordar la pila ni colgarse
        IMPRIMIR("CICLICO: " + n1)

        // Romper el ciclo seteando a NULO
        n3.sig = NULO

        // Recorrido lineal seguro
        Nodo actual = n1
        ENTERO visitados = 0
        MIENTRAS (actual != NULO)
            visitados = visitados + 1
            IMPRIMIR("NODO: " + actual.id)
            actual = actual.sig
        FIN MIENTRAS
        IMPRIMIR("TOTAL_VISITADOS: " + visitados)

        // Reasignación completa sobre n2
        n2 = NULO
        IMPRIMIR("N2_ES_NULO: " + (n2 == NULO))
        // La referencia en n1.sig sigue siendo accesible por semántica de conteo de referencias
        IMPRIMIR("N1_SIG_SIGUE_VIVO: " + n1.sig.id)
    FIN
    "#;

    assert_dual_engine_ok(
        code,
        &[
            "CICLICO: Nodo{id: 'N1', sig: Nodo{id: 'N2', sig: Nodo{id: 'N3', sig: <Nodo ...>}}}",
            "NODO: N1",
            "NODO: N2",
            "NODO: N3",
            "TOTAL_VISITADOS: 3",
            "N2_ES_NULO: VERDADERO",
            "N1_SIG_SIGUE_VIVO: N2",
        ],
    );
}

#[test]
fn test_adv_vector_of_nested_structs_manipulation_ast() {
    let code = r#"
    ESTRUCTURA Coordenada:
        REAL posX
        REAL posY
    FIN ESTRUCTURA

    ESTRUCTURA Celda:
        ENTERO id
        Coordenada coord
        CADENA etiqueta
    FIN ESTRUCTURA

    INICIO
        Celda grid[3]
        IMPRIMIR("LARGO_GRID: " + grid.largo)

        // Todos los atributos de todos los elementos inician en NULO
        PARA (ENTERO i = 0; i < 3; i++)
            SI (grid[i].id == NULO Y grid[i].coord == NULO Y grid[i].etiqueta == NULO)
                IMPRIMIR("CELDA_" + i + "_INIT_NULO: VERDADERO")
            FIN SI
        FIN PARA

        // Mutar celda 0 y celda 2
        grid[0].id = 10
        grid[0].etiqueta = "Origen"
        Coordenada c0
        c0.posX = 0.0
        c0.posY = 0.0
        grid[0].coord = c0

        grid[2].id = 30
        grid[2].etiqueta = "Destino"
        Coordenada c2
        c2.posX = 100.5
        c2.posY = 200.5
        grid[2].coord = c2

        // Verificar celda 0
        IMPRIMIR("C0_ID: " + grid[0].id)
        IMPRIMIR("C0_X: " + grid[0].coord.posX)
        IMPRIMIR("C0_TAG: " + grid[0].etiqueta)

        // Celda 1 se mantiene intacta en NULO
        IMPRIMIR("C1_ID_NULO: " + (grid[1].id == NULO))
        IMPRIMIR("C1_COORD_NULO: " + (grid[1].coord == NULO))

        // Celda 2
        IMPRIMIR("C2_ID: " + grid[2].id)
        IMPRIMIR("C2_X: " + grid[2].coord.posX)
        IMPRIMIR("C2_TAG: " + grid[2].etiqueta)

        // Resetear coord de celda 0 a NULO
        grid[0].coord = NULO
        IMPRIMIR("C0_COORD_RESET: " + (grid[0].coord == NULO))
    FIN
    "#;

    let expected = [
        "LARGO_GRID: 3",
        "CELDA_0_INIT_NULO: VERDADERO",
        "CELDA_1_INIT_NULO: VERDADERO",
        "CELDA_2_INIT_NULO: VERDADERO",
        "C0_ID: 10",
        "C0_X: 0.0",
        "C0_TAG: Origen",
        "C1_ID_NULO: VERDADERO",
        "C1_COORD_NULO: VERDADERO",
        "C2_ID: 30",
        "C2_X: 100.5",
        "C2_TAG: Destino",
        "C0_COORD_RESET: VERDADERO",
    ];
    assert_dual_engine_ok(code, &expected);
}

#[test]
fn test_adv_vector_of_structs_explicit_assignment_dual_engine() {
    let code = r#"
    ESTRUCTURA Casilla:
        ENTERO numero
        CADENA ocupante
    FIN ESTRUCTURA

    INICIO
        Casilla tablero[2]
        Casilla c0
        c0.numero = 100
        tablero[0] = c0
        Casilla c1
        c1.numero = 200
        tablero[1] = c1

        IMPRIMIR("T0: " + tablero[0].numero)
        IMPRIMIR("T1: " + tablero[1].numero)

        // Resetear un elemento a NULO
        tablero[0] = NULO
        IMPRIMIR("T0_NULO: " + (tablero[0] == NULO))
    FIN
    "#;

    assert_dual_engine_ok(
        code,
        &[
            "T0: 100",
            "T1: 200",
            "T0_NULO: VERDADERO",
        ],
    );
}

#[test]
fn test_adv_vector_decl_element_defaults_dual_engine_parity() {
    let code = r#"
    ESTRUCTURA Casilla:
        ENTERO numero
        CADENA ocupante
    FIN ESTRUCTURA

    INICIO
        Casilla tablero[2]
        SI (tablero[0].numero == NULO Y tablero[1].ocupante == NULO)
            IMPRIMIR("TABLERO_NULO: VERDADERO")
        FIN SI
    FIN
    "#;

    assert_dual_engine_ok(code, &["TABLERO_NULO: VERDADERO"]);
}

#[test]
fn test_adv_truthiness_and_logical_operators_on_null_attributes() {
    let code = r#"
    ESTRUCTURA TestItem:
        ENTERO f_entero
        CADENA f_cadena
        TestItem f_sub
    FIN ESTRUCTURA

    INICIO
        TestItem item

        // SI directo sobre atributo NULO debe ser falsey
        SI (item.f_entero)
            IMPRIMIR("FALLA: f_entero tomado como truthy")
        SINO
            IMPRIMIR("ENTERO_FALSY: VERDADERO")
        FIN SI

        SI (item.f_sub)
            IMPRIMIR("FALLA: f_sub tomado como truthy")
        SINO
            IMPRIMIR("SUB_FALSY: VERDADERO")
        FIN SI

        // SI con NO sobre atributo NULO debe ser truthy
        SI (NO item.f_entero)
            IMPRIMIR("NO_ENTERO_TRUTHY: VERDADERO")
        FIN SI

        SI (NO item.f_sub)
            IMPRIMIR("NO_SUB_TRUTHY: VERDADERO")
        FIN SI

        // Operadores booleanos Y / O con atributos NULO
        SI (item.f_entero Y VERDADERO)
            IMPRIMIR("FALLA: NULO Y VERDADERO")
        SINO
            IMPRIMIR("NULO_Y_VERDADERO_ES_FALSO: VERDADERO")
        FIN SI

        SI (item.f_entero O VERDADERO)
            IMPRIMIR("NULO_O_VERDADERO_ES_VERDADERO: VERDADERO")
        FIN SI

        SI (NO item.f_entero Y NO item.f_sub)
            IMPRIMIR("AMBOS_NULOS_NEGADOS_CON_Y: VERDADERO")
        FIN SI

        // Comprobación de que asignar un valor modifica la truthiness adecuadamente
        item.f_entero = 100
        SI (item.f_entero)
            IMPRIMIR("ENTERO_ASIGNADO_TRUTHY: VERDADERO")
        FIN SI

        item.f_entero = 0
        SI (item.f_entero)
            IMPRIMIR("FALLA: Cero es truthy")
        SINO
            IMPRIMIR("CERO_FALSY: VERDADERO")
        FIN SI

        item.f_entero = NULO
        SI (item.f_entero)
            IMPRIMIR("FALLA: Nulo reasignado es truthy")
        SINO
            IMPRIMIR("REASIGNADO_NULO_FALSY: VERDADERO")
        FIN SI
    FIN
    "#;

    assert_dual_engine_ok(
        code,
        &[
            "ENTERO_FALSY: VERDADERO",
            "SUB_FALSY: VERDADERO",
            "NO_ENTERO_TRUTHY: VERDADERO",
            "NO_SUB_TRUTHY: VERDADERO",
            "NULO_Y_VERDADERO_ES_FALSO: VERDADERO",
            "NULO_O_VERDADERO_ES_VERDADERO: VERDADERO",
            "AMBOS_NULOS_NEGADOS_CON_Y: VERDADERO",
            "ENTERO_ASIGNADO_TRUTHY: VERDADERO",
            "CERO_FALSY: VERDADERO",
            "REASIGNADO_NULO_FALSY: VERDADERO",
        ],
    );
}

#[test]
fn test_adv_arithmetic_on_null_attributes_graceful_errors() {
    let code_add = r#"
    ESTRUCTURA Item:
        ENTERO val
    FIN ESTRUCTURA
    INICIO
        Item it
        ENTERO res = it.val + 10
    FIN
    "#;
    // AST error: "Operador '+' no compatible"
    // VM error: "Operador '+' no compatible"
    assert_dual_engine_err(code_add, "Operador '+' no compatible");

    let code_sub = r#"
    ESTRUCTURA Item:
        ENTERO val
    FIN ESTRUCTURA
    INICIO
        Item it
        ENTERO res = it.val - 5
    FIN
    "#;
    assert_dual_engine_err(code_sub, "Operador '-' solo compatible con números");

    let code_mul = r#"
    ESTRUCTURA Item:
        ENTERO val
    FIN ESTRUCTURA
    INICIO
        Item it
        ENTERO res = it.val * 3
    FIN
    "#;
    assert_dual_engine_err(code_mul, "Operador '*' solo compatible con números");

    let code_div = r#"
    ESTRUCTURA Item:
        ENTERO val
    FIN ESTRUCTURA
    INICIO
        Item it
        REAL res = it.val / 2
    FIN
    "#;
    assert_dual_engine_err(code_div, "Operador '/' solo compatible con números");

    let code_neg = r#"
    ESTRUCTURA Item:
        ENTERO val
    FIN ESTRUCTURA
    INICIO
        Item it
        ENTERO res = -it.val
    FIN
    "#;
    // AST error contains: "Operador unario '-' no aplicable"
    // VM error contains: "Operador unario '-' requiere número"
    let ast_err = run_code(code_neg, false).unwrap_err().to_string();
    let vm_err = run_code_vm(code_neg, false).unwrap_err().to_string();
    assert!(ast_err.contains("Operador unario '-'"));
    assert!(vm_err.contains("Operador unario '-'"));
}

#[test]
fn test_adv_equality_and_cross_type_comparisons() {
    let code = r#"
    ESTRUCTURA Entidad:
        ENTERO id
        REAL saldo
        CADENA nombre
        BOOLEANO activo
    FIN ESTRUCTURA

    INICIO
        Entidad e

        // Igualdad con NULO, null, None
        IMPRIMIR("ID_NULO: " + (e.id == NULO))
        IMPRIMIR("ID_NULL: " + (e.id == null))
        IMPRIMIR("ID_NONE: " + (e.id == None))

        // Desigualdad con NULO, null, None
        IMPRIMIR("ID_NO_NULO: " + (e.id != NULO))
        IMPRIMIR("ID_NO_NULL: " + (e.id != null))
        IMPRIMIR("ID_NO_NONE: " + (e.id != None))

        // Comparación cruzada contra valores por defecto primitivos (NO deben coincidir)
        IMPRIMIR("ID_NO_CERO: " + (e.id != 0))
        IMPRIMIR("ID_EQ_CERO: " + (e.id == 0))

        IMPRIMIR("SALDO_NO_CERO: " + (e.saldo != 0.0))
        IMPRIMIR("SALDO_EQ_CERO: " + (e.saldo == 0.0))

        IMPRIMIR("NOMBRE_NO_VACIO: " + (e.nombre != ""))
        IMPRIMIR("NOMBRE_EQ_VACIO: " + (e.nombre == ""))

        IMPRIMIR("ACTIVO_NO_FALSO: " + (e.activo != FALSO))
        IMPRIMIR("ACTIVO_EQ_FALSO: " + (e.activo == FALSO))

        // Simetría de igualdad (NULO a la izquierda)
        IMPRIMIR("SIMETRIA_NULO: " + (NULO == e.id))
        IMPRIMIR("SIMETRIA_NO_CERO: " + (0 != e.id))
    FIN
    "#;

    assert_dual_engine_ok(
        code,
        &[
            "ID_NULO: VERDADERO",
            "ID_NULL: VERDADERO",
            "ID_NONE: VERDADERO",
            "ID_NO_NULO: FALSO",
            "ID_NO_NULL: FALSO",
            "ID_NO_NONE: FALSO",
            "ID_NO_CERO: VERDADERO",
            "ID_EQ_CERO: FALSO",
            "SALDO_NO_CERO: VERDADERO",
            "SALDO_EQ_CERO: FALSO",
            "NOMBRE_NO_VACIO: VERDADERO",
            "NOMBRE_EQ_VACIO: FALSO",
            "ACTIVO_NO_FALSO: VERDADERO",
            "ACTIVO_EQ_FALSO: FALSO",
            "SIMETRIA_NULO: VERDADERO",
            "SIMETRIA_NO_CERO: VERDADERO",
        ],
    );
}
