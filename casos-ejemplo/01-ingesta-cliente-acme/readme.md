# Caso 01: ingesta de entregables de cliente

## Objetivo

Automatizar la entrada de documentos, imagenes y assets que un cliente deja en
una carpeta local. El pipeline mantiene una zona de trabajo sincronizada y copia
el manifiesto a una carpeta de auditoria para revision rapida.

## Estructura sugerida

```text
casos-ejemplo/01-ingesta-cliente-acme/
├── entrada/
│   └── cliente-acme/
│       ├── manifest.csv
│       ├── brief.pdf
│       └── assets/
│           ├── logo.png
│           └── campana/
│               └── banner.psd
├── trabajo/
│   └── cliente-acme/
├── auditoria/
│   └── cliente-acme/
├── pipeline.json
└── readme.md
```

## Pipeline

El archivo `pipeline.json` contiene tres pasos:

1. `echo` para comprobar que el motor arranca correctamente y deja una entrada
   clara en logs.
2. `sync` para sincronizar toda la carpeta recibida con la zona de trabajo.
3. `copy` para duplicar `manifest.csv` en auditoria sin copiar todo el paquete.

## Validar

```bash
cargo run -p fileflow-cli -- validate-config ./casos-ejemplo/01-ingesta-cliente-acme/pipeline.json
```

## Ejecutar manualmente

```bash
cargo run -p fileflow-cli -- run-config ./casos-ejemplo/01-ingesta-cliente-acme/pipeline.json
```

## Ejecutar automaticamente al detectar cambios

```bash
cargo run -p fileflow-cli -- run watch -- --path ./casos-ejemplo/01-ingesta-cliente-acme/entrada/cliente-acme --config ./casos-ejemplo/01-ingesta-cliente-acme/pipeline.json --recursive --debounce-ms 500
```

## Optimizacion aplicada

- `sync` evita crear un paso `copy` por cada archivo recibido.
- `--recursive` permite procesar subcarpetas de assets y campanas.
- `--overwrite` mantiene la zona de trabajo con la ultima version recibida.
- `--delete-extra` convierte `trabajo/cliente-acme` en un espejo del paquete de
  entrada, util cuando el cliente retira o sustituye archivos.
- `copy` se usa solo para el manifiesto, porque auditoria no necesita todo el
  contenido del paquete.

Este caso facilita un flujo de recepcion profesional: el usuario solo deja
archivos en la carpeta de entrada y FileFlow actualiza el espacio de trabajo con
logs, progreso y resultado final.
