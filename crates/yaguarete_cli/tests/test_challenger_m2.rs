//! Adversarial Challenger Test Suite for Milestone 2 (M2_Compatibility_Validation)
//!
//! Empirical Stress Harnesses:
//! 1. Rapid repeated execution (50x) of `colas.yaguarete` under VM and AST engines.
//! 2. Rapid repeated execution (50x) of `semaforo.yaguarete` under VM and AST engines.
//! 3. Concurrent multi-threaded execution of `semaforo.yaguarete` and `colas.yaguarete`.
//! 4. Empty queue underflow and recovery: repeatedly dequeuing empty queue, re-enqueuing, and verifying pointer state.
//! 5. Avenue with zero cars and multi-avenue traffic light cycles.
//! 6. Complex nested loops with dynamic pointer mutation and `== NULO` / `!= NULO` termination conditions.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct CliResult {
    success: bool,
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
    timed_out: bool,
}

struct TempScript {
    path: PathBuf,
}

impl TempScript {
    fn new(prefix: &str, content: &str) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let file_name = format!("yaguarete_adv_m2_{}_{}_{}.yaguarete", prefix, pid, id);
        let path = std::env::temp_dir().join(file_name);
        fs::write(&path, content).unwrap_or_else(|e| {
            panic!("Failed to write temporary test script to {:?}: {}", path, e)
        });
        Self { path }
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

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates directory")
        .parent()
        .expect("repo root directory")
        .to_path_buf()
}

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
                thread::sleep(Duration::from_millis(30));
            }
            Err(e) => {
                panic!("Error waiting on yaguarete child process: {}", e);
            }
        }
    }
}

// =============================================================================
// Stress Harness 1: Rapid Repeated Executions (50x)
// =============================================================================

#[test]
fn test_adv_rapid_repeated_execution_colas_vm() {
    let script = repo_root().join("colas.yaguarete");
    assert!(script.exists(), "colas.yaguarete must exist");

    for i in 1..=50 {
        let res = run_cli_with_timeout(
            &["run", script.to_str().unwrap(), "--engine", "vm"],
            Duration::from_secs(4),
        );
        assert!(!res.timed_out, "colas.yaguarete timed out on VM at iteration {}", i);
        assert!(res.success, "colas.yaguarete failed on VM at iteration {}: {}", i, res.stderr);
        assert_eq!(res.exit_code, Some(0));
        assert!(res.stdout.contains("Ultimo de la cola: NodoCliente{nombre: 4, proximo_cliente: NULO}"));
    }
}

#[test]
fn test_adv_rapid_repeated_execution_colas_ast() {
    let script = repo_root().join("colas.yaguarete");
    assert!(script.exists(), "colas.yaguarete must exist");

    for i in 1..=50 {
        let res = run_cli_with_timeout(
            &["run", script.to_str().unwrap(), "--engine", "ast"],
            Duration::from_secs(4),
        );
        assert!(!res.timed_out, "colas.yaguarete timed out on AST at iteration {}", i);
        assert!(res.success, "colas.yaguarete failed on AST at iteration {}: {}", i, res.stderr);
        assert_eq!(res.exit_code, Some(0));
        assert!(res.stdout.contains("Ultimo de la cola: NodoCliente{nombre: 4, proximo_cliente: NULO}"));
    }
}

#[test]
fn test_adv_rapid_repeated_execution_semaforo_vm() {
    let script = repo_root().join("semaforo.yaguarete");
    assert!(script.exists(), "semaforo.yaguarete must exist");

    for i in 1..=50 {
        let res = run_cli_with_timeout(
            &["run", script.to_str().unwrap(), "--engine", "vm"],
            Duration::from_secs(4),
        );
        assert!(!res.timed_out, "semaforo.yaguarete timed out on VM at iteration {}", i);
        assert!(res.success, "semaforo.yaguarete failed on VM at iteration {}: {}", i, res.stderr);
        assert_eq!(res.exit_code, Some(0));
        assert!(res.stdout.contains("Auto agregado: AUTO{numero_auto: 3, proximo_auto: NULO}"));
    }
}

