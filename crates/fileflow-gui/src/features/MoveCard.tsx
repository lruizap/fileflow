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
      description="Mueve un archivo a otra ubicación."
      badge="move"
    >
      <PathInput
        label="Archivo origen"
        value={src}
        onChange={setSrc}
        buttonText="Elegir archivo"
        onPick={selectSrc}
      />

      <PathInput
        label="Archivo destino"
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
        Sobrescribir destino
      </label>

      <button
        className="primary-btn"
        disabled={loading || !src || !dst}
        onClick={() =>
          runCommand("run_move", {
            src,
            dst,
            overwrite,
          })
        }
      >
        Ejecutar Move
      </button>
    </ActionCard>
  );
}
