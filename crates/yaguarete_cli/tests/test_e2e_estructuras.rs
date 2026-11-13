//! Comprehensive Opaque-Box CLI Integration Test Suite for YAGUARETÉ Tier 4 & Estructuras
//!
//! Validates:
//! 1. `colas.yaguarete`: FIFO queue lifecycle, enqueuing, dequeuing, empty queue NULO handling, exit code 0.
//! 2. `semaforo.yaguarete` & Traffic Simulation: Empty avenue detection (`primer_auto == NULO`), auto status output, termination.
//! 3. Singly linked list scenario: Dynamic insertions, traversal, terminal NULO check, exit code 0.
//! 4. Organizational hierarchy tree scenario: Structs with child pointers defaulting to NULO, leaf detection, exit code 0.
//! 5. Banking transaction registry scenario: Account balance + transaction history defaulting to NULO, deposit mutations, exit code 0.
//! 6. CLI engine flags & diagnostics: `yaguarete check`, exit code 0 on success, exit code 1 on syntax and null runtime errors.
//! 7. Dual-engine parity: Parity between `--engine vm` and `--engine ast`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

// =============================================================================
// CLI Execution Harness & RAII Temp File Guard
// =============================================================================

/// Result of executing the `yaguarete` CLI binary.
#[derive(Debug, Clone)]
struct CliResult {
    success: bool,
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
    timed_out: bool,
}

/// RAII Guard for temporary `.yaguarete` scripts, ensuring clean removal on drop.
struct TempScript {
    path: PathBuf,
}

impl TempScript {
    fn new(prefix: &str, content: &str) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let file_name = format!("yaguarete_e2e_{}_{}_{}.yaguarete", prefix, pid, id);
        let path = std::env::temp_dir().join(file_name);
        fs::write(&path, content).unwrap_or_else(|e| {
            panic!("Failed to write temporary test script to {:?}: {}", path, e)
        });
        Self { path }
    }

    #[allow(dead_code)]
    fn path(&self) -> &Path {
        &self.path
    }

    fn path_str(&self) -> &str {
        self.path.to_str().expect("Path is valid UTF-8")
    }
}

impl Drop for TempScript {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

/// Resolves the repository root directory relative to the CLI test binary.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates directory")
        .parent()
        .expect("repo root directory")
        .to_path_buf()
}

/// Runs the `yaguarete` CLI binary with arguments and an execution timeout.
///
/// If execution exceeds `timeout`, kills the child process cleanly to prevent
/// hanging tests or Windows file locks (`os error 5`).
fn run_cli_with_timeout(args: &[&str], timeout: Duration) -> CliResult {
    let mut child = Command::new(env!("CARGO_BIN_EXE_yaguarete"))
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("Failed to spawn yaguarete binary with args {:?}: {}", args, e));

    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut stdout_buf = Vec::new();
                let mut stderr_buf = Vec::new();
                if let Some(mut out) = child.stdout.take() {
                    let _ = std::io::Read::read_to_end(&mut out, &mut stdout_buf);
                }
                if let Some(mut err) = child.stderr.take() {
                    let _ = std::io::Read::read_to_end(&mut err, &mut stderr_buf);
                }
                return CliResult {
                    success: status.success(),
                    exit_code: status.code(),
                    stdout: String::from_utf8_lossy(&stdout_buf).to_string(),
                    stderr: String::from_utf8_lossy(&stderr_buf).to_string(),
                    timed_out: false,
                };
            }
            Ok(None) => {
                if start.elapsed() > timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    let mut stdout_buf = Vec::new();
                    let mut stderr_buf = Vec::new();
                    if let Some(mut out) = child.stdout.take() {
                        let _ = std::io::Read::read_to_end(&mut out, &mut stdout_buf);
                    }
                    if let Some(mut err) = child.stderr.take() {
                        let _ = std::io::Read::read_to_end(&mut err, &mut stderr_buf);
                    }
                    return CliResult {
                        success: false,
                        exit_code: None,
                        stdout: String::from_utf8_lossy(&stdout_buf).to_string(),
                        stderr: String::from_utf8_lossy(&stderr_buf).to_string(),
                        timed_out: true,
                    };
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                panic!("Error waiting for yaguarete process: {}", e);
            }
        }
    }
}

// =============================================================================
// Scenario 1: colas.yaguarete — Full FIFO Queue Lifecycle
// =============================================================================

