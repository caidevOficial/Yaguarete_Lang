use yaguarete_core::run_code;

#[test]
fn test_division_entera_basica() {
    let code = r#"
    INICIO
        ENTERO a = 10 // 3
        ENTERO b = 20 // 4
        ENTERO c = 7 // 2
        ENTERO d = 100 // 10 // 2
        IMPRIMIR("A: " + a)
        IMPRIMIR("B: " + b)
        IMPRIMIR("C: " + c)
        IMPRIMIR("D: " + d)
    FIN
    "#;
    let out = run_code(code, false).unwrap();
    assert_eq!(out, vec!["A: 3", "B: 5", "C: 3", "D: 5"]);
}

#[test]
fn test_division_entera_dentro_de_indices() {
    let code = r#"
    INICIO
        ENTERO columnas = 2
        ENTERO origen[2][2] = [[1, 2], [3, 4]]
        ENTERO destino[2][2]
        PARA (ENTERO p = 0; p < 4; p++)
            destino[p // columnas][p % columnas] = origen[p // columnas][p % columnas] * 10
        FIN PARA
        IMPRIMIR(destino[0][0] + "," + destino[0][1] + "," + destino[1][0] + "," + destino[1][1])
    FIN
    "#;
    let out = run_code(code, false).unwrap();
    assert_eq!(out, vec!["10,20,30,40"]);
}

#[test]
fn test_division_entera_con_variables_y_precedencia() {
    let code = r#"
    INICIO
        ENTERO x = 14
        ENTERO val_b = 3
        ENTERO res = x // val_b
        ENTERO prec1 = 10 + 20 // 4
        ENTERO prec2 = (10 + 20) // 4
        IMPRIMIR("RES: " + res)
        IMPRIMIR("PREC1: " + prec1)
        IMPRIMIR("PREC2: " + prec2)
    FIN
    "#;
    let out = run_code(code, false).unwrap();
    assert_eq!(out, vec!["RES: 4", "PREC1: 15", "PREC2: 7"]);
}

#[test]
fn test_division_entera_en_funciones_y_retorno() {
    let code = r#"
    FUNCION dividir(dividendo: ENTERO, divisor: ENTERO): ENTERO
        RETORNAR dividendo // divisor
    FIN FUNCION

    INICIO
        ENTERO res = dividir(27, 4)
        IMPRIMIR("DIV: " + res)
    FIN
    "#;
    let out = run_code(code, false).unwrap();
    assert_eq!(out, vec!["DIV: 6"]);
}

#[test]
fn test_division_entera_por_cero_arroja_error() {
    let code = r#"
    INICIO
        ENTERO a = 10 // 0
    FIN
    "#;
    let res = run_code(code, false);
    assert!(res.is_err());
    let err_msg = res.unwrap_err().to_string();
    assert!(err_msg.to_lowercase().contains("cero"));
}

#[test]
fn test_modulo_operador_basico() {
    let code = r#"
    INICIO
        ENTERO m1 = 10 % 3
        ENTERO m2 = 15 % 4
        ENTERO m3 = 20 % 5
        ENTERO m4 = 25 % 7
        IMPRIMIR("M1: " + m1)
        IMPRIMIR("M2: " + m2)
        IMPRIMIR("M3: " + m3)
        IMPRIMIR("M4: " + m4)
    FIN
    "#;
    let out = run_code(code, false).unwrap();
    assert_eq!(out, vec!["M1: 1", "M2: 3", "M3: 0", "M4: 4"]);
}

#[test]
fn test_modulo_con_variables_y_precedencia() {
    let code = r#"
    INICIO
        ENTERO a = 23
        ENTERO b = 5
        ENTERO r = a % b
        ENTERO comb = (a // b) * b + (a % b)
        IMPRIMIR("R: " + r)
        IMPRIMIR("COMB: " + comb)
    FIN
    "#;
    let out = run_code(code, false).unwrap();
    assert_eq!(out, vec!["R: 3", "COMB: 23"]);
}

#[test]
fn test_modulo_por_cero_arroja_error() {
    let code = r#"
    INICIO
        ENTERO a = 10 % 0
    FIN
    "#;
    let res = run_code(code, false);
    assert!(res.is_err());
    let err_msg = res.unwrap_err().to_string();
    assert!(err_msg.to_lowercase().contains("cero") || err_msg.to_lowercase().contains("módulo"));
}

#[test]
fn test_validacion_tipos_division_entera_y_modulo() {
    let code_inv_div = r#"
    INICIO
        CADENA s = "texto"
        ENTERO x = s // 2
    FIN
    "#;
    assert!(run_code(code_inv_div, false).is_err());

    let code_inv_mod = r#"
    INICIO
        CADENA s = "texto"
        ENTERO x = s % 2
    FIN
    "#;
    assert!(run_code(code_inv_mod, false).is_err());
}

#[test]
fn test_convivencia_division_entera_con_comentarios() {
    let code = r#"
    // Comentario en la primera linea
    INICIO
        // Comentario indentado
        ENTERO x = 10 // 3
        ENTERO val_y = 20 // 4 // Comentario explicativo
        ENTERO z = 100 // 2 # Comentario con hash
        IMPRIMIR("X: " + x)
        IMPRIMIR("Y: " + val_y)
        IMPRIMIR("Z: " + z)
    FIN // Fin del programa
    "#;
    let out = run_code(code, false).unwrap();
    assert_eq!(out, vec!["X: 3", "Y: 5", "Z: 50"]);
}

#[test]
fn test_comentarios_con_hash_en_lineas_independientes() {
    let code = r#"
    # Comentario de inicio con hash
    INICIO
        # Declaracion de variable
        ENTERO valor = 42 # inline con hash
        IMPRIMIR("VAL: " + valor)
    FIN
    "#;
    let out = run_code(code, false).unwrap();
    assert_eq!(out, vec!["VAL: 42"]);
}

#[test]
fn test_cortocircuito_operador_y() {
    // 1. Evita división por cero si la parte izquierda es falsa
    let code_div_cero = r#"
    INICIO
        BOOLEANO cond = FALSO Y (10 / 0 == 0)
        IMPRIMIR("COND: " + cond)
    FIN
    "#;
    let out = run_code(code_div_cero, false).unwrap();
    assert_eq!(out, vec!["COND: FALSO"]);

    // 2. Evita desbordamiento de índice si la parte izquierda es falsa (patrón de guarda)
    let code_indice = r#"
    INICIO
        ENTERO v[3] = [10, 20, 30]
        ENTERO j = -1
        SI (j >= 0 Y v[j] > 0)
            IMPRIMIR("DENTRO")
        SINO
            IMPRIMIR("FUERA")
        FIN SI
    FIN
    "#;
    let out_idx = run_code(code_indice, false).unwrap();
    assert_eq!(out_idx, vec!["FUERA"]);
}

#[test]
fn test_cortocircuito_operador_o() {
    // 1. Evita división por cero si la parte izquierda es verdadera
    let code_div_cero = r#"
    INICIO
        BOOLEANO cond = VERDADERO O (10 / 0 == 0)
        IMPRIMIR("COND: " + cond)
    FIN
    "#;
    let out = run_code(code_div_cero, false).unwrap();
    assert_eq!(out, vec!["COND: VERDADERO"]);

    // 2. Evita desbordamiento de índice si la parte izquierda es verdadera
    let code_indice = r#"
    INICIO
        ENTERO v[3] = [10, 20, 30]
        ENTERO j = -1
        SI (j < 0 O v[j] == 0)
            IMPRIMIR("SEGURO")
        SINO
            IMPRIMIR("INSEGURO")
        FIN SI
    FIN
    "#;
    let out_idx = run_code(code_indice, false).unwrap();
    assert_eq!(out_idx, vec!["SEGURO"]);
}

#[test]
fn test_raizcuadrada_entero_real_y_error_negativo() {
    let code = r#"
    INICIO
        REAL r1 = RAIZCUADRADA(16)
        REAL r2 = RAIZCUADRADA(2.25)
        REAL r3 = RAIZCUADRADA(0)
        IMPRIMIR("R1: " + r1)
        IMPRIMIR("R2: " + r2)
        IMPRIMIR("R3: " + r3)
    FIN
    "#;
    let out = run_code(code, false).unwrap();
    assert_eq!(out, vec!["R1: 4.0", "R2: 1.5", "R3: 0.0"]);

    let code_neg = r#"
    INICIO
        REAL err = RAIZCUADRADA(-9)
    FIN
    "#;
    let res = run_code(code_neg, false);
    assert!(res.is_err());
    assert!(res.unwrap_err().to_string().contains("negativo"));
}

#[test]
fn test_operador_potencia_precedencia_asociatividad_y_unario() {
    let code = r#"
    INICIO
        REAL p_ent = 2 ^ 3
        REAL p_real = 2.5 ^ 2
        REAL p_frac = 9 ^ 0.5
        REAL prec = 2 + 3 * 2 ^ 3
        REAL asoc_der = 2 ^ 3 ^ 2
        REAL unario_izq = -2 ^ 2
        REAL unario_exp = 2 ^ -3
        IMPRIMIR("ENT: " + p_ent)
        IMPRIMIR("REAL: " + p_real)
        IMPRIMIR("FRAC: " + p_frac)
        IMPRIMIR("PREC: " + prec)
        IMPRIMIR("ASOC: " + asoc_der)
        IMPRIMIR("UN_IZQ: " + unario_izq)
        IMPRIMIR("UN_EXP: " + unario_exp)
    FIN
    "#;
    let out = run_code(code, false).unwrap();
    assert_eq!(
        out,
        vec![
            "ENT: 8.0",
            "REAL: 6.25",
            "FRAC: 3.0",
            "PREC: 26.0",
            "ASOC: 512.0",
            "UN_IZQ: -4.0",
            "UN_EXP: 0.125"
        ]
    );

    let code_err_neg_frac = r#"
    INICIO
        REAL r = (-4) ^ 0.5
    FIN
    "#;
    assert!(run_code(code_err_neg_frac, false).is_err());

    let code_err_zero_neg = r#"
    INICIO
        REAL r = 0 ^ -2
    FIN
    "#;
    assert!(run_code(code_err_zero_neg, false).is_err());
}

#[test]
fn test_funcion_potencia_enteros_y_reales() {
    let code = r#"
    INICIO
        REAL p1 = POTENCIA(2, 10)
        REAL p2 = POTENCIA(1.5, 2)
        REAL p3 = POTENCIA(16, 0.5)
        REAL p4 = POTENCIA(5, 0)
        IMPRIMIR("P1: " + p1)
        IMPRIMIR("P2: " + p2)
        IMPRIMIR("P3: " + p3)
        IMPRIMIR("P4: " + p4)
    FIN
    "#;
    let out = run_code(code, false).unwrap();
    assert_eq!(out, vec!["P1: 1024.0", "P2: 2.25", "P3: 4.0", "P4: 1.0"]);
}

#[test]
fn test_funcion_abs_enteros_y_reales() {
    let code = r#"
    INICIO
        ENTERO a1 = ABS(-42)
        ENTERO a2 = ABS(42)
        ENTERO a3 = ABS(0)
        REAL r1 = ABS(-3.75)
        REAL r2 = ABS(3.75)
        REAL r3 = ABS(0.0)
        IMPRIMIR("A1: " + a1)
        IMPRIMIR("A2: " + a2)
        IMPRIMIR("A3: " + a3)
        IMPRIMIR("R1: " + r1)
        IMPRIMIR("R2: " + r2)
        IMPRIMIR("R3: " + r3)
    FIN
    "#;
    let out = run_code(code, false).unwrap();
    assert_eq!(
        out,
        vec![
            "A1: 42",
            "A2: 42",
            "A3: 0",
            "R1: 3.75",
            "R2: 3.75",
            "R3: 0.0"
        ]
    );
}

#[test]
fn test_funcion_aleatorio_rango_igual_y_error() {
    let code = r#"
    INICIO
        BOOLEANO todos_en_rango = VERDADERO
        PARA (ENTERO i = 0; i < 50; i++)
            ENTERO n = ALEATORIO(10, 20)
            SI (n < 10 O n > 20)
                todos_en_rango = FALSO
            FIN SI
        FIN PARA
        ENTERO fijo = ALEATORIO(7, 7)
        IMPRIMIR("RANGO: " + todos_en_rango)
        IMPRIMIR("FIJO: " + fijo)
    FIN
    "#;
    let out = run_code(code, false).unwrap();
    assert_eq!(out, vec!["RANGO: VERDADERO", "FIJO: 7"]);

    let code_err = r#"
    INICIO
        ENTERO inv = ALEATORIO(20, 10)
    FIN
    "#;
    let res = run_code(code_err, false);
    assert!(res.is_err());
    assert!(res.unwrap_err().to_string().contains("mayor que 'max'"));
}

#[test]
fn test_constante_pi_y_error_reasignacion() {
    let code = r#"
    INICIO
        REAL radio = 5.0
        REAL area = PI * radio ^ 2
        BOOLEANO pi_valido = (PI > 3.14159) Y (PI < 3.14160)
        BOOLEANO area_valida = (area > 78.539) Y (area < 78.540)
        IMPRIMIR("PI_OK: " + pi_valido)
        IMPRIMIR("AREA_OK: " + area_valida)
    FIN
    "#;
    let out = run_code(code, false).unwrap();
    assert_eq!(out, vec!["PI_OK: VERDADERO", "AREA_OK: VERDADERO"]);

    let code_reassign = r#"
    INICIO
        PI = 3.14
    FIN
    "#;
    let res = run_code(code_reassign, false);
    assert!(res.is_err());
    assert!(res.unwrap_err().to_string().contains("PI"));

    let code_redecl = r#"
    INICIO
        REAL PI = 3.14
    FIN
    "#;
    let res2 = run_code(code_redecl, false);
    assert!(res2.is_err());
    assert!(res2.unwrap_err().to_string().contains("PI"));
}

