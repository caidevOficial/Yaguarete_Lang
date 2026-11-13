//! Adversarial CLI Dual-Engine Stress & Parity Test Suite (Challenger 2)
//!
//! Verifies:
//! 1. Parity between `--engine vm` and `--engine ast` on struct operations.
//! 2. Exact match of stdout and exit code 0 on valid struct lifecycle scripts.
//! 3. Exact match of exit code 1 and controlled null dereference diagnostics in stderr.
//! 4. Dual-engine parity on `colas.yaguarete` and `semaforo.yaguarete`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

#[allow(dead_code)]
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
        let file_name = format!("yaguarete_adv_cli_m5_{}_{}_{}.yaguarete", prefix, pid, id);
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
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(e) => panic!("Error monitoring child process: {}", e),
        }
    }
}

// -----------------------------------------------------------------------------
// 1. Dual-Engine Parity on Uninitialized Struct Lifecycle
// -----------------------------------------------------------------------------

#[test]
fn test_adv_cli_dual_engine_uninitialized_structs_parity() {
    let script = TempScript::new(
        "uninit_lifecycle",
        r#"
    ESTRUCTURA Nodo:
        ENTERO id
        CADENA etiqueta
        Nodo sig
    FIN ESTRUCTURA

    INICIO
        Nodo n
        SI (n.id == NULO Y n.etiqueta == NULO Y n.sig == NULO)
            IMPRIMIR("INIT_ALL_NULO")
        FIN SI

        n.id = 42
        n.etiqueta = "TestNode"
        SI (n.id != NULO Y n.etiqueta != NULO)
            IMPRIMIR("FIELDS_ASSIGNED:", n.id, n.etiqueta)
        FIN SI
    FIN
    "#,
    );

    let res_vm = run_cli_with_timeout(&["run", script.path_str(), "--engine", "vm"], Duration::from_secs(10));
    let res_ast = run_cli_with_timeout(&["run", script.path_str(), "--engine", "ast"], Duration::from_secs(10));

    assert_eq!(res_vm.exit_code, Some(0), "VM exited with error: {}", res_vm.stderr);
    assert_eq!(res_ast.exit_code, Some(0), "AST exited with error: {}", res_ast.stderr);

    assert!(res_vm.stdout.contains("INIT_ALL_NULO"));
    assert!(res_vm.stdout.contains("FIELDS_ASSIGNED: 42 TestNode"));
    assert_eq!(res_vm.stdout, res_ast.stdout, "CLI stdout divergence between VM and AST!");
}

// -----------------------------------------------------------------------------
// 2. Dual-Engine Parity on Reassignment to NULO
// -----------------------------------------------------------------------------

#[test]
fn test_adv_cli_dual_engine_reassignment_to_null_parity() {
    let script = TempScript::new(
        "reassign_nulo",
        r#"
    ESTRUCTURA Item:
        CADENA codigo
    FIN ESTRUCTURA

    INICIO
        Item it
        it.codigo = "ART-001"
        IMPRIMIR("CODIGO_INICIAL:", it.codigo)

        it.codigo = NULO
        SI (it.codigo == NULO)
            IMPRIMIR("CODIGO_RESET_NULO_VERDADERO")
        FIN SI
    FIN
    "#,
    );

    let res_vm = run_cli_with_timeout(&["run", script.path_str(), "--engine", "vm"], Duration::from_secs(10));
    let res_ast = run_cli_with_timeout(&["run", script.path_str(), "--engine", "ast"], Duration::from_secs(10));

    assert_eq!(res_vm.exit_code, Some(0));
    assert_eq!(res_ast.exit_code, Some(0));
    assert!(res_vm.stdout.contains("CODIGO_INICIAL: ART-001"));
    assert!(res_vm.stdout.contains("CODIGO_RESET_NULO_VERDADERO"));
    assert_eq!(res_vm.stdout, res_ast.stdout);
}

// -----------------------------------------------------------------------------
// 3. Dual-Engine Parity on Controlled Null Read Dereference Error (Exit Code 1)
// -----------------------------------------------------------------------------