#[test]
fn test_e2e_colas_fifo_lifecycle_vm_engine() {
    let colas_file = repo_root().join("colas.yaguarete");
    assert!(colas_file.exists(), "colas.yaguarete must exist in repository root");

    let res = run_cli_with_timeout(
        &["run", colas_file.to_str().unwrap(), "--engine", "vm"],
        Duration::from_secs(10),
    );

    assert!(!res.timed_out, "colas.yaguarete timed out on VM engine");
    assert!(
        res.success,
        "colas.yaguarete failed with exit code {:?}. Stderr:\n{}",
        res.exit_code, res.stderr
    );
    assert_eq!(res.exit_code, Some(0));

    // Verify FIFO enqueue progression
    assert!(res.stdout.contains("Agregado: NodoCliente{nombre: 1, proximo_cliente: NULO}"));
    assert!(res.stdout.contains("Agregado: NodoCliente{nombre: 2, proximo_cliente: NULO}"));
    assert!(res.stdout.contains("Agregado: NodoCliente{nombre: 3, proximo_cliente: NULO}"));
    assert!(res.stdout.contains("Agregado: NodoCliente{nombre: 4, proximo_cliente: NULO}"));

    // Verify queue report structure
    assert!(res.stdout.contains("============= INFO COLA ============="));
    assert!(res.stdout.contains("Ultimo de la cola: NodoCliente{nombre: 4, proximo_cliente: NULO}"));

    // Verify FIFO dequeue operation (item 1 dequeued first)
    assert!(res.stdout.contains("Elemento eliminado: NodoCliente{nombre: 1, proximo_cliente: NULO}"));

    // Verify resulting queue starting at node 2
    assert!(res.stdout.contains("Impresión de la cola sin el primer elemento: Cola{procesar_primero: NodoCliente{nombre: 2, proximo_cliente: NodoCliente{nombre: 3, proximo_cliente: NodoCliente{nombre: 4, proximo_cliente: NULO}}}"));
    assert!(res.stdout.contains("============= FIN INFO COLA ============="));
}

#[test]
fn test_e2e_colas_fifo_lifecycle_ast_engine() {
    let colas_file = repo_root().join("colas.yaguarete");
    assert!(colas_file.exists(), "colas.yaguarete must exist in repository root");

    let res = run_cli_with_timeout(
        &["run", colas_file.to_str().unwrap(), "--engine", "ast"],
        Duration::from_secs(10),
    );

    assert!(!res.timed_out, "colas.yaguarete timed out on AST engine");
    assert!(
        res.success,
        "colas.yaguarete failed with exit code {:?}. Stderr:\n{}",
        res.exit_code, res.stderr
    );
    assert_eq!(res.exit_code, Some(0));

    // Dual-engine parity validation for FIFO steps
    assert!(res.stdout.contains("Agregado: NodoCliente{nombre: 1, proximo_cliente: NULO}"));
    assert!(res.stdout.contains("Agregado: NodoCliente{nombre: 4, proximo_cliente: NULO}"));
    assert!(res.stdout.contains("Elemento eliminado: NodoCliente{nombre: 1, proximo_cliente: NULO}"));
    assert!(res.stdout.contains("============= FIN INFO COLA ============="));
}

#[test]
fn test_e2e_colas_direct_declaration_without_factory() {
    // Tests that with v0.2.1 default NULO initialization, a Cola instance
    // declared directly without manual factory functions has NULO pointers.
    let code = r#"
    ESTRUCTURA NodoCliente:
        CADENA nombre
        NodoCliente proximo_cliente
    FIN ESTRUCTURA

    ESTRUCTURA Cola:
        NodoCliente procesar_primero
        NodoCliente procesar_ultimo
    FIN ESTRUCTURA

    INICIO
        Cola c
        SI (c.procesar_primero == NULO Y c.procesar_ultimo == NULO)
            IMPRIMIR("COLA_INICIAL_NULA: VERDADERO")
        FIN SI

        NodoCliente nuevo
        nuevo.nombre = "Cliente Uno"
        nuevo.proximo_cliente = NULO

        c.procesar_primero = nuevo
        c.procesar_ultimo = nuevo

        SI (c.procesar_primero != NULO Y c.procesar_ultimo != NULO)
            IMPRIMIR("COLA_ENCOLADO: " + c.procesar_primero.nombre)
        FIN SI

        // Desencolado del único elemento
        c.procesar_primero = c.procesar_primero.proximo_cliente
        SI (c.procesar_primero == NULO)
            c.procesar_ultimo = NULO
            IMPRIMIR("COLA_DESENCOLADA_NULA: VERDADERO")
        FIN SI
    FIN
    "#;

    let script = TempScript::new("cola_directa", code);

    for engine in ["vm", "ast"] {
        let res = run_cli_with_timeout(
            &["run", script.path_str(), "--engine", engine],
            Duration::from_secs(5),
        );

        assert!(!res.timed_out, "Timed out on engine {}", engine);
        assert!(res.success, "Failed on engine {}: {}", engine, res.stderr);
        assert!(res.stdout.contains("COLA_INICIAL_NULA: VERDADERO"));
        assert!(res.stdout.contains("COLA_ENCOLADO: Cliente Uno"));
        assert!(res.stdout.contains("COLA_DESENCOLADA_NULA: VERDADERO"));
    }
}

