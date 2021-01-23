import fs from 'fs';
import { getAbsolutePath } from './utils/helpers/path';
import Context from './utils/plugins/context';
// TODO: Create seporate class to dispatch tasks for a config/profile

export type Task = {}[];

export default abstract class Dispatcher {
  context: Context;
  only: Array<string> | null;
  skip: Array<string> | null;
  constructor(base_directory: string, only = null, skip = null) {
    this.setupContext(baseDirectory);
    this.loadPlugins();
    this.only = only;
    this.skip = skip;
  }

  setupContext(baseDirectory: string) {
    const path = getAbsolutePath(baseDirectory);
    if (!fs.existsSync(path)) {
      throw 'Nonexistent base directory';
    }
    this.context = new Context(path);
  }

  //TODO: Types
  //TODO actually iterate over keys
  //TODO handle default action
  //TODO use plugins to process actions
  dispatch(tasks: Map<string, string>) {
    let success = true;
    for (const task of tasks) {
      for (const action of task) {
        if (
          (!this.only?.includes(action) || this.skip?.includes(action)) &&
          action != 'defaults'
        ) {
          console.log(`Skipping action ${action}`);
          continue;
        }
      }
    }

    return success;
  }

  // TODO: Create plugin dictionary
}
