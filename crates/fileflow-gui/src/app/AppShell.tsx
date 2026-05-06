import type { ReactNode } from "react";
import { Sidebar } from "../components/Sidebar";
import { Header } from "../components/Header";
import type { PageId } from "./navigation";

type Props = {
  activePage: PageId;
  status: string;
  children: ReactNode;
  onChangePage: (page: PageId) => void;
};

export function AppShell({
  activePage,
  status,
  children,
  onChangePage,
}: Props) {
  return (
    <div className="app-shell">
      <Sidebar activePage={activePage} onChangePage={onChangePage} />

      <main className="app-main">
        <Header status={status} />
        {children}
      </main>
    </div>
  );
}