#[test]
fn test_adv_rapid_repeated_execution_semaforo_ast() {
    let script = repo_root().join("semaforo.yaguarete");
    assert!(script.exists(), "semaforo.yaguarete must exist");

    for i in 1..=50 {
        let res = run_cli_with_timeout(
            &["run", script.to_str().unwrap(), "--engine", "ast"],
            Duration::from_secs(4),
        );
        assert!(!res.timed_out, "semaforo.yaguarete timed out on AST at iteration {}", i);
        assert!(res.success, "semaforo.yaguarete failed on AST at iteration {}: {}", i, res.stderr);
        assert_eq!(res.exit_code, Some(0));
        assert!(res.stdout.contains("Auto agregado: AUTO{numero_auto: 3, proximo_auto: NULO}"));
    }
}

// =============================================================================
// Stress Harness 2: Concurrent Multi-Threaded Execution
// =============================================================================

#[test]
fn test_adv_concurrent_execution_dual_engine() {
    let colas_path = Arc::new(repo_root().join("colas.yaguarete").to_str().unwrap().to_string());
    let semaforo_path = Arc::new(repo_root().join("semaforo.yaguarete").to_str().unwrap().to_string());

    let mut handles = Vec::new();

    // Spawn 8 parallel threads: 4 running colas (2 VM, 2 AST) and 4 running semaforo (2 VM, 2 AST)
    for t in 0..8 {
        let colas = Arc::clone(&colas_path);
        let semaforo = Arc::clone(&semaforo_path);
        let handle = thread::spawn(move || {
            let (target_file, engine) = match t % 4 {
                0 => (&*colas, "vm"),
                1 => (&*colas, "ast"),
                2 => (&*semaforo, "vm"),
                _ => (&*semaforo, "ast"),
            };

            for _ in 0..5 {
                let res = run_cli_with_timeout(
                    &["run", target_file, "--engine", engine],
                    Duration::from_secs(5),
                );
                assert!(!res.timed_out, "Thread {} timed out on {} with engine {}", t, target_file, engine);
                assert!(res.success, "Thread {} failed on {} with engine {}: {}", t, target_file, engine, res.stderr);
                assert_eq!(res.exit_code, Some(0));
            }
        });
        handles.push(handle);
    }

    for (idx, handle) in handles.into_iter().enumerate() {
        handle.join().unwrap_or_else(|_| panic!("Thread {} panicked during concurrent stress test", idx));
    }
}

// =============================================================================
// Stress Harness 3: Adversarial Queue Underflow and Recovery
// =============================================================================

