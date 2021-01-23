export default abstract class Plugin {
  context: any;
  constructor(context: unknown) {
    this.context = context;
  }

  abstract canHandle(directive: string): boolean;
  abstract handle(directive: string, data: unknown): void;
}
