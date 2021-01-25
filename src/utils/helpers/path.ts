import path from 'path';
import os from 'os';

// TODO: Make this work on windows
export function getAbsolutePath(filepath: string, delim = '/') {
  const homedir = os.homedir();
  filepath = filepath.replace(/\~/g, homedir + delim);
  return path.resolve(filepath);
}

export function getParentDirectory(filepath: string) {
  return path.basename(path.dirname(filepath));
}

export { path };
