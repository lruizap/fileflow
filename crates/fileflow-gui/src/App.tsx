import { useState } from "react";
import "./App.css";

import { useActionFormState } from "./app/useActionFormState";
import { useFileFlowRunner } from "./app/useFileFlowRunner";
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

function App() {
  const [activePage, setActivePage] = useState<PageId>("actions");
  const [showIntro, setShowIntro] = useState(true);

  const forms = useActionFormState();
  const runner = useFileFlowRunner();

  return (
    <>
      {showIntro && <HelpModal onClose={() => setShowIntro(false)} />}

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
            setConfigPath={forms.setConfigPath}
            runCommand={runner.runCommand}
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
