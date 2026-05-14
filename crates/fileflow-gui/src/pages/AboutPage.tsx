import { APP_VERSION } from "../app/constants";
import { GitHubLink } from "../components/GitHubLink";

export function AboutPage() {
  return (
    <section className="page-section">
      <div className="page-title">
        <span>❔</span>

        <div>
          <h2>Proyecto y actualizaciones</h2>
          <p>
            Información general de FileFlow y acceso al repositorio oficial.
          </p>
        </div>
      </div>

      <section className="about-grid">
        <article className="card info-card">
          <h2>¿Qué es FileFlow?</h2>
          <p>
            FileFlow es una aplicación local para automatizar tareas de
            archivos: copiar, mover, sincronizar carpetas y ejecutar
            automatizaciones JSON desde una interfaz gráfica.
          </p>
        </article>

        <article className="card info-card">
          <h2>Estado del proyecto</h2>
          <p>
            La versión {APP_VERSION} unifica el empaquetado, añade biblioteca de
            pipelines, persistencia local y vigilancia visual de carpetas.
          </p>
        </article>

        <article className="card github-card">
          <div className="github-card-content">
            <div className="github-card-text">
              <h2>Repositorio oficial</h2>
              <p>
                Revisa actualizaciones, releases y documentación del proyecto.
                Desde aquí podrás descargar nuevas versiones cuando estén
                disponibles.
              </p>
            </div>

            <GitHubLink />
          </div>
        </article>
      </section>
    </section>
  );
}
