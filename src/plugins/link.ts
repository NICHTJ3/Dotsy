import { existsSync } from 'fs';
import fs from 'fs/promises';

export default async function link(from: string, to: string) {
  // TODO: Should I check they exist? check dotbot for more info
  // TODO: Should it be a symlink of a hard link
}
