import { HistoryPanel } from "../components/HistoryPanel";
import { CopyCard } from "../features/CopyCard";
import { EchoCard } from "../features/EchoCard";
import { MoveCard } from "../features/MoveCard";
import { SyncCard } from "../features/SyncCard";
import type { HistoryItem, RunCommand } from "../types";
import type { useActionFormState } from "../app/useActionFormState";

type FormState = ReturnType<typeof useActionFormState>;

type Props = {
  loading: boolean;
  history: HistoryItem[];
  forms: FormState;
  runCommand: RunCommand;
  onClearHistory: () => void;
};

export function ActionsPage({
  loading,
  history,
  forms,
  runCommand,
  onClearHistory,
}: Props) {
  return (
    <section className="page-section">
      <div className="page-title">
        <span>⚡</span>
        <div>
          <h2>Acciones rápidas</h2>
          <p>Copia, mueve y sincroniza archivos sin escribir comandos.</p>
        </div>
      </div>

      <section className="grid">
        <div className="stack-card">
          <EchoCard loading={loading} runCommand={runCommand} />
          <HistoryPanel history={history} onClear={onClearHistory} />
        </div>

        <CopyCard
          loading={loading}
          src={forms.copySrc}
          dst={forms.copyDst}
          overwrite={forms.copyOverwrite}
          setSrc={forms.setCopySrc}
          setDst={forms.setCopyDst}
          setOverwrite={forms.setCopyOverwrite}
          runCommand={runCommand}
        />

        <MoveCard
          loading={loading}
          src={forms.moveSrc}
          dst={forms.moveDst}
          overwrite={forms.moveOverwrite}
          setSrc={forms.setMoveSrc}
          setDst={forms.setMoveDst}
          setOverwrite={forms.setMoveOverwrite}
          runCommand={runCommand}
        />

        <SyncCard
          loading={loading}
          src={forms.syncSrc}
          dst={forms.syncDst}
          recursive={forms.recursive}
          deleteExtra={forms.deleteExtra}
          overwrite={forms.syncOverwrite}
          dryRun={forms.dryRun}
          setSrc={forms.setSyncSrc}
          setDst={forms.setSyncDst}
          setRecursive={forms.setRecursive}
          setDeleteExtra={forms.setDeleteExtra}
          setOverwrite={forms.setSyncOverwrite}
          setDryRun={forms.setDryRun}
          runCommand={runCommand}
        />
      </section>
    </section>
  );
}
