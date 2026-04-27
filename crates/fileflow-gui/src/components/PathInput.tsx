type Props = {
  label: string;
  value: string;
  buttonText: string;
  onChange: (value: string) => void;
  onPick: () => void;
};

export function PathInput({
  label,
  value,
  buttonText,
  onChange,
  onPick,
}: Props) {
  return (
    <label>
      {label}
      <div className="path-row">
        <input value={value} onChange={(e) => onChange(e.target.value)} />
        <button type="button" className="small-btn" onClick={onPick}>
          {buttonText}
        </button>
      </div>
    </label>
  );
}
