const { invoke } = window.__TAURI__.tauri;

document.querySelector('#player-back').addEventListener('click', back);
document.querySelector('#player-play-pause').addEventListener('click', play);
document.querySelector('#player-forward').addEventListener('click', forward);

function back() {
    console.log("BACK!!");
}
function play() {
    console.log("PLAY!!");
}
function forward() {
    console.log("FORWARD!!");
}