import { existsSync } from 'fs';
import fs from 'fs/promises';

export default async function listDir(dir: string) {
  const profilesExist = existsSync(dir);
  if (profilesExist) {
    return await fs.readdir(dir);
  }
  throw 'Directory did not exist';
}
