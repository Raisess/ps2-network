export class UIItem {
  constructor(
    public title: string,
    private readonly action?: (ctx: UIComponent) => void,
  ) {
    this.title = title;
    this.action = action;
  }

  public handle(ctx: UIComponent): void {
    if (this.action) this.action(ctx);
  }

  public text(selected: boolean): string {
    return selected ? `> ${this.title}` : this.title;
  }
}

export class UIComponent {
  private items: UIItem[];
  private selected: number;

  constructor(
    public readonly parent: UI,
    private readonly title: string,
    private readonly font: Font,
    private readonly pad: Pad,
  ) {
    this.items = [];
    this.selected = 0;
  }

  public addItem(item: UIItem): void {
    this.items.push(item);
  }

  reset() {
    this.selected = 0;
    this.items = [];
  }

  render() {
    this.__control();
    this.font.print(0, 0, this.title);

    const offset = 30;
    this.items.forEach((item, index) => {
      const x = 0;
      const y = this.font.getTextSize(this.title).height + (offset * index);

      this.font.print(x, index === 0 ? y + 5 : y, item.text(this.selected === index));
    })
  }

  __control() {
    if (this.pad.justPressed(Pads.START)) {
      this.items[this.selected].handle(this);
    }

    if(this.pad.justPressed(Pads.UP)){
      if (this.selected <= 0) return;
      this.selected -= 1;
    }

    if(this.pad.justPressed(Pads.DOWN)){
      if (this.selected >= this.items.length - 1) return;
      this.selected += 1;
    }
  }
}

export class UI {
  private readonly components: Record<string, UIComponent>;
  private current: string | undefined;

  constructor(private readonly font: Font, private readonly pad: Pad) {
    this.components = {};
  }

  createComponent(id: string, title: string): UIComponent {
    this.current = id;
    this.components[id] = new UIComponent(this, title, this.font, this.pad);
    return this.components[id];
  }

  set(id: string): UIComponent {
    if (this.components.hasOwnProperty(id)) this.current = id;
    return this.components[this.current!];
  }

  render() {
    this.components[this.current!].render();
  }
}
