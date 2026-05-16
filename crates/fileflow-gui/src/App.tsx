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
import { QueuePanel } from "./components/QueuePanel";
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
  const watchRunning = runner.activeJobs.some((job) => job.command === "run_watch");
  const showQueuePanel =
    activePage === "actions" || activePage === "pipelines" || activePage === "watch";

  function closeIntro() {
    setShowIntro(false);
    setIntroDismissed(true);
  }

  return (
    <>
      {showIntro && <HelpModal onClose={closeIntro} />}

      <Toast toast={runner.toast} onClose={() => runner.setToast(null)} />

      <FloatingProgress
        jobs={runner.activeJobs}
        visible={runner.loading}
        cancellingJobIds={runner.cancellingJobIds}
        onCancel={runner.cancelJob}
        onUpdatePriority={runner.updateJobPriority}
      />

      <AppShell
        activePage={activePage}
        status={runner.status}
        onChangePage={setActivePage}
      >
        {showQueuePanel && (
          <QueuePanel
            jobs={runner.queue.jobs}
            runningCount={runner.queue.runningCount}
            queuedCount={runner.queue.queuedCount}
            concurrencyLimit={runner.queue.concurrencyLimit}
            defaultPriority={runner.defaultPriority}
            cancellingJobIds={runner.cancellingJobIds}
            onSetConcurrencyLimit={runner.setConcurrencyLimit}
            onSetDefaultPriority={runner.setDefaultPriority}
            onUpdatePriority={runner.updateJobPriority}
            onCancel={runner.cancelJob}
          />
        )}

        {activePage === "actions" && (
          <ActionsPage
            loading={false}
            history={runner.history}
            forms={forms}
            runCommand={runner.runCommand}
            onClearHistory={() => runner.setHistory([])}
          />
        )}

        {activePage === "pipelines" && (
          <PipelinesPage
            loading={false}
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
            loading={watchRunning}
            watchPath={forms.watchPath}
            configPath={forms.configPath}
            recursive={forms.watchRecursive}
            debounceMs={forms.watchDebounceMs}
            setWatchPath={forms.setWatchPath}
            setConfigPath={forms.setConfigPath}
            setRecursive={forms.setWatchRecursive}
            setDebounceMs={forms.setWatchDebounceMs}
            runCommand={runner.runCommand}
            onCancel={() => {
              const watchJob = runner.activeJobs.find((job) => job.command === "run_watch");
              if (watchJob) runner.cancelJob(watchJob.id);
            }}
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
