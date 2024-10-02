const cached = new Map<string, unknown>();

function loadAudio (url: string) {
    const audio = new Audio(url);
    return {
        play(){
            audio.currentTime = 0;
            audio.play();
        }
    }
}

export function loadBGM () {
    let bgm = cached.get("bgm") as undefined | HTMLAudioElement;
    if (!bgm) {
        bgm = new Audio("./BGM2.mp3");
        bgm.loop = true;
        cached.set("bgm", bgm);
    }
    bgm.innerText = "Witching Dream...";
    return bgm;
}

export function loadSFX () {
    let sfx = cached.get("sfx");
    if (!sfx) {
        sfx = {
            move: loadAudio("./ogg/move.ogg"),
            line: loadAudio("./ogg/line.ogg")
        }
        cached.set("sfx", sfx);
    }
    return sfx as Record<"move" | "line", ReturnType<typeof loadAudio>>;
}