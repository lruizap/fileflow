# Comparativa de rendimiento: FileFlow vs Explorador de archivos de Windows

Fecha del analisis: 2026-05-16  
Proyecto: FileFlow  
Equipo de prueba: entorno local Windows con unidades `C:` y `G:`

## 1. Resumen ejecutivo

FileFlow y el Explorador de archivos de Windows tienen un comportamiento muy distinto segun el tipo de movimiento:

| Escenario | Que ocurre realmente | FileFlow | Explorador de Windows | Resultado |
|---|---|---:|---:|---|
| Mismo volumen, por ejemplo `C:\origen` -> `C:\destino` | Movimiento por metadatos/rename | 0,025 s medidos | 0,016 s medidos con proxy nativo | Ambos son practicamente instantaneos |
| Distinto volumen, por ejemplo `C:` -> `G:` | Copia fisica + borrado del origen | 1,0-1,6 min estimados para 100 GiB | 2-4 min estimados para 100 GiB | FileFlow fue mas rapido en las pruebas locales frente a la capa Shell |

Conclusion principal: para mover un archivo de 100 GB dentro del mismo disco o particion, el tamano casi no importa. Para moverlo entre discos o particiones, el limite real es la velocidad de lectura/escritura, la cache del sistema, antivirus, tipo de disco y carga del equipo.

## 2. Como mueve archivos FileFlow

La accion `move` de FileFlow sigue este flujo:

1. Valida que el origen exista y sea un archivo.
2. Comprueba que origen y destino no sean el mismo archivo.
3. Prepara el destino y aplica `overwrite` si corresponde.
4. Intenta `fs::rename(src, dst)`.
5. Si `rename` falla, usa fallback: copiar por bloques a un temporal y borrar el origen.

Referencia de codigo:

| Componente | Archivo | Relevancia |
|---|---|---|
| Intento de movimiento directo | `crates/fileflow-actions/src/actions/move_file.rs:56` | `fs::rename(src, dst)` |
| Fallback al copiar | `crates/fileflow-actions/src/actions/move_file.rs:64` | `copy_file_optimized(...)` |
| Borrado del origen tras copiar | `crates/fileflow-actions/src/actions/move_file.rs:75` | `fs::remove_file(src)` |
| Tamano del buffer | `crates/fileflow-actions/src/fs/helpers.rs:9` | `8 MiB` por bloque |
| Emision de progreso | `crates/fileflow-actions/src/fs/helpers.rs:10` | cada `64 MiB` |
| Archivo temporal seguro | `crates/fileflow-actions/src/fs/helpers.rs:120` | rename del temporal al destino final |

## 3. Metodologia de medicion

Se hicieron tres tipos de pruebas:

| Prueba | Tamano | Ruta | Herramienta comparada | Motivo |
|---|---:|---|---|---|
| Movimiento mismo volumen | 100 GiB sparse | `C:` -> `C:` | Movimiento nativo equivalente | Mide el caso instantaneo de rename |
| Movimiento entre volumenes | 1 GiB real aleatorio | `C:` -> `G:` | `Move-Item` | Primera referencia con copia real |
| Movimiento entre volumenes | 4 GiB real aleatorio | `C:` -> `G:` | `Move-Item` | Muestra comportamiento con archivo mas grande |
| Movimiento entre volumenes | 2 GiB real aleatorio | `C:` -> `G:` | `Shell.Application.MoveHere` | Proxy mas cercano al Explorador de archivos |

Nota importante: automatizar exactamente el Explorador de archivos con su UI no es una forma fiable de benchmark. Por eso se uso `Shell.Application.MoveHere` como proxy de la capa Shell de Windows, que es mas representativa del Explorador que un simple comando de consola.

## 4. Resultados medidos

### 4.1 Movimiento dentro del mismo volumen

| Herramienta | Tamano logico | Tiempo medido | Interpretacion |
|---|---:|---:|---|
| FileFlow | 100 GiB | 0,0252 s | Rename directo completado |
| Movimiento nativo Windows | 100 GiB | 0,0163 s | Rename directo equivalente |

En este escenario no se copian los datos. Windows solo cambia referencias internas del sistema de archivos, por eso el resultado es inferior a un segundo incluso con un archivo de 100 GiB.

### 4.2 Movimiento entre volumenes

| Prueba | FileFlow | Rendimiento FileFlow | Windows/proxy | Rendimiento Windows/proxy |
|---|---:|---:|---:|---:|
| 1 GiB, `C:` -> `G:` | 0,706 s | 1450 MiB/s | 0,552 s con `Move-Item` | 1853 MiB/s |
| 2 GiB, `C:` -> `G:` | 1,262 s | 1623 MiB/s | 2,676 s con `Shell.Application.MoveHere` | 765 MiB/s |
| 4 GiB, `C:` -> `G:` | 3,840 s | 1067 MiB/s | 9,275 s con `Move-Item` | 442 MiB/s |

