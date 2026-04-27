import { useState } from "react";
import "./App.css";

import { invokeFileFlow } from "./api/fileflow";
import { FloatingHelpButton } from "./components/FloatingHelpButton";
import { Header } from "./components/Header";
import { HelpModal } from "./components/HelpModal";
import { HistoryPanel } from "./components/HistoryPanel";
import { LogsPanel } from "./components/LogsPanel";
import { Toast } from "./components/Toast";
import { CopyCard } from "./features/CopyCard";
import { EchoCard } from "./features/EchoCard";
import { MoveCard } from "./features/MoveCard";
import { PipelineCard } from "./features/PipelineCard";
import { SyncCard } from "./features/SyncCard";
import type { HistoryItem, RunCommand, ToastState } from "./types";

function App() {
  const [showIntro, setShowIntro] = useState(true);

  const [copySrc, setCopySrc] = useState("");
  const [copyDst, setCopyDst] = useState("");

  const [moveSrc, setMoveSrc] = useState("");
  const [moveDst, setMoveDst] = useState("");

  const [syncSrc, setSyncSrc] = useState("");
  const [syncDst, setSyncDst] = useState("");

  const [configPath, setConfigPath] = useState("");

  const [recursive, setRecursive] = useState(true);
  const [deleteExtra, setDeleteExtra] = useState(false);
  const [overwrite, setOverwrite] = useState(false);

  const [status, setStatus] = useState("READY");
  const [logs, setLogs] = useState<string[]>([
    "FileFlow GUI inicializada.",
    "Selecciona una acción para comenzar.",
  ]);

  const [history, setHistory] = useState<HistoryItem[]>([]);
  const [toast, setToast] = useState<ToastState>(null);
  const [loading, setLoading] = useState(false);

  function pushHistory(label: string, command: string, status: string) {
    const item: HistoryItem = {
      id: crypto.randomUUID(),
      label,
      command,
      status,
      createdAt: new Date().toLocaleTimeString(),
    };

    setHistory((prev) => [item, ...prev].slice(0, 20));
  }

  function showToast(toast: ToastState) {
    setToast(toast);
    window.setTimeout(() => setToast(null), 4500);
  }

  const runCommand: RunCommand = async (command, args, label = command) => {
    setLoading(true);
    setStatus("RUNNING");
    setLogs((prev) => [`Ejecutando: ${label}`, ...prev]);

    try {
      const result = await invokeFileFlow(command, args);
      setStatus(result.status);
      setLogs(result.logs);
      pushHistory(label, command, result.status);

      if (result.status.includes("SUCCESS")) {
        showToast({
          type: "success",
          message: `${label} completado correctamente.`,
        });
      } else {
        showToast({
          type: "error",
          message: `${label} terminó con errores.`,
        });
      }
    } catch (err) {
      const errorMessage =
        typeof err === "string" ? err : JSON.stringify(err, null, 2);

      setStatus("ERROR");
      setLogs(["Error ejecutando comando:", errorMessage]);
      pushHistory(label, command, "ERROR");

      showToast({
        type: "error",
        message: `No se pudo ejecutar ${label}.`,
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="app">
      {showIntro && <HelpModal onClose={() => setShowIntro(false)} />}

      <Toast toast={toast} onClose={() => setToast(null)} />

      <Header status={status} />

      <section className="grid">
        <div className="stack-card">
          <EchoCard loading={loading} runCommand={runCommand} />

          <HistoryPanel history={history} onClear={() => setHistory([])} />
        </div>

        <CopyCard
          loading={loading}
          src={copySrc}
          dst={copyDst}
          overwrite={overwrite}
          setSrc={setCopySrc}
          setDst={setCopyDst}
          setOverwrite={setOverwrite}
          runCommand={runCommand}
        />

        <MoveCard
          loading={loading}
          src={moveSrc}
          dst={moveDst}
          overwrite={overwrite}
          setSrc={setMoveSrc}
          setDst={setMoveDst}
          setOverwrite={setOverwrite}
          runCommand={runCommand}
        />

        <SyncCard
          loading={loading}
          src={syncSrc}
          dst={syncDst}
          recursive={recursive}
          deleteExtra={deleteExtra}
          overwrite={overwrite}
          setSrc={setSyncSrc}
          setDst={setSyncDst}
          setRecursive={setRecursive}
          setDeleteExtra={setDeleteExtra}
          setOverwrite={setOverwrite}
          runCommand={runCommand}
        />

        <PipelineCard
          loading={loading}
          configPath={configPath}
          setConfigPath={setConfigPath}
          runCommand={runCommand}
        />

        <LogsPanel
          logs={logs}
          onClear={() =>
            setLogs(["Logs limpiados. Ejecuta una acción para ver resultados."])
          }
        />
      </section>

      <FloatingHelpButton onClick={() => setShowIntro(true)} />
    </main>
  );
}

export default App;
