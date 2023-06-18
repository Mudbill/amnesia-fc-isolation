import { dialog, invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { FaCheck, FaFolder, FaSpinner } from "react-icons/fa";

export default function App() {
  const [gameDir, setGameDir] = useState("");
  const [targetMod, setTargetMod] = useState("");
  const [isInstalling, setIsInstalling] = useState(false);
  const [isComplete, setIsComplete] = useState(false);
  const [mainInit, setMainInit] = useState("");

  useEffect(() => {
    invoke("show_window");
  }, []);

  const handleClickInstall = async () => {
    setIsInstalling(true);
    const mainInitPath = await invoke("install_mod", {
      gameDir,
      targetMod,
    });
    setIsInstalling(false);
    setIsComplete(true);
    console.log(mainInitPath);
    if (typeof mainInitPath === "string") {
      setMainInit(mainInitPath.slice(mainInitPath.indexOf("custom_stories")));
    }
  };

  const handleClickOpenDir = async () => {
    const selected = await dialog.open({
      directory: true,
      title: "Open Amnesia game location",
    });
    if (!selected || Array.isArray(selected)) {
      return;
    }
    setGameDir(selected);
  };

  const handleClickOpenModTarget = async () => {
    const selected = await dialog.open({
      title: "Select mod to install (.zip)",
      filters: [
        {
          extensions: ["zip"],
          name: "Archive (.zip)",
        },
      ],
    });
    if (!selected || Array.isArray(selected)) {
      return;
    }
    setTargetMod(selected);
  };

  return (
    <main className="select-none cursor-default">
      <h1 className="text-center text-xl my-4">Amnesia TDD Mod Installer</h1>
      <div className="p-4 flex flex-col gap-2">
        <div>
          <label>Game location:</label>
          <div className="flex gap-2">
            <input
              className="w-full border border-zinc-700 rounded text-sm"
              value={gameDir}
              onChange={(e) => setGameDir(e.target.value)}
            />
            <button
              className="p-1 text-blue-300 border border-zinc-700 rounded bg-zinc-900"
              type="button"
              onClick={handleClickOpenDir}
            >
              <FaFolder />
            </button>
          </div>
        </div>
        <div>
          <label>Mod to install:</label>
          <div className="flex gap-2">
            <input
              className="w-full border border-zinc-700 rounded text-sm"
              value={targetMod}
              onChange={(e) => setTargetMod(e.target.value)}
            />
            <button
              className="p-1 text-blue-300 border border-zinc-700 rounded bg-zinc-900"
              type="button"
              onClick={handleClickOpenModTarget}
            >
              <FaFolder />
            </button>
          </div>
        </div>
        <hr className="border-zinc-700" />
        <div className="flex justify-center">
          <button
            className="px-3 border text-xl border-zinc-700 rounded bg-zinc-900"
            type="button"
            onClick={handleClickInstall}
            disabled={isInstalling}
          >
            Install
          </button>
        </div>
        {isInstalling ? (
          <div className="flex justify-center">
            <FaSpinner className="animate-spin" />
          </div>
        ) : (
          isComplete && (
            <div className="flex flex-col items-center break-all">
              <FaCheck className="text-green-500" />
              <p>Ok, now launch the game with the argument:</p>
              <code className="select-text cursor-text text-sm bg-zinc-900 border p-1">{mainInit}</code>
            </div>
          )
        )}
      </div>
    </main>
  );
}
