import { ActionCard } from "../components/ActionCard";
import { PathInput } from "../components/PathInput";
import { pickDestinationFile, pickFile } from "../api/fileflow";
import type { RunCommand } from "../types";

type Props = {
  loading: boolean;
  src: string;
  dst: string;
  overwrite: boolean;
  setSrc: (value: string) => void;
  setDst: (value: string) => void;
  setOverwrite: (value: boolean) => void;
  runCommand: RunCommand;
};

export function MoveCard({
  loading,
  src,
  dst,
  overwrite,
  setSrc,
  setDst,
  setOverwrite,
  runCommand,
}: Props) {
  async function selectSrc() {
    const selected = await pickFile();
    if (selected) setSrc(selected);
  }

  async function selectDst() {
    const selected = await pickDestinationFile();
    if (selected) setDst(selected);
  }

  return (
    <ActionCard
      title="Mover archivo"
      description="Traslada un archivo a otra ubicación y elimina el original."
      badge="mover"
    >
      <PathInput
        label="Archivo que quieres mover"
        value={src}
        onChange={setSrc}
        buttonText="Elegir archivo"
        onPick={selectSrc}
      />

      <PathInput
        label="Nueva ubicación del archivo"
        value={dst}
        onChange={setDst}
        buttonText="Guardar como"
        onPick={selectDst}
      />

      <label className="check solo">
        <input
          type="checkbox"
          checked={overwrite}
          onChange={(e) => setOverwrite(e.target.checked)}
        />
        Sobrescribir si ya existe
      </label>

      <button
        className="primary-btn"
        disabled={loading || !src || !dst}
        onClick={() =>
          runCommand(
            "run_move",
            {
              src,
              dst,
              overwrite,
            },
            "Mover archivo",
          )
        }
      >
        Mover archivo
      </button>
    </ActionCard>
  );
}
