import fs from 'fs/promises';
import { existsSync } from 'fs';
import { globalConfig, GlobalConfigType } from './validator';

class Dotsyrc {
  // TODO: Also check for dotsyrc in ~/.config etc
  private static config: GlobalConfigType | undefined;
  async getConfig() {
    if (existsSync('./.dotsyrc') && !Dotsyrc.config) {
      const data = await fs.readFile('./.dotsyrc', 'utf8');
      if (data) Dotsyrc.config = globalConfig.parse(JSON.parse(data));
    }
    return Dotsyrc.config;
  }
}

export default new Dotsyrc();
