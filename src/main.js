const { invoke } = window.__TAURI__.primitives;
const { check } = window.__TAURI__.updater;

let greetInputEl;
let greetMsgEl;
let updateMsgEl;

async function greet() {
  await updateIfAvailable();

  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  updateMsgEl = document.querySelector("#update-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

async function updateIfAvailable() {
  let update;
  try {
    update = await check();
  } catch (e) {
    console.debug("failed check", e);
    return;
  }

  if (update !== null) {
    console.debug(`Update available for ${update.currentVersion}, updating to ${update.version}`);

    try {
      await update.downloadAndInstall(progressHandler);
    } catch (e) {
      console.debug("failed check", e);
      return;
    }
  } else {
    console.debug("Already running latest version");
  }
}

function progressHandler(progress) {
        switch (progress.event) {
          case "Started": {
            console.debug("Update event started", progress.data.contentLength);
            updateMsgEl.textContent = `Update available of size: ${progress.data.contentLength}`;
            break;
          }
          case "Progress": {
            console.debug("Update event progress", progress.data.chunkLength);
            updateMsgEl.textContent = `Update download progress: ${progress.data.chunkLength}`;
            break;
          }
          case "Finished": {
            console.debug("Update event finished");
            updateMsgEl.textContent = "Update finished";
            break;
          }
          default: {
            console.debug("Update event unrecognised");
            break;
          }
        }
      }