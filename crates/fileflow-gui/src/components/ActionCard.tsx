import type { ReactNode } from "react";

type Props = {
  title: string;
  description: string;
  badge: string;
  children: ReactNode;
  className?: string;
};

export function ActionCard({
  title,
  description,
  badge,
  children,
  className = "",
}: Props) {
  return (
    <article className={`card ${className}`}>
      <div className="card-header">
        <div>
          <h2>{title}</h2>
          <p>{description}</p>
        </div>
        <span className="badge">{badge}</span>
      </div>

      {children}
    </article>
  );
}
