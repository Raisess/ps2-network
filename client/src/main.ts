import { PS2NetworkClient } from "./ps2n-client";
import { UI, UIItem } from "./ui";

const SERVER_IP = "192.168.3.10";
const SERVER_PORT = 8080;

IOP.loadDefaultModule(IOP.network);
console.log("INITING...");
Network.init();
console.log("DONE!");

const canvas = Screen.getMode();
canvas.zbuffering = true;
canvas.psmz = Z16S;
Screen.setMode(canvas);

const font = new Font("default");
const pad = Pads.get();
pad.setEventHandler();

const client = new PS2NetworkClient(SERVER_IP, SERVER_PORT)
const ui = new UI(font, pad);

const backToMainItem = new UIItem("Back", (ctx) => {
  ctx.parent.set("main");
});

// -------- MAIN SCREEN -------- //
const main = ui.createComponent("main", `PS2 Network: ${SERVER_IP}:${SERVER_PORT}`);

// -------- SEARCH SCREEN -------- //
main.addItem(new UIItem("Search Games", (ctx) => {
  const to = ctx.parent.createKeyboardComponent();
  to.setup((value, ctx) => {
    switch (value) {
      case '0':
        ctx.parent.set("main");
        break;
      case '<':
        ctx.buffer = ctx.buffer.slice(0, ctx.buffer.length - 1);
        break;
      case '>':
        console.log(ctx.buffer.length, ctx.buffer);
        const to = ctx.parent.createComponent("results", "Results");
        to.addItem(backToMainItem);
        to.addItem(new UIItem(ctx.buffer));
        ctx.buffer = "";
        break;
      default:
        ctx.buffer += value;
        break;
    }
  });
}));

// -------- DOWNLOADS SCREEN -------- //
main.addItem(new UIItem("My Downloads", (ctx) => {
  const to = ctx.parent.createComponent("downloads", "Downloads");
  to.addItem(backToMainItem);

  const downloads = client.listDownloads();
  downloads.forEach((download) => {
    to.addItem(new UIItem(download.name));
  });
}));

// -------- PING SCREEN -------- //
main.addItem(new UIItem("Ping Server", (ctx) => {
  const pong = client.ping();
  const to = ctx.parent.createComponent("ping", `Server ping: ${pong}`);
  to.addItem(backToMainItem);
}));

ui.set("main");
os.setInterval(() => {
  //console.log('MEM CORE:', System.getMemoryStats().core)
  //console.log('MEM USED:', System.getMemoryStats().used)
  Screen.clear();
  ui.render();
  Screen.flip();
}, 0);
