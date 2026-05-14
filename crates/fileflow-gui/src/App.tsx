import { useState } from "react";
import "./App.css";

import { useActionFormState } from "./app/useActionFormState";
import { useFileFlowRunner } from "./app/useFileFlowRunner";
import { usePersistentState } from "./app/usePersistentState";
import { useSavedPipelines } from "./app/useSavedPipelines";
import { AppShell } from "./app/AppShell";
import type { PageId } from "./app/navigation";

import { FloatingHelpButton } from "./components/FloatingHelpButton";
import { FloatingProgress } from "./components/FloatingProgress";
import { HelpModal } from "./components/HelpModal";
import { Toast } from "./components/Toast";

import { ActionsPage } from "./pages/ActionsPage";
import { ActivityPage } from "./pages/ActivityPage";
import { AboutPage } from "./pages/AboutPage";
import { GuidePage } from "./pages/GuidePage";
import { PipelinesPage } from "./pages/PipelinesPage";
import { WatchPage } from "./pages/WatchPage";

function App() {
  const [activePage, setActivePage] = useState<PageId>("actions");
  const [introDismissed, setIntroDismissed] = usePersistentState(
    "fileflow.introDismissed.v0.5.0",
    false,
  );
  const [showIntro, setShowIntro] = useState(!introDismissed);

  const forms = useActionFormState();
  const runner = useFileFlowRunner();
  const savedPipelines = useSavedPipelines();

  function closeIntro() {
    setShowIntro(false);
    setIntroDismissed(true);
  }

  return (
    <>
      {showIntro && <HelpModal onClose={closeIntro} />}

      <Toast toast={runner.toast} onClose={() => runner.setToast(null)} />

      <FloatingProgress
        progress={runner.progress}
        visible={runner.showProgress && runner.loading}
        cancelling={runner.cancelling}
        onCancel={runner.cancelCurrentJob}
      />

      <AppShell
        activePage={activePage}
        status={runner.status}
        onChangePage={setActivePage}
      >
        {activePage === "actions" && (
          <ActionsPage
            loading={runner.loading}
            history={runner.history}
            forms={forms}
            runCommand={runner.runCommand}
            onClearHistory={() => runner.setHistory([])}
          />
        )}

        {activePage === "pipelines" && (
          <PipelinesPage
            loading={runner.loading}
            configPath={forms.configPath}
            pipelines={savedPipelines.pipelines}
            setConfigPath={forms.setConfigPath}
            onRememberPipeline={savedPipelines.rememberPipeline}
            onForgetPipeline={savedPipelines.forgetPipeline}
            runCommand={runner.runCommand}
          />
        )}

        {activePage === "watch" && (
          <WatchPage
            loading={runner.loading}
            watchPath={forms.watchPath}
            configPath={forms.configPath}
            recursive={forms.watchRecursive}
            debounceMs={forms.watchDebounceMs}
            setWatchPath={forms.setWatchPath}
            setConfigPath={forms.setConfigPath}
            setRecursive={forms.setWatchRecursive}
            setDebounceMs={forms.setWatchDebounceMs}
            runCommand={runner.runCommand}
            onCancel={runner.cancelCurrentJob}
          />
        )}

        {activePage === "activity" && (
          <ActivityPage
            logs={runner.logs}
            history={runner.history}
            onClearLogs={() =>
              runner.setLogs([
                "Logs limpiados. Ejecuta una acción para ver resultados.",
              ])
            }
            onClearHistory={() => runner.setHistory([])}
          />
        )}

        {activePage === "guide" && <GuidePage />}

        {activePage === "about" && <AboutPage />}
      </AppShell>

      <FloatingHelpButton onClick={() => setShowIntro(true)} />
    </>
  );
}

export default App;
