# 📦 FileFlow

## 🚀 Automatizador de tareas de archivos para Windows (Rust Edition)

FileFlow es una aplicación de escritorio **portable** diseñada para
automatizar procesos repetitivos del sistema de archivos en Windows.

Permite ejecutar tareas como:

- Copiar / mover archivos grandes (rápido y robusto)
- Comprimir (ZIP / 7z)
- Buscar archivos instantáneamente
- Convertir imágenes y documentos
- Ejecutar pipelines automáticos
- Programar flujos
- Automatizar carpetas

Disponible como:

- ✅ GUI
- ✅ CLI
- ✅ Ejecutable portable (.exe sin instalación)

------------------------------------------------------------------------

# 🎯 Objetivos del proyecto

## Problema

Windows Explorer es lento y muy manual para tareas repetitivas.

## Solución

FileFlow automatiza y acelera:

- Copias masivas
- Backups
- Limpieza de discos
- Compresiones
- Conversiones
- Flujos por lotes

------------------------------------------------------------------------

# 🧠 Filosofía

- Portable first (sin instalación)
- Alto rendimiento
- Bajo consumo de RAM
- Automatización \> Clicks
- Arquitectura modular (plugins)
- CLI + GUI
- Siempre funcional desde el primer release

------------------------------------------------------------------------

# 🦀 Stack tecnológico

## Core

- Rust
- Tokio (async + paralelismo)
- Rayon (CPU paralelo)
- Walkdir (filesystem scanning)
- Serde (config JSON / YAML)
- Clap (CLI)

## GUI

- Tauri (Rust backend + frontend web ligera) o
- egui (GUI nativa inmediata)

## Herramientas externas

- Robocopy (Windows)
- 7zip portable
- Pandoc
- ImageMagick
- LibreOffice portable (opcional)

------------------------------------------------------------------------

# 🏗 Arquitectura

``` Markdown
fileflow/
├─ core/ → motor de tareas
├─ actions/ → plugins (copy, zip, convert...)
├─ cli/ → comandos terminal
├─ gui/ → interfaz Tauri/egui
├─ tools/ → binarios portables (7zip, pandoc...)
├─ configs/ └─ releases/
```

------------------------------------------------------------------------

# ⚙ Características planeadas

## MVP

- Cola de tareas
- Copiar / mover robusto
- Compresión 7zip
- Logs
- UI mínima

## v0.2

- Buscador rápido
- Presets

## v0.3

- Pipelines
- Watchers de carpetas
- Renombrado masivo
- Deduplicado

## v1.0

- Conversión imágenes / documentos
- OCR
- CLI completa
- Menú contextual Windows

------------------------------------------------------------------------

# 🚀 Ejemplos de uso CLI

``` bash
fileflow copy origen destino
fileflow compress carpeta/
fileflow convert *.md --pdf
fileflow pipeline run backup-nas
```

------------------------------------------------------------------------

# 📌 Estado del proyecto

Actualmente en desarrollo activo. Arquitectura modular orientada a
crecimiento progresivo por versiones.