// =============================================================================
// Scenario 2: Traffic Avenue Simulation & semaforo.yaguarete
// =============================================================================

#[test]
fn test_e2e_semaforo_avenue_simulation_scenario() {
    // Verifies the complete Tier 4 traffic simulation scenario:
    // 1. Uninitialized avenue starts with primer_auto == NULO and ultimo_auto == NULO.
    // 2. cantidad_autos_avenida returns 0 for an empty avenue.
    // 3. Cars are enqueued sequentially without ghost autos (numero_auto: 0).
    // 4. Traversal terminates cleanly at NULO.
    let code = r#"
    ESTRUCTURA AUTO:
        ENTERO numero_auto
        AUTO proximo_auto
    FIN ESTRUCTURA

    ESTRUCTURA AVENIDA:
        CADENA semaforo
        AUTO primer_auto
        AUTO ultimo_auto
    FIN ESTRUCTURA

    FUNCION cantidad_autos_avenida(avenida: AVENIDA): ENTERO
        ENTERO contador = 0
        AUTO aux
        aux.proximo_auto = avenida.primer_auto

        MIENTRAS(aux.proximo_auto != NULO)
            aux = aux.proximo_auto
            contador++
        FIN MIENTRAS
        RETORNAR contador
    FIN FUNCION

    FUNCION agregar_a_cola(lista: AVENIDA, nuevo_auto: AUTO)
        SI (nuevo_auto == NULO)
            RETORNAR
        FIN SI

        nuevo_auto.proximo_auto = NULO

        SI (lista.ultimo_auto != NULO)
            lista.ultimo_auto.proximo_auto = nuevo_auto
            lista.ultimo_auto = nuevo_auto
        SINO
            lista.primer_auto = nuevo_auto
            lista.ultimo_auto = nuevo_auto
        FIN SI
        IMPRIMIR("Auto agregado: " + nuevo_auto)
    FIN FUNCION

    INICIO
        AVENIDA calle_1
        calle_1.semaforo = "rojo"

        // 1. Validación de calle vacía inicial en NULO
        SI (calle_1.primer_auto == NULO Y calle_1.ultimo_auto == NULO)
            IMPRIMIR("CALLE_VACIA_NULA: VERDADERO")
        FIN SI

        ENTERO autos_iniciales = cantidad_autos_avenida(calle_1)
        IMPRIMIR("CANTIDAD_INICIAL: " + autos_iniciales)

        // 2. Creación de autos de prueba
        AUTO vector_autos[3]
        PARA(ENTERO i = 0; i < 3; i++)
            AUTO a
            a.numero_auto = i + 1
            a.proximo_auto = NULO
            vector_autos[i] = a
        FIN PARA

        // 3. Encolado secuencial controlado (simulación de semáforo)
        PARA(ENTERO idx = 0; idx < 3; idx++)
            SI (calle_1.semaforo == "rojo" Y cantidad_autos_avenida(calle_1) < 3)
                SI (calle_1.primer_auto == NULO)
                    calle_1.primer_auto = vector_autos[idx]
                    calle_1.ultimo_auto = vector_autos[idx]
                    IMPRIMIR("Primer auto asignado: " + vector_autos[idx].numero_auto)
                SINO
                    agregar_a_cola(calle_1, vector_autos[idx])
                FIN SI
            FIN SI
        FIN PARA

        ENTERO autos_finales = cantidad_autos_avenida(calle_1)
        IMPRIMIR("CANTIDAD_FINAL: " + autos_finales)
        IMPRIMIR("ESTADO_AVENIDA: " + calle_1)
    FIN
    "#;

    let script = TempScript::new("semaforo_escenario", code);

    for engine in ["vm", "ast"] {
        let res = run_cli_with_timeout(
            &["run", script.path_str(), "--engine", engine],
            Duration::from_secs(5),
        );

        assert!(!res.timed_out, "Traffic scenario timed out on engine {}", engine);
        assert!(res.success, "Traffic scenario failed on engine {}: {}", engine, res.stderr);
        assert_eq!(res.exit_code, Some(0));

        assert!(res.stdout.contains("CALLE_VACIA_NULA: VERDADERO"));
        assert!(res.stdout.contains("CANTIDAD_INICIAL: 0"));
        assert!(res.stdout.contains("Primer auto asignado: 1"));
        assert!(res.stdout.contains("Auto agregado: AUTO{numero_auto: 2, proximo_auto: NULO}"));
        assert!(res.stdout.contains("Auto agregado: AUTO{numero_auto: 3, proximo_auto: NULO}"));
        assert!(res.stdout.contains("CANTIDAD_FINAL: 3"));
        assert!(!res.stdout.contains("numero_auto: 0"), "Ghost auto detected in output!");
    }
}