#[test]
fn test_adv_queue_repeated_underflow_and_repopulation() {
    let code = r#"
    ESTRUCTURA Nodo:
        ENTERO val
        Nodo prox
    FIN ESTRUCTURA

    ESTRUCTURA Cola:
        Nodo primero
        Nodo ultimo
    FIN ESTRUCTURA

    FUNCION desencolar(c: Cola): Nodo
        SI (c.primero == NULO)
            RETORNAR NULO
        FIN SI
        Nodo res = c.primero
        c.primero = c.primero.prox
        SI (c.primero == NULO)
            c.ultimo = NULO
        FIN SI
        res.prox = NULO
        RETORNAR res
    FIN FUNCION

    FUNCION encolar(c: Cola, n: Nodo)
        SI (n == NULO)
            RETORNAR
        FIN SI
        n.prox = NULO
        SI (c.ultimo != NULO)
            c.ultimo.prox = n
            c.ultimo = n
        SINO
            c.primero = n
            c.ultimo = n
        FIN SI
    FIN FUNCION

    INICIO
        Cola q
        // Aseguramos que inicialmente todo es nulo por defecto
        IMPRIMIR("Q_INI_PRIMERO_NULO: " + (q.primero == NULO))
        IMPRIMIR("Q_INI_ULTIMO_NULO: " + (q.ultimo == NULO))

        // Desencolamos 10 veces en cola vacía
        PARA(ENTERO i = 0; i < 10; i++)
            Nodo vacio = desencolar(q)
            IMPRIMIR("POP_VACIO_" + i + ": " + (vacio == NULO))
        FIN PARA

        // Encolamos 3 elementos
        PARA(ENTERO j = 1; j <= 3; j++)
            Nodo item
            item.val = j * 10
            encolar(q, item)
        FIN PARA

        IMPRIMIR("Q_MID_PRIMERO_NO_NULO: " + (q.primero != NULO))
        IMPRIMIR("Q_MID_VAL_1: " + q.primero.val)
        IMPRIMIR("Q_MID_VAL_ULTIMO: " + q.ultimo.val)

        // Desencolamos los 3 elementos
        Nodo n1 = desencolar(q)
        Nodo n2 = desencolar(q)
        Nodo n3 = desencolar(q)
        IMPRIMIR("POP_VAL_1: " + n1.val)
        IMPRIMIR("POP_VAL_2: " + n2.val)
        IMPRIMIR("POP_VAL_3: " + n3.val)

        // Ahora la cola debe estar vacía nuevamente
        IMPRIMIR("Q_EMPTY_PRIMERO_NULO: " + (q.primero == NULO))
        IMPRIMIR("Q_EMPTY_ULTIMO_NULO: " + (q.ultimo == NULO))

        // Desencolamos nuevamente sobre la cola que volvió a quedar vacía
        Nodo extra = desencolar(q)
        IMPRIMIR("POP_AFTER_EMPTY: " + (extra == NULO))

        // Re-encolamos un nuevo elemento para verificar recuperación completa
        Nodo nuevo
        nuevo.val = 999
        encolar(q, nuevo)
        IMPRIMIR("RECOVERY_VAL: " + q.primero.val)
        IMPRIMIR("RECOVERY_SAME: " + (q.primero.val == q.ultimo.val))
    FIN
    "#;

    let script = TempScript::new("queue_underflow", code);

    for engine in ["vm", "ast"] {
        let res = run_cli_with_timeout(
            &["run", script.path_str(), "--engine", engine],
            Duration::from_secs(5),
        );
        assert!(!res.timed_out, "Queue underflow timed out on engine {}", engine);
        assert!(res.success, "Queue underflow failed on engine {}: {}", engine, res.stderr);
        assert_eq!(res.exit_code, Some(0));

        assert!(res.stdout.contains("Q_INI_PRIMERO_NULO: VERDADERO"));
        assert!(res.stdout.contains("Q_INI_ULTIMO_NULO: VERDADERO"));
        assert!(res.stdout.contains("POP_VACIO_0: VERDADERO"));
        assert!(res.stdout.contains("POP_VACIO_9: VERDADERO"));
        assert!(res.stdout.contains("Q_MID_VAL_1: 10"));
        assert!(res.stdout.contains("Q_MID_VAL_ULTIMO: 30"));
        assert!(res.stdout.contains("POP_VAL_1: 10"));
        assert!(res.stdout.contains("POP_VAL_2: 20"));
        assert!(res.stdout.contains("POP_VAL_3: 30"));
        assert!(res.stdout.contains("Q_EMPTY_PRIMERO_NULO: VERDADERO"));
        assert!(res.stdout.contains("Q_EMPTY_ULTIMO_NULO: VERDADERO"));
        assert!(res.stdout.contains("POP_AFTER_EMPTY: VERDADERO"));
        assert!(res.stdout.contains("RECOVERY_VAL: 999"));
        assert!(res.stdout.contains("RECOVERY_SAME: VERDADERO"));
    }
}

// =============================================================================
// Stress Harness 4: Zero-Car Empty Avenues & Multi-Avenue Traffic Cycles
// =============================================================================

