# 🚀 FileFlow

**FileFlow** es un automatizador local de tareas de archivos para Windows, construido en Rust.

Permite ejecutar acciones como copiar, mover, sincronizar archivos y automatizar procesos mediante pipelines reutilizables.

> 🧠 Filosofía: **“Zapier local para archivos”**

---

# ✨ Características

- ⚡ Alto rendimiento (Rust)
- 📦 Portable (ejecutable `.exe`, sin instalación)
- 🧩 Arquitectura modular (actions/plugins)
- 🖥️ CLI funcional (GUI futura)
- 🔗 Soporte de pipelines
- 📄 Configuración mediante JSON
- ✅ Validación de configuraciones (`validate-config`)
- 🧪 Tests incluidos

---

# 📦 Acciones disponibles (v0.1.0)

| Acción   | Descripción |
|---------|------------|
| `echo`  | Acción de prueba |
| `copy`  | Copia archivos |
| `move`  | Mueve archivos |
| `sync`  | Sincroniza carpetas (nivel superior) |
| `pipeline` | Ejecuta múltiples acciones en secuencia |

---

# 🚀 Instalación

## Requisitos

- Rust → <https://rustup.rs>

## Clonar repositorio

```bash
git clone https://github.com/lruizap/fileflow.git
cd fileflow
````

## Compilar

```bash
cargo build --release
```

El ejecutable estará en:

```bash
target/release/fileflow-cli.exe
```

---

# 🧪 Uso básico

## Listar acciones disponibles

```bash
cargo run -p fileflow-cli -- actions list
```

---

## Ejecutar una acción

```bash
cargo run -p fileflow-cli -- run echo
```

---

## Copiar archivo

```bash
cargo run -p fileflow-cli -- run copy -- --src a.txt --dst b.txt
```

---

## Mover archivo

```bash
cargo run -p fileflow-cli -- run move -- --src a.txt --dst b.txt
```

---

## Sincronizar carpetas

```bash
cargo run -p fileflow-cli -- run sync -- --src ./origen --dst ./destino
```

### Con eliminación de archivos extra

```bash
cargo run -p fileflow-cli -- run sync -- --src ./origen --dst ./destino --delete-extra
```

---

# 🔗 Pipelines

## Ejecutar pipeline desde CLI

```bash
cargo run -p fileflow-cli -- run pipeline -- --step echo --step echo
```

---

## Pipeline con argumentos

```bash
cargo run -p fileflow-cli -- run pipeline -- --step move --step-args "move:--src=./a.txt,--dst=./b.txt"
```

---

# 📄 Pipelines con JSON

## Ejecutar desde archivo

```bash
cargo run -p fileflow-cli -- run-config ./pipelines/demo.json
```

---

## Validar JSON sin ejecutar

```bash
cargo run -p fileflow-cli -- validate-config ./pipelines/demo.json
```

---

## Ejemplo de JSON

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

---

# 🧪 Ejemplo real

### Sincronizar carpeta automáticamente

```json
{
  "name": "sync_docs",
  "steps": [
    {
      "action": "sync",
      "args": ["--src", "./docs", "--dst", "./backup", "--delete-extra"]
    }
  ]
}
```

Ejecución:

```bash
cargo run -p fileflow-cli -- run-config ./pipelines/sync.json
```

---

# ⚠️ Errores comunes

## ❌ Falta de `--`

Incorrecto:

```bash
fileflow run copy --src a.txt --dst b.txt
```

Correcto:

```bash
fileflow run copy -- --src a.txt --dst b.txt
```

---

## ❌ Archivo o ruta no existe

Verifica:

```bash
--src ./archivo.txt
```

---

# 🧪 Tests

```bash
cargo test
```

---

# 🧱 Arquitectura

```
fileflow/
├── crates/
│   ├── fileflow-core    # Motor de ejecución
│   ├── fileflow-actions # Acciones y factories
│   └── fileflow-cli     # CLI
```

## Flujo interno

```
CLI → Registry → Action → Engine → Job → Logs → Resultado
```

---

# ⚠️ Limitaciones actuales (v0.1.0)

- `sync` solo funciona en nivel superior (no recursivo)
- No hay GUI aún
- No hay watchers automáticos
- No hay ejecución async

---

# 🔮 Roadmap

- Sync recursivo (walkdir)
- Watchers de carpetas
- GUI multiplataforma
- Plugins dinámicos
- Paralelismo (tokio / rayon)
- Integración con herramientas externas (7zip, robocopy)

---

# 👨‍💻 Autor

Lucas Ruiz
Proyecto personal en Rust
