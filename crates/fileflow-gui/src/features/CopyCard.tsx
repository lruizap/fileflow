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

export function CopyCard({
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
      title="Copiar archivo"
      description="Duplica un archivo en otra ubicación sin borrar el original."
      badge="copiar"
    >
      <PathInput
        label="Archivo que quieres copiar"
        value={src}
        onChange={setSrc}
        buttonText="Elegir archivo"
        onPick={selectSrc}
      />

      <PathInput
        label="Dónde guardar la copia"
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
            "run_copy",
            {
              src,
              dst,
              overwrite,
            },
            "Copiar archivo",
          )
        }
      >
        Copiar archivo
      </button>
    </ActionCard>
  );
}