#[test]
fn test_e2e_semaforo_root_script_execution() {
    let semaforo_file = repo_root().join("semaforo.yaguarete");
    assert!(semaforo_file.exists(), "semaforo.yaguarete must exist in repository root");

    for engine in ["vm", "ast"] {
        let res = run_cli_with_timeout(
            &["run", semaforo_file.to_str().unwrap(), "--engine", engine],
            Duration::from_secs(4),
        );

        assert!(!res.timed_out, "semaforo.yaguarete timed out on engine {}", engine);
        assert!(res.success, "semaforo.yaguarete on engine {} failed: {}", engine, res.stderr);
        assert_eq!(res.exit_code, Some(0), "semaforo.yaguarete did not exit with code 0 on engine {}", engine);
        assert!(
            res.stdout.contains("Auto agregado: AUTO{numero_auto: 2, proximo_auto: NULO}"),
            "semaforo.yaguarete on engine {} missing auto 2 addition: {}",
            engine,
            res.stdout
        );
        assert!(
            res.stdout.contains("Auto agregado: AUTO{numero_auto: 3, proximo_auto: NULO}"),
            "semaforo.yaguarete on engine {} missing auto 3 addition: {}",
            engine,
            res.stdout
        );
        assert!(
            res.stdout.contains("Estado lista: AVENIDA{semaforo: 'rojo'"),
            "semaforo.yaguarete on engine {} missing avenue state: {}",
            engine,
            res.stdout
        );
    }
}

// =============================================================================
// Scenario 3: Singly Linked List Scenario
// =============================================================================

#[test]
fn test_e2e_singly_linked_list_traversal_vm_and_ast() {
    let code = r#"
    ESTRUCTURA Nodo:
        ENTERO valor
        Nodo proximo
    FIN ESTRUCTURA

    FUNCION contar_nodos(cabeza: Nodo): ENTERO
        ENTERO cuenta = 0
        Nodo actual
        actual = cabeza
        MIENTRAS(actual != NULO)
            cuenta++
            actual = actual.proximo
        FIN MIENTRAS
        RETORNAR cuenta
    FIN FUNCION

    INICIO
        Nodo n1
        n1.valor = 10
        n1.proximo = NULO

        Nodo n2
        n2.valor = 20
        n2.proximo = NULO
        n1.proximo = n2

        Nodo n3
        n3.valor = 30
        n3.proximo = NULO
        n2.proximo = n3

        // Recorrido de nodos
        Nodo actual = n1
        MIENTRAS(actual != NULO)
            IMPRIMIR("NODO_VALOR: " + actual.valor)
            actual = actual.proximo
        FIN MIENTRAS

        SI (actual == NULO)
            IMPRIMIR("RECORRIDO_TERMINADO_EN_NULO: VERDADERO")
        FIN SI

        ENTERO total = contar_nodos(n1)
        IMPRIMIR("TOTAL_NODOS: " + total)

        // Eliminación de cabeza avanzando puntero
        Nodo nueva_cabeza = n1.proximo
        IMPRIMIR("NUEVA_CABEZA: " + nueva_cabeza.valor)
        IMPRIMIR("TOTAL_RESTANTE: " + contar_nodos(nueva_cabeza))
    FIN
    "#;

    let script = TempScript::new("lista_enlazada", code);

    for engine in ["vm", "ast"] {
        let res = run_cli_with_timeout(
            &["run", script.path_str(), "--engine", engine],
            Duration::from_secs(5),
        );

        assert!(!res.timed_out, "Linked list test timed out on engine {}", engine);
        assert!(res.success, "Linked list test failed on engine {}: {}", engine, res.stderr);
        assert_eq!(res.exit_code, Some(0));

        assert!(res.stdout.contains("NODO_VALOR: 10"));
        assert!(res.stdout.contains("NODO_VALOR: 20"));
        assert!(res.stdout.contains("NODO_VALOR: 30"));
        assert!(res.stdout.contains("RECORRIDO_TERMINADO_EN_NULO: VERDADERO"));
        assert!(res.stdout.contains("TOTAL_NODOS: 3"));
        assert!(res.stdout.contains("NUEVA_CABEZA: 20"));
        assert!(res.stdout.contains("TOTAL_RESTANTE: 2"));
    }
}

