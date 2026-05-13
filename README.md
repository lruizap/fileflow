# FileFlow

**FileFlow** es un automatizador local de tareas de archivos para Windows, construido principalmente en Rust.

La idea del proyecto es sencilla: ofrecer un **"Zapier local para archivos"** que permita copiar, mover, sincronizar, vigilar carpetas y ejecutar automatizaciones reutilizables sin depender de servicios externos.

El repositorio incluye una CLI funcional, una GUI de escritorio con Tauri/React y un motor modular compartido por ambas interfaces.

## Estado actual

- CLI en Rust para ejecutar acciones desde terminal.
- GUI de escritorio en Tauri + React.
- Motor central reutilizable con logs, progreso, estados y cancelacion.
- Acciones modulares registradas mediante factories.
- Pipelines JSON para encadenar varias acciones.
- Sincronizacion de carpetas con modo recursivo.
- Watcher de carpetas mediante `notify`.
- Build portable para Windows en `release/v0.2.5`.

## Acciones disponibles

| Accion | Descripcion |
| --- | --- |
| `echo` | Accion de prueba para validar el flujo completo. |
| `copy` | Copia un archivo de origen a destino. |
| `move` | Mueve un archivo de origen a destino. |
| `sync` | Sincroniza carpetas, con soporte recursivo opcional. |
| `watch` | Vigila una carpeta y ejecuta un pipeline cuando detecta cambios. |
| `pipeline` | Ejecuta varias acciones en secuencia. |

## Arquitectura

```text
fileflow/
├── crates/
│   ├── fileflow-core        # Motor, Action trait, Context, logs, progreso y estados
│   ├── fileflow-actions     # Acciones, factories, registry y pipelines JSON
│   ├── fileflow-cli         # CLI basada en clap
│   └── fileflow-gui         # GUI Tauri + React
├── pipelines/               # Ejemplos de automatizaciones JSON
├── release/                 # Builds publicados
└── tests/                   # Tests y pruebas de comportamiento
```

Flujo interno simplificado:

```text
CLI / GUI -> Registry -> ActionFactory -> Action -> Engine -> Context -> Logs / Progress / Result
```

La GUI no reimplementa la logica de archivos: invoca comandos Tauri que usan el mismo motor Rust que la CLI.

## Requisitos

Para desarrollo:

- Rust: <https://rustup.rs>
- Node.js y npm
- Dependencias de Tauri 2 para Windows

Para uso final en Windows, la idea es distribuir ejecutables portables desde `release/`.

## Compilar el workspace Rust

```bash
cargo build --release
```

El ejecutable CLI queda en:

```text
target/release/fileflow-cli.exe
```

## Usar la CLI

Listar acciones:

```bash
cargo run -p fileflow-cli -- actions list
```

Ejecutar `echo`:

```bash
cargo run -p fileflow-cli -- run echo
```

Copiar archivo:

```bash
cargo run -p fileflow-cli -- run copy -- --src a.txt --dst b.txt
```

Copiar sobrescribiendo si el destino existe:

```bash
cargo run -p fileflow-cli -- run copy -- --src a.txt --dst b.txt --overwrite
```

Mover archivo:

```bash
cargo run -p fileflow-cli -- run move -- --src a.txt --dst b.txt
```

Sincronizar carpetas:

```bash
cargo run -p fileflow-cli -- run sync -- --src ./origen --dst ./destino
```

Sincronizar de forma recursiva:

```bash
cargo run -p fileflow-cli -- run sync -- --src ./origen --dst ./destino --recursive
```

Sincronizar eliminando archivos extra del destino:

```bash
cargo run -p fileflow-cli -- run sync -- --src ./origen --dst ./destino --delete-extra
```

## Pipelines JSON

Los pipelines permiten guardar automatizaciones reutilizables en archivos JSON.

Ejemplo:

```json
{
  "name": "demo",
  "steps": [
    {
      "action": "echo",
      "args": []
    },
    {
      "action": "move",
      "args": ["--src", "./a.txt", "--dst", "./b.txt", "--overwrite"]
    }
  ]
}
```

Ejecutar un pipeline:

```bash
cargo run -p fileflow-cli -- run-config ./pipelines/demo.json
```

Validar un pipeline sin ejecutarlo:

```bash
cargo run -p fileflow-cli -- validate-config ./pipelines/demo.json
```

## Watcher de carpetas

La accion `watch` observa una carpeta y lanza un pipeline cuando detecta cambios.

Ejemplo conceptual:

```bash
cargo run -p fileflow-cli -- run watch -- --path ./entrada --config ./pipelines/sync.json
```

Opciones soportadas por la accion:

- `--path`: carpeta a vigilar.
- `--config`: pipeline JSON que se ejecutara.
- `--recursive`: vigila subcarpetas.
- `--once`: ejecuta una vez y termina.
- `--debounce-ms`: evita ejecuciones repetidas demasiado seguidas.

## GUI de escritorio

La GUI esta en `crates/fileflow-gui` y usa:

- React 19
- TypeScript
- Vite
- Tauri 2

Instalar dependencias frontend:

```bash
cd crates/fileflow-gui
npm install
```

Ejecutar frontend en desarrollo:

```bash
npm run dev
```

Compilar frontend:

```bash
npm run build
```

Ejecutar con Tauri:

```bash
npm run tauri dev
```

La interfaz incluye:

- Acciones rapidas para `echo`, `copy`, `move` y `sync`.
- Selector de archivos y carpetas.
- Ejecucion y validacion de pipelines JSON.
- Panel de actividad e historial.
- Progreso flotante.
- Cancelacion del trabajo actual.
- Guia integrada y pantalla "Acerca de".

## Ejecutables publicados

El repositorio contiene una build de Windows en:

```text
release/v0.2.5/
├── fileflow-cli.exe
└── fileflow-gui.exe
```

## Tests y verificacion

Ejecutar comprobaciones Rust:

```bash
cargo test
```

Compilar la GUI:

```bash
cd crates/fileflow-gui
npm run build
```

Nota: el workspace compila correctamente con `cargo test`. Algunos tests de integracion estan en la carpeta raiz `tests/`; si se quiere que Cargo los ejecute automaticamente, conviene moverlos a la crate correspondiente o crear una crate dedicada de tests.

## Errores comunes

### Falta de `--` al pasar argumentos a una accion

Incorrecto:

```bash
fileflow run copy --src a.txt --dst b.txt
```

Correcto:

```bash
fileflow run copy -- --src a.txt --dst b.txt
```

### Ruta de origen inexistente

Comprueba que el archivo o carpeta indicado en `--src` exista y que la ruta sea valida desde el directorio en el que ejecutas el comando.

### Destino ya existente

Para `copy`, `move` o `sync`, usa `--overwrite` cuando quieras permitir sobrescritura.

## Roadmap

- Mejorar la cobertura y ubicacion de tests de integracion.
- Ampliar el builder visual de pipelines.
- Mejorar empaquetado y distribucion de releases.
- Persistencia de historial/configuracion de la GUI.
- Plugins dinamicos o integraciones externas.
- Optimizaciones para trabajos grandes y ejecucion paralela.

## Autor

Lucas Ruiz

Proyecto personal en Rust para automatizacion local de archivos.
