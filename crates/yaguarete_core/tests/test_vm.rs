use yaguarete_core::run_code_vm;

#[test]
fn test_vm_basic_arithmetic_and_print() {
    let code = r#"
    INICIO
        ENTERO a = 10
        ENTERO b = 20
        ENTERO c = a + b * 2 - 5
        IMPRIMIR("RESULTADO:", c)
    FIN
    "#;
    let out = run_code_vm(code, false).unwrap();
    assert_eq!(out, vec!["RESULTADO: 45"]);
}

#[test]
fn test_vm_conditionals_if_else() {
    let code = r#"
    INICIO
        ENTERO x = 15
        SI (x > 20)
            IMPRIMIR("MAYOR A 20")
        SINO SI (x == 15)
            IMPRIMIR("EXACTO 15")
        SINO
            IMPRIMIR("MENOR")
        FIN SI
    FIN
    "#;
    let out = run_code_vm(code, false).unwrap();
    assert_eq!(out, vec!["EXACTO 15"]);
}

#[test]
fn test_vm_loops_while_and_for() {
    let code = r#"
    INICIO
        ENTERO total = 0
        PARA (ENTERO i = 0; i < 5; i++)
            total = total + i
        FIN PARA

        ENTERO cuenta = 3
        MIENTRAS (cuenta > 0)
            total = total + cuenta
            cuenta--
        FIN MIENTRAS

        IMPRIMIR("TOTAL:", total)
    FIN
    "#;
    let out = run_code_vm(code, false).unwrap();
    // PARA: 0+1+2+3+4 = 10. MIENTRAS: 10 + 3 + 2 + 1 = 16.
    assert_eq!(out, vec!["TOTAL: 16"]);
}

#[test]
fn test_vm_functions_and_recursion() {
    let code = r#"
    FUNCION factorial(n: ENTERO): ENTERO
        SI (n <= 1)
            RETORNAR 1
        SINO
            RETORNAR n * factorial(n - 1)
        FIN SI
    FIN FUNCION

    INICIO
        ENTERO res = factorial(5)
        IMPRIMIR("FACT(5):", res)
    FIN
    "#;
    let out = run_code_vm(code, false).unwrap();
    assert_eq!(out, vec!["FACT(5): 120"]);
}

#[test]
fn test_vm_vector_and_selection_sort() {
    let code = r#"
    INICIO
        ENTERO v[5] = [50, 20, 40, 10, 30]
        ENTERO n = v.largo

        PARA (ENTERO i = 0; i < n - 1; i++)
            ENTERO min_idx = i
            PARA (ENTERO j = i + 1; j < n; j++)
                SI (v[j] < v[min_idx])
                    min_idx = j
                FIN SI
            FIN PARA

            SI (min_idx != i)
                ENTERO tmp = v[i]
                v[i] = v[min_idx]
                v[min_idx] = tmp
            FIN SI
        FIN PARA

        IMPRIMIR("ORDENADO:", v[0], v[1], v[2], v[3], v[4])
    FIN
    "#;
    let out = run_code_vm(code, false).unwrap();
    assert_eq!(out, vec!["ORDENADO: 10 20 30 40 50"]);
}

#[test]
fn test_vm_builtins() {
    let code = r#"
    INICIO
        CADENA s = "  hola mundo  "
        IMPRIMIR(MAYUSCULAS(RECORTAR(s)))
        IMPRIMIR(MINUSCULAS("YAGUARETE"))

        ENTERO n = A_ENTERO("42")
        REAL r = A_REAL("3.14")
        BOOLEANO b = A_BOOLEANO("VERDADERO")
        IMPRIMIR("CONVERTIDOS:", n, r, b)

        IMPRIMIR("MIN_MAX:", MIN(10, 20), MAX(10, 20))
        IMPRIMIR("REDONDEAR:", REDONDEAR(3.14159, 2))

        CADENA partes[] = DIVIDIR_CADENA("uno,dos,tres", ",")
        IMPRIMIR("PARTES:", partes.largo, partes[0], partes[1], partes[2])
    FIN
    "#;
    let out = run_code_vm(code, false).unwrap();
    assert_eq!(out, vec![
        "HOLA MUNDO",
        "yaguarete",
        "CONVERTIDOS: 42 3.14 VERDADERO",
        "MIN_MAX: 10 20",
        "REDONDEAR: 3.14",
        "PARTES: 3 uno dos tres"
    ]);
}

#[test]
fn test_vm_structs() {
    let code = r#"
    INICIO
        ESTRUCTURA Punto:
            ENTERO pos_x
            ENTERO pos_y
        Fin ESTRUCTURA

        Punto p
        p.pos_x = 100
        p.pos_y = 200
        IMPRIMIR("PUNTO:", p.pos_x, p.pos_y)
    FIN
    "#;
    let out = run_code_vm(code, false).unwrap();
    assert_eq!(out, vec!["PUNTO: 100 200"]);
}

#[test]
fn test_vm_ejemplo_file() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let file = root.join("ejemplo.yaguarete");
    let out = yaguarete_core::run_file_vm(file, false).unwrap();
    assert!(out.iter().any(|l| l.contains("Recorriendo el nodo: Nodo 1")));
    assert!(out.iter().any(|l| l.contains("Recorriendo el nodo: Nodo 5")));
}
