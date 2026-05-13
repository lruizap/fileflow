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
  dryRun: boolean;
  setSrc: (value: string) => void;
  setDst: (value: string) => void;
  setRecursive: (value: boolean) => void;
  setDeleteExtra: (value: boolean) => void;
  setOverwrite: (value: boolean) => void;
  setDryRun: (value: boolean) => void;
  runCommand: RunCommand;
};

export function SyncCard({
  loading,
  src,
  dst,
  recursive,
  deleteExtra,
  overwrite,
  dryRun,
  setSrc,
  setDst,
  setRecursive,
  setDeleteExtra,
  setOverwrite,
  setDryRun,
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

  function runSync() {
    if (deleteExtra && !dryRun) {
      const confirmed = window.confirm(
        "Esta sincronización puede borrar archivos que existan solo en la carpeta destino. Revisa las rutas antes de continuar.",
      );

      if (!confirmed) return;
    }

    runCommand(
      "run_sync",
      {
        src,
        dst,
        recursive,
        deleteExtra,
        overwrite,
        dryRun,
      },
      dryRun ? "Previsualizar sincronización" : "Sincronizar carpetas",
    );
  }

  return (
    <ActionCard
      title="Sincronizar carpetas"
      description="Actualiza una carpeta destino con el contenido de una carpeta origen."
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
          Incluir subcarpetas
        </label>

        <label className="check">
          <input
            type="checkbox"
            checked={deleteExtra}
            onChange={(e) => setDeleteExtra(e.target.checked)}
          />
          Borrar archivos que sobren en destino
        </label>

        <label className="check">
          <input
            type="checkbox"
            checked={overwrite}
            onChange={(e) => setOverwrite(e.target.checked)}
          />
          Sobrescribir archivos
        </label>

        <label className="check">
          <input
            type="checkbox"
            checked={dryRun}
            onChange={(e) => setDryRun(e.target.checked)}
          />
          Solo previsualizar cambios
        </label>
      </div>

      <button
        className="primary-btn"
        disabled={loading || !src || !dst}
        onClick={runSync}
      >
        {dryRun ? "Previsualizar sincronización" : "Sincronizar carpetas"}
      </button>
    </ActionCard>
  );
}
