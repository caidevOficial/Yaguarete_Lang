use std::cell::RefCell;
use std::rc::Rc;
use yaguarete_core::{run_code, Interpreter, Lexer, Parser};

fn ok(expr: &str) -> String {
    let code = format!("INICIO\n    IMPRIMIR({})\nFIN\n", expr);
    let out = run_code(&code, false).unwrap_or_else(|e| panic!("{} fallo: {}", expr, e));
    out.join("\n")
}

fn err(expr: &str) -> String {
    let code = format!("INICIO\n    IMPRIMIR({})\nFIN\n", expr);
    match run_code(&code, false) {
        Ok(o) => panic!("{} debía fallar, salida: {:?}", expr, o),
        Err(e) => e.to_string(),
    }
}

type Prompts = Rc<RefCell<Vec<String>>>;

fn run_with_input(src: &str, lines: Vec<&str>) -> (Result<Vec<String>, String>, Vec<String>) {
    let tokens = Lexer::new(src).tokenize().unwrap();
    let program = Parser::new(tokens, false).parse().unwrap();
    let mut interp = Interpreter::new();
    let prompts: Prompts = Rc::new(RefCell::new(Vec::new()));
    let p2 = prompts.clone();
    let mut queue: std::collections::VecDeque<String> = lines.into_iter().map(String::from).collect();
    interp.set_input_reader(Box::new(move |prompt: &str| {
        p2.borrow_mut().push(prompt.to_string());
        queue.pop_front()
    }));
    let res = interp.run(&program).map(|_| interp.output.clone()).map_err(|e| e.to_string());
    let seen = prompts.borrow().clone();
    (res, seen)
}

fn leer(expr: &str, line: Option<&str>) -> (String, Vec<String>) {
    let src = format!("INICIO\n    IMPRIMIR(A_CADENA({}))\n    IMPRIMIR({})\nFIN\n", expr, expr);
    let _ = src;
    let src = format!("INICIO\n    IMPRIMIR({})\nFIN\n", expr);
    let (r, p) = run_with_input(&src, line.into_iter().collect());
    (r.unwrap().join("\n"), p)
}

#[test]
fn mayusculas_minusculas_recortar() {
    assert_eq!(ok("MAYUSCULAS(\"hola\")"), "HOLA");
    assert_eq!(ok("MINUSCULAS(\"HoLa\")"), "hola");
    assert_eq!(ok("RECORTAR(\"  hola  \") + \"|\""), "hola|");
    assert_eq!(ok("MAYUSCULAS('a')"), "A");
    assert!(err("MAYUSCULAS(5)").contains("CADENA"));
    assert!(err("MINUSCULAS()").contains("exactamente 1"));
    assert!(err("RECORTAR(\"a\", \"b\")").contains("exactamente 1"));
}

#[test]
fn dividir_cadena() {
    assert_eq!(ok("DIVIDIR_CADENA(\"a,b,,c\", \",\")"), "[a, b, , c]");
    assert_eq!(ok("DIVIDIR_CADENA(\"a--b\", \"--\").largo"), "2");
    assert!(err("DIVIDIR_CADENA(\"abc\", \"\")").contains("El separador no puede estar vacío."));
    assert!(err("DIVIDIR_CADENA(\"abc\")").contains("exactamente 2"));
    assert!(err("DIVIDIR_CADENA(1, \",\")").contains("CADENA"));
    let code = r#"
    INICIO
        CADENA partes[] = DIVIDIR_CADENA("x;y;z", ";")
        IMPRIMIR(partes.largo)
        IMPRIMIR(partes[1])
    FIN
    "#;
    let out = run_code(code, false).unwrap();
    assert_eq!(out, vec!["3".to_string(), "y".to_string()]);
}

