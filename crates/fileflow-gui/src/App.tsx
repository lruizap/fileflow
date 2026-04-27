import { useState } from "react";
import "./App.css";

import { invokeFileFlow } from "./api/fileflow";
import { FloatingHelpButton } from "./components/FloatingHelpButton";
import { Header } from "./components/Header";
import { HelpModal } from "./components/HelpModal";
import { LogsPanel } from "./components/LogsPanel";
import { CopyCard } from "./features/CopyCard";
import { EchoCard } from "./features/EchoCard";
import { MoveCard } from "./features/MoveCard";
import { PipelineCard } from "./features/PipelineCard";
import { SyncCard } from "./features/SyncCard";
import type { RunCommand } from "./types";

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

  const [loading, setLoading] = useState(false);

  const runCommand: RunCommand = async (command, args) => {
    setLoading(true);
    setStatus("RUNNING");
    setLogs((prev) => [`Ejecutando: ${command}`, ...prev]);

    try {
      const result = await invokeFileFlow(command, args);
      setStatus(result.status);
      setLogs(result.logs);
    } catch (err) {
      setStatus("ERROR");
      setLogs([
        "Error ejecutando comando:",
        typeof err === "string" ? err : JSON.stringify(err, null, 2),
      ]);
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="app">
      {showIntro && <HelpModal onClose={() => setShowIntro(false)} />}

      <Header status={status} />

      <section className="grid">
        <EchoCard loading={loading} runCommand={runCommand} />

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

        <LogsPanel logs={logs} />
      </section>

      <FloatingHelpButton onClick={() => setShowIntro(true)} />
    </main>
  );
}

export default App;
