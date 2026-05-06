import type { PipelineActionType } from "../../types";

type Props = {
  onAddStep: (action: PipelineActionType) => void;
};

export function PipelineToolbar({ onAddStep }: Props) {
  return (
    <div className="pipeline-toolbar">
      <button
        className="secondary-btn"
        type="button"
        onClick={() => onAddStep("copy")}
      >
        Añadir copiar
      </button>

      <button
        className="secondary-btn"
        type="button"
        onClick={() => onAddStep("move")}
      >
        Añadir mover
      </button>

      <button
        className="secondary-btn"
        type="button"
        onClick={() => onAddStep("sync")}
      >
        Añadir sync
      </button>

      <button
        className="secondary-btn"
        type="button"
        onClick={() => onAddStep("echo")}
      >
        Añadir prueba
      </button>
    </div>
  );
}