// =============================================================================
// Scenario 4: Organizational Hierarchy Tree Scenario
// =============================================================================

#[test]
fn test_e2e_organizational_tree_leaf_detection_vm_and_ast() {
    let code = r#"
    ESTRUCTURA Empleado:
        CADENA cargo
        ENTERO id
        Empleado sub_izq
        Empleado sub_der
    FIN ESTRUCTURA

    INICIO
        Empleado ceo
        ceo.cargo = "Director Ejecutivo"
        ceo.id = 1

        // Verificación de que los punteros hijos inician por defecto en NULO
        SI (ceo.sub_izq == NULO Y ceo.sub_der == NULO)
            IMPRIMIR("CEO_ES_HOJA_INICIAL: VERDADERO")
        FIN SI

        // Creación y asignación de directores subordinados
        Empleado dir_tech
        dir_tech.cargo = "Director de Tecnologia"
        dir_tech.id = 101

        Empleado dir_fin
        dir_fin.cargo = "Director Financiero"
        dir_fin.id = 102

        ceo.sub_izq = dir_tech
        ceo.sub_der = dir_fin

        SI (ceo.sub_izq != NULO Y ceo.sub_der != NULO)
            IMPRIMIR("CEO_TIENE_SUBORDINADOS: VERDADERO")
        FIN SI

        // Los subordinados son hojas
        SI (ceo.sub_izq.sub_izq == NULO Y ceo.sub_izq.sub_der == NULO)
            IMPRIMIR("DIR_TECH_ES_HOJA: VERDADERO")
        FIN SI

        SI (ceo.sub_der.sub_izq == NULO Y ceo.sub_der.sub_der == NULO)
            IMPRIMIR("DIR_FIN_ES_HOJA: VERDADERO")
        FIN SI

        IMPRIMIR("SUB_IZQ_CARGO: " + ceo.sub_izq.cargo)
        IMPRIMIR("SUB_DER_CARGO: " + ceo.sub_der.cargo)
    FIN
    "#;

    let script = TempScript::new("arbol_jerarquia", code);

    for engine in ["vm", "ast"] {
        let res = run_cli_with_timeout(
            &["run", script.path_str(), "--engine", engine],
            Duration::from_secs(5),
        );

        assert!(!res.timed_out, "Tree test timed out on engine {}", engine);
        assert!(res.success, "Tree test failed on engine {}: {}", engine, res.stderr);
        assert_eq!(res.exit_code, Some(0));

        assert!(res.stdout.contains("CEO_ES_HOJA_INICIAL: VERDADERO"));
        assert!(res.stdout.contains("CEO_TIENE_SUBORDINADOS: VERDADERO"));
        assert!(res.stdout.contains("DIR_TECH_ES_HOJA: VERDADERO"));
        assert!(res.stdout.contains("DIR_FIN_ES_HOJA: VERDADERO"));
        assert!(res.stdout.contains("SUB_IZQ_CARGO: Director de Tecnologia"));
        assert!(res.stdout.contains("SUB_DER_CARGO: Director Financiero"));
    }
}

// =============================================================================
// Scenario 5: Banking Transaction Registry Scenario
// =============================================================================

