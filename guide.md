# 📘 Guía básica de FileFlow

Esta guía explica cómo usar FileFlow de forma sencilla.

---

# 🧠 Conceptos básicos

FileFlow funciona con:

- **Actions** → tareas individuales (copy, move, sync…)
- **Pipelines** → varias acciones encadenadas
- **Config JSON** → pipelines reutilizables

---

# 🚀 Comandos principales

## Ver acciones disponibles

```bash
fileflow actions list
````

---

## Ejecutar una acción

```bash
fileflow run <acción> -- <argumentos>
```

Ejemplo:

```bash
fileflow run copy -- --src a.txt --dst b.txt
```

---

# 📦 Acciones

## copy

Copia un archivo

```bash
fileflow run copy -- --src a.txt --dst b.txt
```

---

## move

Mueve un archivo

```bash
fileflow run move -- --src a.txt --dst b.txt
```

---

## sync

Sincroniza carpetas

```bash
fileflow run sync -- --src ./origen --dst ./destino
```

### Borrar archivos extra

```bash
fileflow run sync -- --src ./origen --dst ./destino --delete-extra
```

---

# 🔗 Pipelines

## Desde CLI

```bash
fileflow run pipeline -- --step echo --step echo
```

---

## Con argumentos por step

```bash
fileflow run pipeline -- --step move --step-args "move:--src=./a.txt,--dst=./b.txt"
```

---

# 📄 Pipelines con JSON

## Crear archivo

```json
{
  "name": "mi_pipeline",
  "steps": [
    {
      "action": "copy",
      "args": ["--src", "./a.txt", "--dst", "./b.txt"]
    }
  ]
}
```

---

## Ejecutarlo

```bash
fileflow run-config ./pipeline.json
```

---

# 🧪 Ejemplo real

Mover archivo automáticamente:

```json
{
  "name": "move_file",
  "steps": [
    {
      "action": "move",
      "args": ["--src", "./tmp/a.txt", "--dst", "./tmp/b.txt"]
    }
  ]
}
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

## ❌ Archivo no existe

Verifica rutas:

```bash
--src ./archivo.txt
```

---

# 🧭 Recomendaciones

- Usa rutas relativas (`./`)
- Empieza con `echo` para probar pipelines
- Usa JSON para automatizaciones reales

---

# 🚀 Próximos pasos

- Crear pipelines propios
- Automatizar tareas repetitivas
- Integrar en scripts o tareas programadas

---

# 💡 Consejo

Piensa en FileFlow como:

👉 “Automatización de archivos sin depender de la nube”
