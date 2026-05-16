# Caso 03: publicacion local de una build

## Objetivo

Preparar una carpeta limpia de publicacion a partir de una build local. Es util
para equipos que generan artefactos en `build/` o `dist/` y necesitan dejar una
carpeta final lista para empaquetar, revisar o compartir.

## Estructura sugerida

```text
casos-ejemplo/03-publicacion-build-local/
├── build/
│   ├── app.exe
│   ├── assets/
│   └── release-notes.md
├── publicacion/
│   └── app/
├── auditoria/
│   └── release-notes.md
├── pipeline.json
└── readme.md
```

## Pipeline

El pipeline sincroniza la build completa con una carpeta de publicacion y copia
las notas de version a una zona de auditoria.

## Validar

```bash
cargo run -p fileflow-cli -- validate-config ./casos-ejemplo/03-publicacion-build-local/pipeline.json
```

## Ejecutar publicacion

```bash
cargo run -p fileflow-cli -- run-config ./casos-ejemplo/03-publicacion-build-local/pipeline.json
```

## Ejecutar automaticamente cuando cambie la build

```bash
cargo run -p fileflow-cli -- run watch -- --path ./casos-ejemplo/03-publicacion-build-local/build --config ./casos-ejemplo/03-publicacion-build-local/pipeline.json --recursive --debounce-ms 750
```

## Optimizacion aplicada

- `sync` publica la carpeta completa manteniendo subcarpetas y assets.
- `--delete-extra` limpia archivos antiguos del destino, importante en carpetas
  de publicacion donde no deben quedar artefactos obsoletos.
- `--overwrite` sustituye binarios o recursos modificados.
- `copy` extrae `release-notes.md` a auditoria sin duplicar toda la build.
- El watcher usa `--debounce-ms 750` para agrupar cambios rapidos generados por
  herramientas de build.

Este caso facilita la preparacion de entregables: el usuario no tiene que borrar
manualmente restos de builds anteriores ni copiar notas de version por separado.
