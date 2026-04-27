import { ActionCard } from "../components/ActionCard";
import { PathInput } from "../components/PathInput";
import { pickDirectory } from "../api/fileflow";
import type { RunCommand } from "../types";

type Props = {
  loading: boolean;
  src: string;
  dst: string;
  recursive: boolean;
  deleteExtra: boolean;
  overwrite: boolean;
  setSrc: (value: string) => void;
  setDst: (value: string) => void;
  setRecursive: (value: boolean) => void;
  setDeleteExtra: (value: boolean) => void;
  setOverwrite: (value: boolean) => void;
  runCommand: RunCommand;
};

export function SyncCard({
  loading,
  src,
  dst,
  recursive,
  deleteExtra,
  overwrite,
  setSrc,
  setDst,
  setRecursive,
  setDeleteExtra,
  setOverwrite,
  runCommand,
}: Props) {
  async function selectSrc() {
    const selected = await pickDirectory();
    if (selected) setSrc(selected);
  }

  async function selectDst() {
    const selected = await pickDirectory();
    if (selected) setDst(selected);
  }

  return (
    <ActionCard
      title="Sincronizar carpetas"
      description="Copia archivos desde origen a destino usando sync."
      badge="sync"
    >
      <PathInput
        label="Carpeta origen"
        value={src}
        onChange={setSrc}
        buttonText="Elegir carpeta"
        onPick={selectSrc}
      />

      <PathInput
        label="Carpeta destino"
        value={dst}
        onChange={setDst}
        buttonText="Elegir carpeta"
        onPick={selectDst}
      />

      <div className="options">
        <label className="check">
          <input
            type="checkbox"
            checked={recursive}
            onChange={(e) => setRecursive(e.target.checked)}
          />
          Recursivo
        </label>

        <label className="check">
          <input
            type="checkbox"
            checked={deleteExtra}
            onChange={(e) => setDeleteExtra(e.target.checked)}
          />
          Borrar extras
        </label>

        <label className="check">
          <input
            type="checkbox"
            checked={overwrite}
            onChange={(e) => setOverwrite(e.target.checked)}
          />
          Sobrescribir
        </label>
      </div>

      <button
        className="primary-btn"
        disabled={loading || !src || !dst}
        onClick={() =>
          runCommand("run_sync", {
            src,
            dst,
            recursive,
            deleteExtra,
            overwrite,
          })
        }
      >
        Ejecutar Sync
      </button>
    </ActionCard>
  );
}