#[test]
fn conversiones() {
    assert_eq!(ok("A_ENTERO(\" 42 \")"), "42");
    assert_eq!(ok("A_ENTERO(\"-3.9\")"), "-3");
    assert_eq!(ok("A_ENTERO(3.99)"), "3");
    assert_eq!(ok("A_ENTERO(-3.99)"), "-3");
    assert_eq!(ok("A_ENTERO(VERDADERO)"), "1");
    assert_eq!(ok("A_ENTERO(FALSO)"), "0");
    assert_eq!(ok("A_ENTERO(7)"), "7");
    assert_eq!(ok("A_ENTERO('7')"), "7");
    assert!(err("A_ENTERO(\"abc\")").contains("No se puede convertir 'abc' a ENTERO"));
    assert!(err("A_ENTERO('x')").contains("No se puede convertir 'x' a ENTERO"));
    assert!(err("A_ENTERO()").contains("exactamente 1"));

    assert_eq!(ok("A_REAL(\"2.5\")"), "2.5");
    assert_eq!(ok("A_REAL(3)"), "3.0");
    assert_eq!(ok("A_REAL(VERDADERO)"), "1.0");
    assert_eq!(ok("A_REAL(FALSO)"), "0.0");
    assert!(err("A_REAL(\"zzz\")").contains("No se puede convertir"));
    assert_eq!(ok("A_FLOTANTE(\"1.5\")"), "1.5");
    assert_eq!(ok("A_FLOTANTE(2)"), "2.0");
    assert_eq!(ok("A_FLOTANTE(VERDADERO)"), "1.0");
    assert!(err("A_FLOTANTE(\"x\")").contains("No se puede convertir"));

    assert_eq!(ok("A_CADENA(12)"), "12");
    assert_eq!(ok("A_CADENA(VERDADERO)"), "VERDADERO");
    assert_eq!(ok("A_CADENA(2.0)"), "2.0");
    assert_eq!(ok("A_CADENA(5) + \"x\""), "5x");

    assert_eq!(ok("A_BOOLEANO(\" true \")"), "VERDADERO");
    assert_eq!(ok("A_BOOLEANO(\"Falso\")"), "FALSO");
    assert_eq!(ok("A_BOOLEANO(0)"), "FALSO");
    assert_eq!(ok("A_BOOLEANO(3)"), "VERDADERO");
    assert_eq!(ok("A_BOOLEANO(0.0)"), "FALSO");
    assert_eq!(ok("A_BOOLEANO(VERDADERO)"), "VERDADERO");
    assert!(err("A_BOOLEANO(\"quizas\")").contains("BOOLEANO"));
}

#[test]
fn min_max() {
    assert_eq!(ok("MIN(3, 5)"), "3");
    assert_eq!(ok("MAX(3, 5)"), "5");
    assert_eq!(ok("MIN(3, 5.5)"), "3.0");
    assert_eq!(ok("MAX(3, 5.5)"), "5.5");
    assert_eq!(ok("MAX(-1, -2)"), "-1");
    assert!(err("MIN(1)").contains("exactamente 2"));
    assert!(err("MAX(1, 2, 3)").contains("exactamente 2"));
    assert!(err("MIN(\"a\", 1)").contains("numéricos"));
    let code = r#"
    INICIO
        FLOTANTE a = 1.5
        FLOTANTE b = 2.5
        IMPRIMIR(MAX(a, b))
    FIN
    "#;
    assert_eq!(run_code(code, false).unwrap(), vec!["2.5".to_string()]);
}

#[test]
fn redondear() {
    assert_eq!(ok("REDONDEAR(2.5)"), "3");
    assert_eq!(ok("REDONDEAR(-2.5)"), "-3");
    assert_eq!(ok("REDONDEAR(2.4)"), "2");
    assert_eq!(ok("REDONDEAR(7)"), "7");
    assert_eq!(ok("REDONDEAR(3.14159, 2)"), "3.14");
    assert_eq!(ok("REDONDEAR(1.005, 2)"), "1.0");
    assert_eq!(ok("REDONDEAR(2.675, 0)"), "3.0");
    assert!(err("REDONDEAR()").contains("1 o 2"));
    assert!(err("REDONDEAR(1, 2, 3)").contains("1 o 2"));
    assert!(err("REDONDEAR(\"a\")").contains("numérico"));
    assert!(err("REDONDEAR(1.5, -1)").contains("decimales"));
    assert!(err("REDONDEAR(1.5, 1.5)").contains("decimales"));
}

