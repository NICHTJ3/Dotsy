import fs from 'fs';
import { getAbsolutePath } from './utils/helpers/path';
import Context from './utils/plugins/context';
// TODO: Create seporate class to dispatch tasks for a profile
// TODO: Once the above is done rename this to config dispatcher

export type Task = {}[];

export default abstract class Dispatcher {
  context: Context;
  only: Array<string> | null;
  skip: Array<string> | null;

  constructor(baseDirectory: string, only = null, skip = null) {
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
  //TODO: Actually iterate over keys
  //TODO: Handle default action
  //TODO: Use plugins to process actions
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
  loadPlugins() {
    return null;
  }
}
