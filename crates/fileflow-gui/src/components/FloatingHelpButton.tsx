type Props = {
  onClick: () => void;
};

export function FloatingHelpButton({ onClick }: Props) {
  return (
    <button
      className="floating-help-btn"
      type="button"
      onClick={onClick}
      title="Abrir ayuda"
    >
      ?
    </button>
  );
}
