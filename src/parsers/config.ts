import fs from 'fs/promises';
import { config } from '../utils/validators/config';

export default class ConfigReader {
  configPath: string;
  constructor(configPath: string) {
    this.configPath = configPath;
  }

  async getConfig() {
    try {
      const data = await fs.readFile(this.configPath, 'utf8');
      return config.parse(JSON.parse(data));
    } catch (e) {
      throw `Could not read config file: ${this.configPath}
        ${e}
      `;
    }
  }
}
