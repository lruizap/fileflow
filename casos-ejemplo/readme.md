# Casos de ejemplo de FileFlow

Esta carpeta reune casos de uso practicos para entender como FileFlow puede
automatizar trabajo real con archivos locales. Cada caso esta separado en su
propia carpeta y contiene:

- `readme.md`: explicacion del problema, estructura recomendada, comandos y
  decisiones de optimizacion.
- `pipeline.json`: automatizacion lista para validar, ejecutar o cargar desde
  la GUI.

Los ejemplos estan pensados para ejecutarse desde la raiz del repositorio. Las
rutas del JSON apuntan a carpetas dentro de cada caso para que puedas probarlos
sin mezclar datos con otros proyectos.

## Casos incluidos

| Caso | Objetivo | Funcionalidades usadas |
| --- | --- | --- |
| `01-ingesta-cliente-acme` | Procesar entregables recibidos de un cliente y mantener una zona de trabajo actualizada. | `echo`, `sync`, `copy`, `watch` |
| `02-backup-proyecto-local` | Crear un backup local reproducible de una carpeta de proyecto. | `echo`, `sync`, `dry-run`, `overwrite` |
| `03-publicacion-build-local` | Preparar una carpeta de publicacion local a partir de una build generada. | `echo`, `sync`, `copy`, `delete-extra` |

## Como usar un caso

1. Entra en la carpeta del caso y lee su `readme.md`.
2. Crea las carpetas y archivos de entrada indicados.
3. Valida el pipeline:

```bash
cargo run -p fileflow-cli -- validate-config ./casos-ejemplo/<caso>/pipeline.json
```

4. Ejecuta la automatizacion:

```bash
cargo run -p fileflow-cli -- run-config ./casos-ejemplo/<caso>/pipeline.json
```

5. Si el caso incluye vigilancia de carpeta, puedes dejar FileFlow escuchando
   cambios con `watch`:

```bash
cargo run -p fileflow-cli -- run watch -- --path ./ruta/a/vigilar --config ./casos-ejemplo/<caso>/pipeline.json --recursive --debounce-ms 500
```

## Criterio de optimizacion usado

Los JSON evitan pasos innecesarios y usan la accion mas adecuada para cada
tarea:

- `sync` para carpetas completas, porque copia solo lo necesario y conserva la
  estructura.
- `copy` para archivos concretos que deben quedar en otra ubicacion.
- `--recursive` cuando hay subcarpetas.
- `--overwrite` cuando el destino debe reflejar siempre la ultima version.
- `--delete-extra` solo en destinos que deben comportarse como espejo exacto.
- `--dry-run` para revisar impacto antes de escribir cambios.

Esto ayuda a que los usuarios entiendan no solo que comandos ejecutar, sino por
que cada pipeline esta construido de esa forma.
