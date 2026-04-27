import type { ToastState } from "../types";

type Props = {
  toast: ToastState;
  onClose: () => void;
};

export function Toast({ toast, onClose }: Props) {
  if (!toast) return null;

  const icon =
    toast.type === "success" ? "✅" : toast.type === "error" ? "❌" : "ℹ️";

  return (
    <div className={`toast toast-${toast.type}`}>
      <span>{icon}</span>
      <p>{toast.message}</p>
      <button type="button" onClick={onClose}>
        ×
      </button>
    </div>
  );
}
