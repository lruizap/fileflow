import { pickDirectory, pickFile } from "../../api/fileflow";

type Props = {
  label: string;
  value: string;
  mode: "file" | "directory";
  onChange: (value: string) => void;
};

export function PathSelector({ label, value, mode, onChange }: Props) {
  async function selectPath() {
    const selected =
      mode === "directory" ? await pickDirectory() : await pickFile();
    if (selected) onChange(selected);
  }

  return (
    <label>
      {label}
      <div className="path-row">
        <input value={value} onChange={(e) => onChange(e.target.value)} />
        <button type="button" className="small-btn" onClick={selectPath}>
          Elegir
        </button>
      </div>
    </label>
  );
}