#[test]
fn test_e2e_banking_transaction_registry_vm_and_ast() {
    let code = r#"
    ESTRUCTURA Transaccion:
        CADENA tipo
        REAL monto
        Transaccion anterior
    FIN ESTRUCTURA

    ESTRUCTURA CuentaBancaria:
        CADENA titular
        REAL saldo
        Transaccion ultima_transaccion
    FIN ESTRUCTURA

    INICIO
        CuentaBancaria cuenta
        cuenta.titular = "Ada Lovelace"

        // 1. Verificación de balance y puntero inicial en NULO
        SI (cuenta.saldo == NULO)
            IMPRIMIR("SALDO_INICIAL_NULO: VERDADERO")
        FIN SI
        SI (cuenta.ultima_transaccion == NULO)
            IMPRIMIR("SIN_TRANSACCIONES: VERDADERO")
        FIN SI

        // 2. Primer depósito: asignación sobre saldo NULO
        cuenta.saldo = 1000.0
        Transaccion tx1
        tx1.tipo = "DEPOSITO_INICIAL"
        tx1.monto = 1000.0
        tx1.anterior = NULO
        cuenta.ultima_transaccion = tx1

        IMPRIMIR("SALDO_TRAS_DEPOSITO_1: " + cuenta.saldo)

        // 3. Segundo depósito: mutación sobre saldo existente y encadenamiento
        cuenta.saldo = cuenta.saldo + 500.5
        Transaccion tx2
        tx2.tipo = "DEPOSITO_SUELDO"
        tx2.monto = 500.5
        tx2.anterior = cuenta.ultima_transaccion
        cuenta.ultima_transaccion = tx2

        IMPRIMIR("SALDO_TRAS_DEPOSITO_2: " + cuenta.saldo)

        // 4. Recorrido del historial de transacciones hacia atrás
        Transaccion actual = cuenta.ultima_transaccion
        ENTERO contador_tx = 0
        MIENTRAS(actual != NULO)
            contador_tx++
            IMPRIMIR("HISTORIAL_TX: " + actual.tipo + " MONTO: " + actual.monto)
            actual = actual.anterior
        FIN MIENTRAS

        IMPRIMIR("TOTAL_TRANSACCIONES: " + contador_tx)
    FIN
    "#;

    let script = TempScript::new("banco_registro", code);

    for engine in ["vm", "ast"] {
        let res = run_cli_with_timeout(
            &["run", script.path_str(), "--engine", engine],
            Duration::from_secs(5),
        );

        assert!(!res.timed_out, "Banking test timed out on engine {}", engine);
        assert!(res.success, "Banking test failed on engine {}: {}", engine, res.stderr);
        assert_eq!(res.exit_code, Some(0));

        assert!(res.stdout.contains("SALDO_INICIAL_NULO: VERDADERO"));
        assert!(res.stdout.contains("SIN_TRANSACCIONES: VERDADERO"));
        assert!(res.stdout.contains("SALDO_TRAS_DEPOSITO_1: 1000"));
        assert!(res.stdout.contains("SALDO_TRAS_DEPOSITO_2: 1500.5"));
        assert!(res.stdout.contains("HISTORIAL_TX: DEPOSITO_SUELDO MONTO: 500.5"));
        assert!(res.stdout.contains("HISTORIAL_TX: DEPOSITO_INICIAL MONTO: 1000"));
        assert!(res.stdout.contains("TOTAL_TRANSACCIONES: 2"));
    }
}

// =============================================================================
// Scenario 6: CLI Engine Flags & Diagnostics Validation
// =============================================================================

#[test]
fn test_e2e_cli_check_struct_valid_code() {
    let code = r#"
    ESTRUCTURA Registro:
        ENTERO id
        CADENA valor
    FIN ESTRUCTURA

    INICIO
        Registro r
        SI (r.id == NULO Y r.valor == NULO)
            IMPRIMIR("OK")
        FIN SI
    FIN
    "#;

    let script = TempScript::new("check_valido", code);

    let res = run_cli_with_timeout(
        &["check", script.path_str()],
        Duration::from_secs(5),
    );

    assert!(!res.timed_out);
    assert!(res.success, "yaguarete check failed: {}", res.stderr);
    assert_eq!(res.exit_code, Some(0));
    assert!(res.stdout.contains("[OK] Sin errores"));
}

#[test]
fn test_e2e_cli_check_with_lenient_flag() {
    let code = r#"
    ESTRUCTURA Punto:
        REAL x
        REAL y_coord
    FIN ESTRUCTURA

    INICIO
        Punto p
        SI (p.x == NULO)
            IMPRIMIR("NULO")
        FIN SI
    FIN
    "#;

    let script = TempScript::new("check_lenient", code);

    let res = run_cli_with_timeout(
        &["check", script.path_str(), "--lenient"],
        Duration::from_secs(5),
    );

    assert!(!res.timed_out);
    assert!(res.success);
    assert_eq!(res.exit_code, Some(0));
    assert!(res.stdout.contains("[OK] Sin errores"));
}

