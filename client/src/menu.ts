export class Component {
  constructor(private title: string, private readonly action?: (ctx: Menu) => void) {
    this.title = title;
    this.action = action;
  }

  public handle(ctx: Menu): void {
    if (this.action) this.action(ctx);
  }

  public render(selected: boolean): string {
    return selected ? `> ${this.title}` : this.title;
  }
}

export class Menu {
  public manager: MenuManager | null;
  private components: Component[];
  private selected: number;

  constructor(private readonly title: string, private readonly font: Font, private readonly pad: Pad) {
    this.manager = null;
    this.components = [];
    this.selected = 0;
  }

  public addComponent(component: Component): void {
    this.components.push(component);
  }

  reset() {
    this.selected = 0;
    this.components = [];
  }

  render() {
    this.__control();
    this.font.print(0, 0, this.title);

    const offset = 30;
    this.components.forEach((component, index) => {
      const x = 0;
      const y = this.font.getTextSize(this.title).height + (offset * index);
      this.font.print(x, y, component.render(this.selected === index));
    })
  }

  __control() {
    if (this.pad.justPressed(Pads.START)) {
      this.components[this.selected].handle(this);
    }

    if(this.pad.justPressed(Pads.UP)){
      if (this.selected <= 0) return;
      this.selected -= 1;
    }

    if(this.pad.justPressed(Pads.DOWN)){
      if (this.selected >= this.components.length - 1) return;
      this.selected += 1;
    }
  }
}

export class MenuManager {
  private readonly menus: Record<string, Menu>;
  private current: string | undefined;

  constructor() {
    this.menus = {};
  }

  addMenu(id: string, menu: Menu) {
    menu.manager = this;
    this.current ??= id;
    this.menus[id] = menu;
  }

  set(id: string) {
    if (this.menus.hasOwnProperty(id)) this.current = id;
    return this.menus[this.current!];
  }

  render() {
    this.menus[this.current!].render();
  }
}
