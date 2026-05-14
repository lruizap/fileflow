import type { PipelineActionType, PipelineStepDraft } from "../../types";
import { ACTION_LABELS } from "./pipelineMapper";
import { PathSelector } from "./PathSelector";

type Props = {
  step: PipelineStepDraft;
  index: number;
  onUpdate: (id: string, patch: Partial<PipelineStepDraft>) => void;
  onRemove: (id: string) => void;
  onMove: (id: string, direction: "up" | "down") => void;
};

export function PipelineStepCard({
  step,
  index,
  onUpdate,
  onRemove,
  onMove,
}: Props) {
  return (
    <div className="pipeline-step">
      <div className="pipeline-step-header">
        <strong>
          Paso {index + 1}: {ACTION_LABELS[step.action]}
        </strong>

        <div className="step-buttons">
          <button type="button" onClick={() => onMove(step.id, "up")}>
            ↑
          </button>
          <button type="button" onClick={() => onMove(step.id, "down")}>
            ↓
          </button>
          <button type="button" onClick={() => onRemove(step.id)}>
            Eliminar
          </button>
        </div>
      </div>

      <label>
        Tipo de acción
        <select
          value={step.action}
          onChange={(e) =>
            onUpdate(step.id, {
              action: e.target.value as PipelineActionType,
            })
          }
        >
          <option value="echo">Comprobar funcionamiento</option>
          <option value="copy">Copiar archivo</option>
          <option value="move">Mover archivo</option>
          <option value="sync">Sincronizar carpetas</option>
        </select>
      </label>

      {step.action !== "echo" && (
        <>
          <PathSelector
            label={step.action === "sync" ? "Carpeta origen" : "Archivo origen"}
            value={step.src}
            onChange={(value) => onUpdate(step.id, { src: value })}
            mode={step.action === "sync" ? "directory" : "file"}
          />

          <PathSelector
            label={
              step.action === "sync" ? "Carpeta destino" : "Archivo destino"
            }
            value={step.dst}
            onChange={(value) => onUpdate(step.id, { dst: value })}
            mode={step.action === "sync" ? "directory" : "file"}
          />

          <div className="options">
            {step.action === "sync" && (
              <>
                <label className="check">
                  <input
                    type="checkbox"
                    checked={step.recursive}
                    onChange={(e) =>
                      onUpdate(step.id, { recursive: e.target.checked })
                    }
                  />
                  Incluir subcarpetas
                </label>

                <label className="check">
                  <input
                    type="checkbox"
                    checked={step.deleteExtra}
                    onChange={(e) =>
                      onUpdate(step.id, { deleteExtra: e.target.checked })
                    }
                  />
                  Borrar extras
                </label>

                <label className="check">
                  <input
                    type="checkbox"
                    checked={step.dryRun}
                    onChange={(e) =>
                      onUpdate(step.id, { dryRun: e.target.checked })
                    }
                  />
                  Previsualizar
                </label>
              </>
            )}

            <label className="check">
              <input
                type="checkbox"
                checked={step.overwrite}
                onChange={(e) =>
                  onUpdate(step.id, { overwrite: e.target.checked })
                }
              />
              Sobrescribir
            </label>
          </div>
        </>
      )}
    </div>
  );
}
