import { Component, Menu, MenuManager } from "./menu";
import { PS2NetworkClient } from "./ps2n-client";

const SERVER_IP = "192.168.3.10";
const SERVER_PORT = 8080;

IOP.loadDefaultModule(IOP.network);
console.log("INITING...");
Network.init();
console.log("DONE!");

const font = new Font("default");
const pad = Pads.get();
pad.setEventHandler();

const client = new PS2NetworkClient(SERVER_IP, SERVER_PORT)

const backToMainComponent = new Component("Back", (ctx) => {
  ctx.manager!.set("main");
});

const title = `PS2 Network: ${SERVER_IP}:${SERVER_PORT}`;
const main = new Menu(title, font, pad);

main.addComponent(new Component("Search Games", (ctx) => {
  const menu = ctx.manager!.set("search");
  menu.reset();
  menu.addComponent(backToMainComponent);
}));

main.addComponent(new Component("My Downloads", (ctx) => {
  const downloads = client.listDownloads();

  const menu = ctx.manager!.set("downloads");
  menu.reset();
  menu.addComponent(backToMainComponent);
  downloads.forEach((download) => {
    menu.addComponent(new Component(download.name));
  });
}));

const menuManager = new MenuManager();
menuManager.addMenu("main", main);
menuManager.addMenu("search", new Menu("Search Games", font, pad));
menuManager.addMenu("downloads", new Menu("My Downloads", font, pad));

os.setInterval(() => {
  Screen.clear();
  menuManager.render();
  Screen.flip();
}, 0);
