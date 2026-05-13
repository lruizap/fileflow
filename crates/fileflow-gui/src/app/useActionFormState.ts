import { useState } from "react";

export function useActionFormState() {
  const [copySrc, setCopySrc] = useState("");
  const [copyDst, setCopyDst] = useState("");

  const [moveSrc, setMoveSrc] = useState("");
  const [moveDst, setMoveDst] = useState("");

  const [syncSrc, setSyncSrc] = useState("");
  const [syncDst, setSyncDst] = useState("");

  const [configPath, setConfigPath] = useState("");

  const [recursive, setRecursive] = useState(true);
  const [deleteExtra, setDeleteExtra] = useState(false);
  const [overwrite, setOverwrite] = useState(false);
  const [dryRun, setDryRun] = useState(false);

  return {
    copySrc,
    copyDst,
    moveSrc,
    moveDst,
    syncSrc,
    syncDst,
    configPath,
    recursive,
    deleteExtra,
    overwrite,
    dryRun,

    setCopySrc,
    setCopyDst,
    setMoveSrc,
    setMoveDst,
    setSyncSrc,
    setSyncDst,
    setConfigPath,
    setRecursive,
    setDeleteExtra,
    setOverwrite,
    setDryRun,
  };
}