#[test]
fn test_adv_cli_dual_engine_null_read_dereference_error() {
    let script = TempScript::new(
        "null_read_err",
        r#"
    ESTRUCTURA Arbol:
        ENTERO val
        Arbol izq
    FIN ESTRUCTURA

    INICIO
        Arbol a
        IMPRIMIR(a.izq.val)
    FIN
    "#,
    );

    let res_vm = run_cli_with_timeout(&["run", script.path_str(), "--engine", "vm"], Duration::from_secs(10));
    let res_ast = run_cli_with_timeout(&["run", script.path_str(), "--engine", "ast"], Duration::from_secs(10));

    assert_eq!(res_vm.exit_code, Some(1), "VM did not exit with code 1");
    assert_eq!(res_ast.exit_code, Some(1), "AST did not exit with code 1");

    assert!(
        res_vm.stderr.contains("referencia nula (NULO)"),
        "VM stderr does not contain null reference diagnostic: {}",
        res_vm.stderr
    );
    assert!(
        res_ast.stderr.contains("referencia nula (NULO)"),
        "AST stderr does not contain null reference diagnostic: {}",
        res_ast.stderr
    );
}

// -----------------------------------------------------------------------------
// 4. Dual-Engine Parity on Controlled Null Write Dereference Error (Exit Code 1)
// -----------------------------------------------------------------------------

#[test]
fn test_adv_cli_dual_engine_null_write_dereference_error() {
    let script = TempScript::new(
        "null_write_err",
        r#"
    ESTRUCTURA Arbol:
        ENTERO val
        Arbol der
    FIN ESTRUCTURA

    INICIO
        Arbol a
        a.der.val = 99
    FIN
    "#,
    );

    let res_vm = run_cli_with_timeout(&["run", script.path_str(), "--engine", "vm"], Duration::from_secs(10));
    let res_ast = run_cli_with_timeout(&["run", script.path_str(), "--engine", "ast"], Duration::from_secs(10));

    assert_eq!(res_vm.exit_code, Some(1), "VM did not exit with code 1");
    assert_eq!(res_ast.exit_code, Some(1), "AST did not exit with code 1");

    assert!(
        res_vm.stderr.contains("referencia nula (NULO)"),
        "VM stderr does not contain null reference diagnostic: {}",
        res_vm.stderr
    );
    assert!(
        res_ast.stderr.contains("referencia nula (NULO)"),
        "AST stderr does not contain null reference diagnostic: {}",
        res_ast.stderr
    );
}

// -----------------------------------------------------------------------------
// 5. Dual-Engine Parity on Real-World colas.yaguarete & semaforo.yaguarete
// -----------------------------------------------------------------------------

#[test]
fn test_adv_cli_colas_exact_parity() {
    let colas_path = repo_root().join("colas.yaguarete");
    assert!(colas_path.exists());

    let res_vm = run_cli_with_timeout(&["run", colas_path.to_str().unwrap(), "--engine", "vm"], Duration::from_secs(15));
    let res_ast = run_cli_with_timeout(&["run", colas_path.to_str().unwrap(), "--engine", "ast"], Duration::from_secs(15));

    assert_eq!(res_vm.exit_code, Some(0));
    assert_eq!(res_ast.exit_code, Some(0));
    assert_eq!(res_vm.stdout, res_ast.stdout, "Divergence between VM and AST on colas.yaguarete");
}

#[test]
fn test_adv_cli_semaforo_exact_parity() {
    let semaforo_path = repo_root().join("semaforo.yaguarete");
    assert!(semaforo_path.exists());

    let res_vm = run_cli_with_timeout(&["run", semaforo_path.to_str().unwrap(), "--engine", "vm"], Duration::from_secs(15));
    let res_ast = run_cli_with_timeout(&["run", semaforo_path.to_str().unwrap(), "--engine", "ast"], Duration::from_secs(15));

    assert_eq!(res_vm.exit_code, Some(0));
    assert_eq!(res_ast.exit_code, Some(0));
    assert_eq!(res_vm.stdout, res_ast.stdout, "Divergence between VM and AST on semaforo.yaguarete");
}