#[test]
fn test_adv_traffic_zero_cars_and_multi_avenue_cycles() {
    let code = r#"
    ESTRUCTURA Auto:
        ENTERO id
        Auto sig
    FIN ESTRUCTURA

    ESTRUCTURA Avenida:
        CADENA nombre
        CADENA semaforo
        Auto primero
        Auto ultimo
    FIN ESTRUCTURA

    FUNCION contar_autos(av: Avenida): ENTERO
        ENTERO c = 0
        Auto actual = av.primero
        MIENTRAS (actual != NULO)
            c++
            actual = actual.sig
        FIN MIENTRAS
        RETORNAR c
    FIN FUNCION

    INICIO
        Avenida av_vacia
        av_vacia.nombre = "Avenida Vacia"
        av_vacia.semaforo = "rojo"

        // Conteo inicial en avenida completamente vacía (todos los punteros son NULO)
        ENTERO c0 = contar_autos(av_vacia)
        IMPRIMIR("VACIA_COUNT: " + c0)
        IMPRIMIR("VACIA_PRIMERO_NULO: " + (av_vacia.primero == NULO))
        IMPRIMIR("VACIA_ULTIMO_NULO: " + (av_vacia.ultimo == NULO))

        // Simulación de 5 iteraciones de semáforo sin autos llegando
        PARA(ENTERO ciclo = 1; ciclo <= 5; ciclo++)
            SI (av_vacia.semaforo == "rojo")
                av_vacia.semaforo = "verde"
            SINO
                av_vacia.semaforo = "rojo"
            FIN SI
            ENTERO c_ciclo = contar_autos(av_vacia)
            IMPRIMIR("CICLO_" + ciclo + "_COUNT: " + c_ciclo)
        FIN PARA

        IMPRIMIR("FIN_VACIA_OK")
    FIN
    "#;

    let script = TempScript::new("zero_cars", code);

    for engine in ["vm", "ast"] {
        let res = run_cli_with_timeout(
            &["run", script.path_str(), "--engine", engine],
            Duration::from_secs(4),
        );
        assert!(!res.timed_out, "Zero cars scenario timed out on engine {}", engine);
        assert!(res.success, "Zero cars scenario failed on engine {}: {}", engine, res.stderr);
        assert_eq!(res.exit_code, Some(0));

        assert!(res.stdout.contains("VACIA_COUNT: 0"));
        assert!(res.stdout.contains("VACIA_PRIMERO_NULO: VERDADERO"));
        assert!(res.stdout.contains("VACIA_ULTIMO_NULO: VERDADERO"));
        assert!(res.stdout.contains("CICLO_1_COUNT: 0"));
        assert!(res.stdout.contains("CICLO_5_COUNT: 0"));
        assert!(res.stdout.contains("FIN_VACIA_OK"));
    }
}

// =============================================================================
// Stress Harness 5: Complex Nested Loops with Dynamic Pointer Mutation
// =============================================================================

#[test]
fn test_adv_nested_loops_dynamic_pointer_mutation_null_checks() {
    let code = r#"
    ESTRUCTURA Celda:
        ENTERO val
        Celda link
    FIN ESTRUCTURA

    INICIO
        ENTERO suma_total = 0

        // Ciclo externo PARA
        PARA(ENTERO ronda = 1; ronda <= 5; ronda++)
            Celda cabecera
            cabecera.val = ronda

            // Ciclo interno: construimos una cadena de celdas
            Celda actual = cabecera
            PARA(ENTERO paso = 1; paso <= 4; paso++)
                Celda siguiente
                siguiente.val = ronda * 10 + paso
                actual.link = siguiente
                actual = siguiente
            FIN PARA

            // Recorremos y sumamos con MIENTRAS comprobando != NULO
            Celda lector = cabecera
            ENTERO celdas_vistas = 0
            MIENTRAS (lector != NULO)
                suma_total = suma_total + lector.val
                celdas_vistas++
                lector = lector.link
            FIN MIENTRAS

            IMPRIMIR("RONDA_" + ronda + "_CELDAS: " + celdas_vistas)

            // Mutamos la cabecera: rompemos el enlace para que apunte a NULO
            cabecera.link = NULO
            IMPRIMIR("RONDA_" + ronda + "_CABECERA_LINK_NULO: " + (cabecera.link == NULO))
        FIN PARA

        IMPRIMIR("SUMA_TOTAL_FINAL: " + suma_total)
    FIN
    "#;

    let script = TempScript::new("nested_mutation", code);

    for engine in ["vm", "ast"] {
        let res = run_cli_with_timeout(
            &["run", script.path_str(), "--engine", engine],
            Duration::from_secs(5),
        );
        assert!(!res.timed_out, "Nested mutation timed out on engine {}", engine);
        assert!(res.success, "Nested mutation failed on engine {}: {}", engine, res.stderr);
        assert_eq!(res.exit_code, Some(0));

        assert!(res.stdout.contains("RONDA_1_CELDAS: 5"));
        assert!(res.stdout.contains("RONDA_5_CELDAS: 5"));
        assert!(res.stdout.contains("RONDA_1_CABECERA_LINK_NULO: VERDADERO"));
        assert!(res.stdout.contains("RONDA_5_CABECERA_LINK_NULO: VERDADERO"));
        // Sum calculation:
        // Round r: r + sum(p=1..4: r*10+p) = r + 40*r + 10 = 41*r + 10.
        // sum for r=1..5: 41*(15) + 50 = 615 + 50 = 665.
        assert!(res.stdout.contains("SUMA_TOTAL_FINAL: 665"));
    }
}
