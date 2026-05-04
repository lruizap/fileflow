import { openUrl } from "@tauri-apps/plugin-opener";
import { GITHUB_URL } from "../app/navigation";

type Props = {
  compact?: boolean;
};

export function GitHubLink({ compact = false }: Props) {
  async function openGitHub() {
    await openUrl(GITHUB_URL);
  }

  return (
    <button
      type="button"
      className={compact ? "github-link compact" : "github-link"}
      onClick={openGitHub}
      title="Abrir repositorio oficial de FileFlow"
    >
      <span>GitHub</span>
      <strong>↗</strong>
    </button>
  );
}
