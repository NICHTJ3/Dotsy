import { PathLike } from 'fs';
import fs from 'fs/promises';

export default class Context {
  baseDirectory: string | null;
  defaults: any;

  constructor(baseDirectory?: string) {
    this.baseDirectory = baseDirectory ?? null;
    this.defaults = {};
  }

  setBaseDirectory(baseDirectory: string) {
    this.baseDirectory = baseDirectory;
  }

  async getBaseDirectory(canonicalPath = true) {
    let baseDirectory = this.baseDirectory;
    if (canonicalPath) {
      baseDirectory = await fs.realpath(baseDirectory as PathLike);
    }
    return baseDirectory;
  }

  setDefaults(defaults: unknown) {
    this.defaults = defaults;
  }

  getDefaults() {
    return { ...this.defaults };
  }
}