#[test]
fn test_e2e_cli_exit_code_0_on_success() {
    let code = r#"
    ESTRUCTURA Par:
        ENTERO a
        ENTERO b
    FIN ESTRUCTURA

    INICIO
        Par p
        p.a = 100
        p.b = 200
        IMPRIMIR("SUMA: " + (p.a + p.b))
    FIN
    "#;

    let script = TempScript::new("exit_cero", code);

    let res = run_cli_with_timeout(
        &["run", script.path_str()],
        Duration::from_secs(5),
    );

    assert!(!res.timed_out);
    assert!(res.success);
    assert_eq!(res.exit_code, Some(0));
    assert!(res.stdout.contains("SUMA: 300"));
}

#[test]
fn test_e2e_cli_exit_code_nonzero_on_syntax_error() {
    // Missing closing keyword
    let code = r#"
    ESTRUCTURA Invalida:
        ENTERO x
    // Faltante FIN ESTRUCTURA
    INICIO
        Invalida i
    FIN
    "#;

    let script = TempScript::new("error_sintaxis", code);

    let res = run_cli_with_timeout(
        &["run", script.path_str()],
        Duration::from_secs(5),
    );

    assert!(!res.timed_out);
    assert!(!res.success, "Expected syntax error exit code");
    assert_eq!(res.exit_code, Some(1));
    assert!(
        res.stderr.contains("Error de sintaxis") || res.stderr.contains("Se esperaba 'FIN ESTRUCTURA'"),
        "Unexpected stderr: {}",
        res.stderr
    );
}

#[test]
fn test_e2e_cli_exit_code_nonzero_on_null_read_dereference() {
    let code = r#"
    ESTRUCTURA Detalle:
        ENTERO codigo
    FIN ESTRUCTURA

    ESTRUCTURA Cabecera:
        Detalle detalle
    FIN ESTRUCTURA

    INICIO
        Cabecera cab
        // cab.detalle es NULO; intentar leer miembro debe provocar error de ejecución
        ENTERO c = cab.detalle.codigo
        IMPRIMIR("NO_DEBE_LLEGAR_AQUI: " + c)
    FIN
    "#;

    let script = TempScript::new("error_deref_read", code);

    for engine in ["vm", "ast"] {
        let res = run_cli_with_timeout(
            &["run", script.path_str(), "--engine", engine],
            Duration::from_secs(5),
        );

        assert!(!res.timed_out, "Timed out on engine {}", engine);
        assert!(!res.success, "Expected runtime error on engine {}", engine);
        assert_eq!(res.exit_code, Some(1));
        assert!(
            res.stderr.contains("referencia nula (NULO)") || res.stderr.contains("Acceso a miembro"),
            "Engine {} stderr mismatch: {}",
            engine,
            res.stderr
        );
        assert!(!res.stdout.contains("NO_DEBE_LLEGAR_AQUI"));
    }
}

#[test]
fn test_e2e_cli_exit_code_nonzero_on_null_write_dereference() {
    let code = r#"
    ESTRUCTURA Detalle:
        ENTERO codigo
    FIN ESTRUCTURA

    ESTRUCTURA Cabecera:
        Detalle detalle
    FIN ESTRUCTURA

    INICIO
        Cabecera cab
        // cab.detalle es NULO; intentar asignar miembro debe provocar error de ejecución
        cab.detalle.codigo = 999
        IMPRIMIR("NO_DEBE_LLEGAR_AQUI")
    FIN
    "#;

    let script = TempScript::new("error_deref_write", code);

    for engine in ["vm", "ast"] {
        let res = run_cli_with_timeout(
            &["run", script.path_str(), "--engine", engine],
            Duration::from_secs(5),
        );

        assert!(!res.timed_out, "Timed out on engine {}", engine);
        assert!(!res.success, "Expected runtime error on engine {}", engine);
        assert_eq!(res.exit_code, Some(1));
        assert!(
            res.stderr.contains("referencia nula (NULO)") || res.stderr.contains("Asignación a miembro"),
            "Engine {} stderr mismatch: {}",
            engine,
            res.stderr
        );
        assert!(!res.stdout.contains("NO_DEBE_LLEGAR_AQUI"));
    }
}

#[test]
fn test_e2e_cli_default_engine_is_vm() {
    let code = r#"
    ESTRUCTURA Dato:
        ENTERO n
    FIN ESTRUCTURA

    INICIO
        Dato d
        IMPRIMIR("MOTOR_DEFAULT_OK: " + (d.n == NULO))
    FIN
    "#;

    let script = TempScript::new("motor_default", code);

    // Executed without --engine flag
    let res = run_cli_with_timeout(
        &["run", script.path_str()],
        Duration::from_secs(5),
    );

    assert!(!res.timed_out);
    assert!(res.success);
    assert_eq!(res.exit_code, Some(0));
    assert!(res.stdout.contains("MOTOR_DEFAULT_OK: VERDADERO"));
}

