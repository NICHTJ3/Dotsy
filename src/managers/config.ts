import { existsSync } from 'fs';
import fs from 'fs/promises';
import {
  configCommandOptions,
  ConfigCommandOptionsType,
} from '../validators/config-args';

const optsToAction = {
  list,
  install,
  uninstall,
};

export function handle(opts: ConfigCommandOptionsType) {
  for (const opt of Object.keys(opts)) {
    if (opt in optsToAction) {
      // TODO: Fix typing
      const action = (optsToAction as any)[opt];
      if (opt in opts) {
        action((opts as any)[opt]);
      }
    }
  }
}

export function validate(cmdObj: unknown) {
  return configCommandOptions.parse(cmdObj); // Validate command options
}

export function install(name: string[]) {
  console.log('Config to install', name);
}

export function uninstall(name: string) {
  console.log('Config to uninstall', name);
}

export async function list() {
  // TODO: Throw error correctly
  // TODO: Use variable from .dotsyrc
  const profilesExist = existsSync('./profiles');
  if (profilesExist) {
    const dir = await fs.readdir('./profiles');
    console.log(dir);
  } else {
    console.error('No profiles found');
  }
}
