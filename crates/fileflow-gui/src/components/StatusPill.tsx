type Props = {
  status: string;
};

export function StatusPill({ status }: Props) {
  const statusClass = status.includes("SUCCESS")
    ? "success"
    : status.includes("FAILED") || status.includes("ERROR")
      ? "error"
      : status.includes("RUNNING")
        ? "running"
        : "ready";

  return (
    <div className={`status-pill ${statusClass}`}>
      <span />
      {status}
    </div>
  );
}