// =============================================================================
// Scenario 7: Adversarial Verification & Boundary Conditions
// =============================================================================

#[test]
fn test_e2e_adversarial_deeply_nested_structs_no_autoinstantiation() {
    let code_valid = r#"
    ESTRUCTURA Nivel4:
        ENTERO val4
    FIN ESTRUCTURA

    ESTRUCTURA Nivel3:
        Nivel4 h4
    FIN ESTRUCTURA

    ESTRUCTURA Nivel2:
        Nivel3 h3
    FIN ESTRUCTURA

    ESTRUCTURA Nivel1:
        Nivel2 h2
    FIN ESTRUCTURA

    INICIO
        Nivel1 n1
        SI (n1.h2 == NULO)
            IMPRIMIR("NIVEL_2_NULO: VERDADERO")
        FIN SI

        Nivel2 n2
        SI (n2.h3 == NULO)
            IMPRIMIR("NIVEL_3_NULO: VERDADERO")
        FIN SI

        Nivel3 n3
        SI (n3.h4 == NULO)
            IMPRIMIR("NIVEL_4_NULO: VERDADERO")
        FIN SI

        Nivel4 n4
        n4.val4 = 9999
        n3.h4 = n4
        n2.h3 = n3
        n1.h2 = n2

        SI (n1.h2.h3.h4.val4 == 9999)
            IMPRIMIR("CADENA_PROFUNDA_ACCEDIDA: VERDADERO")
        FIN SI
    FIN
    "#;

    let script = TempScript::new("adversarial_anidado", code_valid);

    for engine in ["vm", "ast"] {
        let res = run_cli_with_timeout(
            &["run", script.path_str(), "--engine", engine],
            Duration::from_secs(5),
        );

        assert!(!res.timed_out, "Timed out on engine {}", engine);
        assert!(res.success, "Failed on engine {}: {}", engine, res.stderr);
        assert_eq!(res.exit_code, Some(0));

        assert!(res.stdout.contains("NIVEL_2_NULO: VERDADERO"));
        assert!(res.stdout.contains("NIVEL_3_NULO: VERDADERO"));
        assert!(res.stdout.contains("NIVEL_4_NULO: VERDADERO"));
        assert!(res.stdout.contains("CADENA_PROFUNDA_ACCEDIDA: VERDADERO"));
    }
}

#[test]
fn test_e2e_adversarial_primitives_not_confused_with_defaults() {
    let code = r#"
    ESTRUCTURA Primitivos:
        ENTERO e
        REAL r
        CADENA c
        BOOLEANO b
    FIN ESTRUCTURA

    INICIO
        Primitivos p

        // NULO no debe ser igual a 0, 0.0, "" o FALSO
        SI (p.e == 0)
            IMPRIMIR("ERROR: e es cero")
        SINO
            IMPRIMIR("E_NO_ES_CERO: VERDADERO")
        FIN SI

        SI (p.r == 0.0)
            IMPRIMIR("ERROR: r es cero")
        SINO
            IMPRIMIR("R_NO_ES_CERO: VERDADERO")
        FIN SI

        SI (p.c == "")
            IMPRIMIR("ERROR: c es vacio")
        SINO
            IMPRIMIR("C_NO_ES_VACIO: VERDADERO")
        FIN SI

        SI (p.b == FALSO)
            IMPRIMIR("ERROR: b es falso")
        SINO
            IMPRIMIR("B_NO_ES_FALSO: VERDADERO")
        FIN SI
    FIN
    "#;

    let script = TempScript::new("adversarial_primitivos", code);

    for engine in ["vm", "ast"] {
        let res = run_cli_with_timeout(
            &["run", script.path_str(), "--engine", engine],
            Duration::from_secs(5),
        );

        assert!(!res.timed_out, "Timed out on engine {}", engine);
        assert!(res.success, "Failed on engine {}: {}", engine, res.stderr);
        assert_eq!(res.exit_code, Some(0));

        assert!(res.stdout.contains("E_NO_ES_CERO: VERDADERO"));
        assert!(res.stdout.contains("R_NO_ES_CERO: VERDADERO"));
        assert!(res.stdout.contains("C_NO_ES_VACIO: VERDADERO"));
        assert!(res.stdout.contains("B_NO_ES_FALSO: VERDADERO"));
        assert!(!res.stdout.contains("ERROR:"));
    }
}
