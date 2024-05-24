import { invoke } from "@tauri-apps/api/core";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#share")?.addEventListener("click", (e) => {
    navigator.mediaDevices.getDisplayMedia()
    .then(function(mediaStream) {
        const video: HTMLVideoElement = document.querySelector('#stream')!!;
        video.srcObject = mediaStream;
        video.onloadedmetadata = function() {
            console.log('stream loaded');
            video.play();
        }
    })
    .catch(function(err) { console.log(err.name + ": " + err.message); })
  })
})
