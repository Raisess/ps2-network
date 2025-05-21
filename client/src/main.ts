import { PS2NetworkClient } from "./ps2n-client";
import { UI, UIItem } from "./ui";

// @TODO: read this from config.txt file
const SERVER_IP = "192.168.3.10";
const SERVER_PORT = 8080;

console.log("INITING NET...");
IOP.loadDefaultModule(IOP.network);
Network.init();
console.log("DONE INITING NET!");

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
        // -------- SEARCH RESULTS SUBSCREEN -------- //
        const to = ctx.parent.createComponent("results", "Results");
        to.addItem(backToMainItem);
        const results = client.search(encodeURIComponent(ctx.buffer))
        results.forEach((item) => {
          to.addItem(new UIItem(item.name, (ctx) => {
            client.download(item.id);
            ctx.parent.set("main");
          }));
        });
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
