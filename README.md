# FileFlow

**FileFlow** es un automatizador local de tareas de archivos para Windows,
construido principalmente en Rust.

La idea del proyecto es ofrecer un **Zapier local para archivos**: copiar,
mover, sincronizar, vigilar carpetas y ejecutar automatizaciones reutilizables
sin depender de servicios externos.

## Estado actual

- CLI en Rust para ejecutar acciones desde terminal.
- GUI de escritorio con Tauri, React y TypeScript.
- Motor central reutilizable con logs, progreso, estados y cancelacion.
- Acciones modulares registradas mediante factories.
- Pipelines JSON para encadenar acciones.
- Editor visual de pipelines.
- Biblioteca de pipelines recientes/guardados.
- Persistencia local de rutas, historial y preferencias de la GUI.
- Sincronizacion recursiva con `--dry-run`, `--overwrite` y `--delete-extra`.
- Watcher de carpetas mediante `notify`, disponible desde CLI y GUI.
- Builds de Windows publicados directamente en `release/`.

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
│   ├── fileflow-core        # Motor, Action trait, Context, logs y progreso
│   ├── fileflow-actions     # Acciones, factories, registry y pipelines JSON
│   ├── fileflow-cli         # CLI basada en clap
│   └── fileflow-gui         # GUI Tauri + React
├── pipelines/               # Ejemplos de automatizaciones JSON
└── release/                 # Ejecutables e instaladores publicados
```

```text
CLI / GUI -> Registry -> ActionFactory -> Action -> Engine -> Context -> Logs / Progress / Result
```

## Desarrollo

Requisitos:

- Rust: <https://rustup.rs>
- Node.js y npm
- Dependencias de Tauri 2 para Windows

Compilar workspace Rust:

```bash
cargo build --release
```

Ejecutar tests:

```bash
cargo test
```

Ejecutar GUI en desarrollo:

```bash
cd crates/fileflow-gui
npm install
npm run tauri dev
```

Compilar frontend:

```bash
cd crates/fileflow-gui
npm run build
```

## Uso CLI

Listar acciones:

```bash
cargo run -p fileflow-cli -- actions list
```

Copiar archivo:

```bash
cargo run -p fileflow-cli -- run copy -- --src a.txt --dst b.txt --overwrite
```

Mover archivo:

```bash
cargo run -p fileflow-cli -- run move -- --src a.txt --dst b.txt --overwrite
```

Sincronizar carpetas:

```bash
cargo run -p fileflow-cli -- run sync -- --src ./origen --dst ./destino --recursive
```

Previsualizar sincronizacion sin escribir cambios:

```bash
cargo run -p fileflow-cli -- run sync -- --src ./origen --dst ./destino --recursive --delete-extra --dry-run
```

Ejecutar pipeline:

```bash
cargo run -p fileflow-cli -- run-config ./pipelines/demo.json
```

Validar pipeline:

```bash
cargo run -p fileflow-cli -- validate-config ./pipelines/demo.json
```

Vigilar carpeta:

```bash
cargo run -p fileflow-cli -- run watch -- --path ./entrada --config ./pipelines/sync.json --recursive --debounce-ms 500
```

## GUI

La interfaz incluye:

- Acciones rapidas para `echo`, `copy`, `move` y `sync`.
- Selector de archivos y carpetas.
- Previsualizacion de sincronizaciones con `dry-run`.
- Editor visual de pipelines JSON.
- Biblioteca de pipelines recientes y guardados.
- Pantalla para vigilar carpetas y ejecutar pipelines al detectar cambios.
- Historial y rutas persistentes entre sesiones.
- Panel de actividad, logs, progreso flotante y cancelacion.
- Guia integrada y pantalla de proyecto.

## Release

Los artefactos publicados se dejan directamente en `release/`, sin subcarpetas
por version:

```text
release/
├── fileflow.exe
├── fileflow-cli.exe
├── fileflow_0.5.0_x64-setup.exe
└── fileflow_0.5.0_x64_en-US.msi
```

El ejecutable principal de la GUI se llama `fileflow.exe`. La CLI mantiene el
nombre `fileflow-cli.exe` para poder convivir en la misma carpeta.

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

Comprueba que el archivo o carpeta indicado en `--src` exista y que la ruta sea
valida desde el directorio en el que ejecutas el comando.

### Destino ya existente

Para `copy`, `move` o `sync`, usa `--overwrite` cuando quieras permitir
sobrescritura.

## Roadmap

- Mejorar la biblioteca de pipelines con busqueda y etiquetas.
- Ampliar cobertura de tests de integracion.
- Mejorar empaquetado y distribucion de releases.
- Plugins dinamicos o integraciones externas.
- Optimizaciones para trabajos grandes y ejecucion paralela.

## Autor

Lucas Ruiz

Proyecto personal en Rust para automatizacion local de archivos.
