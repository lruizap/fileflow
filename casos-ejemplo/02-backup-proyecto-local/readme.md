# Caso 02: backup local de proyecto

## Objetivo

Crear un backup local reproducible de una carpeta de proyecto. Este caso esta
pensado para usuarios que quieren proteger documentos de trabajo, entregables o
recursos antes de hacer cambios importantes.

## Estructura sugerida

```text
casos-ejemplo/02-backup-proyecto-local/
├── proyecto/
│   ├── documentos/
│   ├── recursos/
│   └── entregables/
├── backup/
│   └── proyecto/
├── pipeline.json
└── readme.md
```

## Pipeline

El pipeline sincroniza `proyecto/` hacia `backup/proyecto/` de forma recursiva y
sobrescribe archivos que hayan cambiado.

## Validar

```bash
cargo run -p fileflow-cli -- validate-config ./casos-ejemplo/02-backup-proyecto-local/pipeline.json
```

## Ejecutar backup

```bash
cargo run -p fileflow-cli -- run-config ./casos-ejemplo/02-backup-proyecto-local/pipeline.json
```

## Revisar impacto antes de copiar

Para previsualizar el backup sin escribir cambios, ejecuta la accion `sync` con
`--dry-run`:

```bash
cargo run -p fileflow-cli -- run sync -- --src ./casos-ejemplo/02-backup-proyecto-local/proyecto --dst ./casos-ejemplo/02-backup-proyecto-local/backup/proyecto --recursive --overwrite --dry-run
```

## Optimizacion aplicada

- `sync` se usa porque compara origen y destino y evita copiar de nuevo lo que
  no ha cambiado.
- `--recursive` cubre documentos, recursos y entregables sin pasos extra.
- `--overwrite` asegura que el backup guarde la version vigente del proyecto.
- No se usa `--delete-extra` para conservar archivos historicos que puedan
  seguir siendo utiles en el backup.

Este caso ayuda a reducir riesgo operativo: antes de tocar un proyecto, el
usuario puede actualizar un backup local con un unico comando y revisar el
progreso desde CLI o GUI.
