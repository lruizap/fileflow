import fileflowIcon from "../assets/fileflow-icon.svg";
import { APP_VERSION } from "../app/constants";
import { StatusPill } from "./StatusPill";

type Props = {
  status: string;
};

export function Header({ status }: Props) {
  return (
    <section className="hero">
      <div className="brand-header">
        <img src={fileflowIcon} alt="FileFlow icon" className="app-logo" />

        <div>
          <p className="eyebrow">FileFlow v{APP_VERSION}</p>
          <h1>Automatización local de archivos</h1>
          <p className="subtitle">
            Elige archivos y carpetas, ejecuta acciones y controla el resultado
            desde una interfaz visual.
          </p>
        </div>
      </div>

      <StatusPill status={status} />
    </section>
  );
}
