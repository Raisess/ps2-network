export class UI {
  private readonly components: Record<string, AbstractUIComponent>;
  private current: string | undefined;

  constructor(private readonly font: Font, private readonly pad: Pad) {
    this.components = {};
  }

  createComponent(id: string, title: string): UIComponent {
    this.current = id;
    this.components[id] = new UIComponent(this, title, this.font, this.pad);
    return this.components[id] as UIComponent;
  }

  createKeyboardComponent(): KeyboardUIComponent {
    this.current = "keyboard";
    this.components["keyboard"] = new KeyboardUIComponent(this, "Type your text:", this.font, this.pad);
    return this.components["keyboard"] as KeyboardUIComponent;
  }

  set(id: string): UIComponent {
    if (this.components.hasOwnProperty(id)) this.current = id;
    return this.components[this.current!] as UIComponent;
  }

  render() {
    this.components[this.current!].render();
  }
}

abstract class AbstractUIComponent {
  protected selected: number;

  constructor(
    public readonly parent: UI,
    protected readonly title: string,
    protected readonly font: Font,
    protected readonly pad: Pad,
  ) {
    this.selected = 0;
  }

  public abstract render(): void;
}

const OFFSET = 30;

export class UIItem {
  constructor(
    public readonly title: string,
    public readonly description: string,
    private readonly action?: (ctx: AbstractUIComponent) => void,
  ) {
    this.title = title;
    this.action = action;
  }

  public handle(ctx: AbstractUIComponent): void {
    if (this.action) this.action(ctx);
  }
}

class UIComponent extends AbstractUIComponent {
  private items: UIItem[] = [];

  public addItem(item: UIItem): void {
    this.items.push(item);
  }

  public render(): void {
    this.control();
    this.font.print(0, 0, this.title);

    this.items.forEach((item, index) => {
      const y = OFFSET + (OFFSET * index);
      this.font.print(
        0,
        index === 0 ? y + 5 : y,
        this.selected === index ? `> ${item.title}` : item.title,
      );
    })

    const selected = this.items[this.selected];
    this.font.print(0, (this.items.length * OFFSET) + OFFSET, selected.description);
  }

  private control(): void {
    if (this.pad.justPressed(Pads.CROSS)) {
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

const VALUES = "1234567890qwertyuiopasdfghjkl-zxcvbnm <>";
const LINE_SIZE = 10;
const LINE_COUNT = Math.ceil(VALUES.length / LINE_SIZE);

class KeyboardUIComponent extends AbstractUIComponent {
  public buffer: string = "";

  private back: string = "";
  private handler!: (value: string, ctx: KeyboardUIComponent) => void;

  public setup(back: string, action: (value: string, ctx: KeyboardUIComponent) => void): void {
    this.back = back;
    this.handler = action;
  }

  public render(): void {
    this.control();
    this.font.print(0, 0, `${this.title} ${this.buffer}`);

    for (let i = 0; i < LINE_COUNT; i++) {
      const line = VALUES.slice(i * LINE_SIZE, (i + 1) * LINE_SIZE);
      line.split('').forEach((v, j) => {
        const x = j * OFFSET;
        const y = OFFSET + (OFFSET * i);
        // @TODO: calculate the selected value based on i and j
        const value = VALUES[this.selected] === v ? `*${v}` : v;
        this.font.print(x, i === 0 ? y + 10 : y, value);
      });
    }

    this.font.print(0, (LINE_COUNT * OFFSET) + OFFSET, "- SQUARE: Delete | CIRCLE: Back | CROSS: Select | START: Submit");
  }

  private control(): void {
    if (this.pad.justPressed(Pads.START)) {
      this.handler(VALUES[this.selected], this);
    }

    if (this.pad.justPressed(Pads.CROSS)) {
      this.buffer += VALUES[this.selected];
    }

    if (this.pad.justPressed(Pads.SQUARE)) {
      this.buffer = this.buffer.slice(0, this.buffer.length - 1);
    }

    if (this.pad.justPressed(Pads.CIRCLE)) {
      this.parent.set(this.back)
    }

    if(this.pad.justPressed(Pads.UP)){
      this.selected -= LINE_SIZE;
      if (this.selected <= 0) {
        this.selected = 0;
        return;
      }
    }

    if(this.pad.justPressed(Pads.LEFT)){
      if (this.selected <= 0) return;
      this.selected -= 1;
    }

    if(this.pad.justPressed(Pads.DOWN)){
      this.selected += LINE_SIZE;
      if (this.selected >= VALUES.length - 1) {
        this.selected = VALUES.length - 1;
        return;
      }
    }

    if(this.pad.justPressed(Pads.RIGHT)){
      if (this.selected >= VALUES.length - 1) return;
      this.selected += 1;
    }
  }
}