#[test]
fn leer_tipos() {
    assert_eq!(leer("LEER()", Some("42")).0, "42");
    let (_, _) = leer("LEER()", Some("x"));
    let src = |l: &str, f: &str| {
        let s = format!("INICIO\n    IMPRIMIR({})\nFIN\n", f);
        run_with_input(&s, vec![l]).0.unwrap().join("\n")
    };
    assert_eq!(src("-7", "LEER() + 1"), "-6");
    assert_eq!(src("+7", "LEER() + 1"), "8");
    assert_eq!(src(" 7 ", "LEER() + 1"), "8");
    assert_eq!(src("2.5", "LEER() * 2"), "5.0");
    assert_eq!(src("1e3", "LEER() + 0.5"), "1000.5");
    assert_eq!(src("verdadero", "LEER()"), "VERDADERO");
    assert_eq!(src("False", "LEER()"), "FALSO");
    assert_eq!(src("TRUE", "LEER()"), "VERDADERO");
    assert_eq!(src("hola mundo", "LEER() + \"!\""), "hola mundo!");
    assert_eq!(src("  hola  ", "LEER() + \"|\""), "  hola  |");
    assert_eq!(src("12abc", "LEER() + \"|\""), "12abc|");
    assert_eq!(src("", "LEER() + \"|\""), "|");
    assert_eq!(src("nulo", "LEER() + \"|\""), "nulo|");
}

#[test]
fn leer_prompt_y_eof() {
    let (out, prompts) = leer("LEER(\"Edad: \")", Some("30"));
    assert_eq!(out, "30");
    assert_eq!(prompts, vec!["Edad: ".to_string()]);

    let (out, prompts) = leer("LEER()", Some("5"));
    assert_eq!(out, "5");
    assert_eq!(prompts, vec!["".to_string()]);

    // EOF => cadena vacía
    let (out, prompts) = leer("LEER(\"?\") + \"|\"", None);
    assert_eq!(out, "|");
    assert_eq!(prompts, vec!["?".to_string()]);

    // Varias lecturas consecutivas y asignación con adaptación de tipo
    let code = r#"
    INICIO
        REAL x = LEER("x: ")
        ENTERO n = LEER("n: ")
        CADENA s = LEER("s: ")
        IMPRIMIR(x, n, s)
    FIN
    "#;
    let (res, prompts) = run_with_input(code, vec!["3", "4", "hola"]);
    assert_eq!(res.unwrap(), vec!["3.0 4 hola".to_string()]);
    assert_eq!(prompts, vec!["x: ", "n: ", "s: "]);
}

#[test]
fn leer_errores() {
    let run = |expr: &str| {
        let s = format!("INICIO\n    IMPRIMIR({})\nFIN\n", expr);
        run_with_input(&s, vec!["1"]).0.unwrap_err()
    };
    assert!(run("LEER(1)").contains("CADENA"));
    assert!(run("LEER(\"a\", \"b\")").contains("0 o 1"));

    let tokens = Lexer::new("INICIO\n    IMPRIMIR(LEER())\nFIN\n").tokenize().unwrap();
    let program = Parser::new(tokens, false).parse().unwrap();
    let mut interp = Interpreter::new();
    interp.set_input_reader(Box::new(|_| Some("1".to_string())));
    interp.disable_input();
    let e = interp.run(&program).unwrap_err().to_string();
    assert!(e.contains("LEER no está disponible durante la depuración (DAP)."));
}

#[test]
fn parse_auto_value_publico() {
    use yaguarete_core::Value;
    assert_eq!(yaguarete_core::value::parse_auto_value("12"), Value::Entero(12));
    assert_eq!(yaguarete_core::eval::parse_input_line("1.5"), Value::Real(1.5));
    assert_eq!(yaguarete_core::eval::parse_input_line("abc"), Value::Cadena("abc".into()));
}
