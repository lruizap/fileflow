import { ActionCard } from "../components/ActionCard";
import { PathInput } from "../components/PathInput";
import { pickDirectory, pickJsonFile } from "../api/fileflow";
import type { RunCommand } from "../types";

type Props = {
  loading: boolean;
  watchPath: string;
  configPath: string;
  recursive: boolean;
  debounceMs: number;
  setWatchPath: (value: string) => void;
  setConfigPath: (value: string) => void;
  setRecursive: (value: boolean) => void;
  setDebounceMs: (value: number) => void;
  runCommand: RunCommand;
  onCancel: () => void;
};

export function WatchPage({
  loading,
  watchPath,
  configPath,
  recursive,
  debounceMs,
  setWatchPath,
  setConfigPath,
  setRecursive,
  setDebounceMs,
  runCommand,
  onCancel,
}: Props) {
  async function selectWatchPath() {
    const selected = await pickDirectory();
    if (selected) setWatchPath(selected);
  }

  async function selectPipeline() {
    const selected = await pickJsonFile();
    if (selected) setConfigPath(selected);
  }

  const canStart = Boolean(watchPath && configPath && debounceMs > 0);

  return (
    <section className="page-section">
      <div className="page-title">
        <span>👁</span>
        <div>
          <h2>Vigilar carpeta</h2>
          <p>
            Ejecuta una automatización cuando FileFlow detecte cambios en una
            carpeta.
          </p>
        </div>
      </div>

      <div className="single-column wide-column">
        <ActionCard
          title="Watcher de automatizaciones"
          description="Mantén una carpeta bajo vigilancia y lanza un pipeline cuando cambie."
          badge="watch"
        >
          <PathInput
            label="Carpeta a vigilar"
            value={watchPath}
            onChange={setWatchPath}
            buttonText="Elegir carpeta"
            onPick={selectWatchPath}
          />

          <PathInput
            label="Pipeline JSON"
            value={configPath}
            onChange={setConfigPath}
            buttonText="Elegir JSON"
            onPick={selectPipeline}
          />

          <div className="options">
            <label className="check">
              <input
                type="checkbox"
                checked={recursive}
                onChange={(e) => setRecursive(e.target.checked)}
              />
              Vigilar subcarpetas
            </label>

            <label>
              Debounce en milisegundos
              <input
                min={100}
                step={100}
                type="number"
                value={debounceMs}
                onChange={(e) => setDebounceMs(Number(e.target.value))}
              />
            </label>
          </div>

          <div className="button-row">
            <button
              className="primary-btn"
              type="button"
              disabled={loading || !canStart}
              onClick={() =>
                runCommand(
                  "run_watch",
                  {
                    path: watchPath,
                    config: configPath,
                    recursive,
                    debounceMs,
                  },
                  "Vigilar carpeta",
                )
              }
            >
              Iniciar vigilancia
            </button>

            <button
              className="secondary-btn"
              type="button"
              disabled={!loading}
              onClick={onCancel}
            >
              Detener vigilancia
            </button>
          </div>
        </ActionCard>
      </div>
    </section>
  );
}

