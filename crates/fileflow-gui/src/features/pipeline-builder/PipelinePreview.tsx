type Props = {
  jsonPreview: string;
  savedPath: string;
};

export function PipelinePreview({ jsonPreview, savedPath }: Props) {
  return (
    <div className="pipeline-preview">
      <div className="pipeline-preview-header">
        <strong>JSON generado</strong>
        {savedPath && <span>{savedPath}</span>}
      </div>

      <pre>{jsonPreview}</pre>
    </div>
  );
}
