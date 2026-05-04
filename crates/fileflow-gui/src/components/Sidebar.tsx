import fileflowIcon from "../assets/fileflow-icon.svg";
import { NAV_ITEMS, type PageId } from "../app/navigation";
import { GitHubLink } from "./GitHubLink";

type Props = {
  activePage: PageId;
  onChangePage: (page: PageId) => void;
};

export function Sidebar({ activePage, onChangePage }: Props) {
  return (
    <aside className="sidebar">
      <div className="sidebar-brand">
        <img src={fileflowIcon} alt="FileFlow" />
        <div>
          <strong>FileFlow</strong>
          <span>Automatizador local</span>
        </div>
      </div>

      <nav className="sidebar-nav">
        {NAV_ITEMS.map((item) => (
          <button
            key={item.id}
            type="button"
            className={activePage === item.id ? "nav-item active" : "nav-item"}
            onClick={() => onChangePage(item.id)}
          >
            <span>{item.icon}</span>
            <div>
              <strong>{item.label}</strong>
              <small>{item.description}</small>
            </div>
          </button>
        ))}
      </nav>

      <div className="sidebar-footer">
        <div className="sidebar-github-box">
          <strong>Actualizaciones</strong>
          <span>Descarga nuevas versiones desde GitHub.</span>
          <GitHubLink compact />
        </div>
      </div>
    </aside>
  );
}
