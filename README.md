# Evil Maid Attack & EM Shell

Este repositorio contiene el desarrollo de la **Tarea 1** para el curso *Principios de Seguridad en Sistemas Operativos* de la **Maestría en Ciberseguridad** del Tecnológico de Costa Rica (TEC), correspondiente al ciclo académico de 2026.

El proyecto es desarrollado de forma individual por Kevin Andres Vasquez Benavies, cuyo objetivo es comprender los vectores de ataque físicos en dispositivos desatendidos y la mecánica de persistencia/redirección de flujos a nivel de sistema operativo.

## Características del Proyecto

El sistema está desarrollado íntegramente en **Rust** y consta de dos componentes principales compilados para arquitectura **x86**:

1. **Vector de Ataque al Arranque (Evil Maid):** Simulación del compromiso de un sistema operativo durante su fase de booteo.
2. **EM Shell:** Un binario persistente que se carga en memoria tras el inicio del sistema operativo e interactúa de manera nativa con la red para establecer un *reverse shell* (socket TCP saliente) hacia una dirección IP predefinida.

## Requisitos e Instalación

### Target de Compilación
Dado que se requiere compatibilidad con arquitectura x86, asegúrate de tener instalado el *target* correspondiente a través de `rustup`:

```bash
# Para entornos Linux (ejemplo)
rustup target add i686-unknown-linux-gnu

# Para entornos Windows (ejemplo)
rustup target add i686-pc-windows-msvc