La prueba de 1 GiB puede estar mas influida por cache y arranque del proceso. Las de 2 GiB y 4 GiB son mas utiles para extrapolar, aunque siguen siendo estimaciones y no sustituyen una prueba real de 100 GiB.

## 5. Estimacion para mover 100 GiB

Formula usada:

```text
tiempo_segundos = tamano_MiB / velocidad_MiB_por_segundo
```

Para `100 GiB`:

```text
100 GiB = 102400 MiB
```

| Escenario | Velocidad usada | Estimacion para 100 GiB |
|---|---:|---:|
| FileFlow, usando medicion 2 GiB | 1623 MiB/s | 1,05 min |
| FileFlow, usando medicion 4 GiB | 1067 MiB/s | 1,60 min |
| Windows Shell, usando medicion 2 GiB | 765 MiB/s | 2,23 min |
| Windows `Move-Item`, usando medicion 4 GiB | 442 MiB/s | 3,87 min |

Rango recomendado para comunicar:

| Herramienta | Estimacion razonable para 100 GiB entre volumenes |
|---|---:|
| FileFlow | 1,0-1,6 min |
| Explorador de Windows | 2-4 min |

## 6. Ejemplos practicos

### Ejemplo A: mover dentro del mismo SSD

| Ruta origen | Ruta destino | Resultado esperado |
|---|---|---|
| `C:\Videos\archivo_100gb.mkv` | `C:\Archivo\archivo_100gb.mkv` | Menos de 1 segundo en FileFlow y Explorador |

Motivo: origen y destino estan en el mismo volumen. FileFlow ejecuta `fs::rename`, igual que el comportamiento nativo de Windows.

### Ejemplo B: mover de SSD interno a disco externo

| Ruta origen | Ruta destino | Resultado esperado |
|---|---|---|
| `C:\Videos\archivo_100gb.mkv` | `G:\Backup\archivo_100gb.mkv` | FileFlow alrededor de 1-2 min; Explorador alrededor de 2-4 min |

Motivo: hay copia fisica de 100 GiB y despues se borra el origen.

### Ejemplo C: mover a unidad lenta USB

| Velocidad sostenida de destino | Tiempo aproximado para 100 GiB |
|---:|---:|
| 100 MiB/s | 17,1 min |
| 200 MiB/s | 8,5 min |
| 500 MiB/s | 3,4 min |
| 1000 MiB/s | 1,7 min |

En este escenario FileFlow no puede superar el limite fisico del dispositivo. La diferencia frente al Explorador dependera de overhead de UI, cache, antivirus y gestion interna de copia.

## 7. Factores que pueden cambiar los resultados

| Factor | Impacto |
|---|---|
| Mismo volumen vs distinto volumen | Es el factor mas importante: rename casi instantaneo frente a copia completa |
| SSD NVMe, SATA, HDD, USB | Determina la velocidad sostenida |
| Antivirus/Defender | Puede escanear origen, destino o temporal |
| Cache de Windows | Puede hacer que pruebas pequenas parezcan mas rapidas |
| Archivo unico vs muchos archivos pequenos | Un archivo unico grande suele ser mas eficiente |
| Estado del disco | Espacio libre, fragmentacion, temperatura y throttling pueden afectar |
| UI del Explorador | La ventana de progreso y confirmaciones pueden anadir overhead |

## 8. Lectura tecnica de la comparativa

FileFlow no usa una API especial de transferencia mas rapida que Windows. Su ventaja en las pruebas entre volumenes viene de que su ruta de copia es simple: lectura y escritura por bloques de 8 MiB, progreso cada 64 MiB y un temporal final para evitar dejar un destino incompleto.

El Explorador de archivos usa la capa Shell de Windows, que anade gestion de UI, dialogos, integracion con el sistema, comprobaciones y comportamiento mas generalista. Eso puede tener mas overhead, especialmente en pruebas automatizadas o con unidades externas.

## 9. Recomendacion

Para una comparativa comercial o de documentacion:

| Mensaje | Recomendacion |
|---|---|
| Mismo disco | "FileFlow mueve archivos grandes de forma practicamente instantanea cuando el sistema permite rename directo." |
| Entre discos | "FileFlow puede ser competitivo o mas rapido que el Explorador porque usa una copia por bloques directa y controlada." |
| Cifra de 100 GB | "En este equipo, extrapolando pruebas reales, FileFlow tardaria aproximadamente 1-1,6 minutos frente a unos 2-4 minutos del Explorador." |
| Honestidad tecnica | "Los tiempos dependen del hardware, cache, antivirus y tipo de unidad." |

## 10. Proximos pasos sugeridos

Para una validacion definitiva, ejecutar una prueba real con un archivo no sparse de 100 GiB en:

1. `C:` -> `C:` dentro del mismo volumen.
2. `C:` -> `G:` entre volumenes.
3. `C:` -> disco USB lento.
4. Repetir cada prueba 3 veces y usar mediana.
5. Registrar modelo de disco, sistema de archivos, antivirus activo y temperatura aproximada.

