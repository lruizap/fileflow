export type PageId =
  | "actions"
  | "pipelines"
  | "watch"
  | "activity"
  | "guide"
  | "about";

export type NavItem = {
  id: PageId;
  label: string;
  description: string;
  icon: string;
};

export const GITHUB_URL = "https://github.com/lruizap/fileflow";

export const NAV_ITEMS: NavItem[] = [
  {
    id: "actions",
    label: "Acciones",
    description: "Copy, move, sync y prueba rápida",
    icon: "⚡",
  },
  {
    id: "pipelines",
    label: "Pipelines",
    description: "Automatizaciones JSON",
    icon: "🔗",
  },
  {
    id: "watch",
    label: "Vigilar",
    description: "Automatiza cambios en carpetas",
    icon: "👁",
  },
  {
    id: "activity",
    label: "Actividad",
    description: "Historial, logs y progreso",
    icon: "📊",
  },
  {
    id: "guide",
    label: "Guía rápida",
    description: "Cómo usar FileFlow paso a paso",
    icon: "📘",
  },
  {
    id: "about",
    label: "Proyecto",
    description: "Info, GitHub y actualizaciones",
    icon: "❔",
  },
];
