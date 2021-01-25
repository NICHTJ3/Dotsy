import fs from 'fs/promises';
import { getAbsolutePath, getParentDirectory } from 'src/utils/helpers/path';
import Context from '../utils/plugins/context';
import Plugin from '../utils/plugins/plugin';

export default class Link extends Plugin {
  directive = 'link';

  constructor(context: Context) {
    super(context);
  }

  handle(directive: string, data: unknown) {
    if (directive !== this.directive) {
      throw `The Link plugin cannot handle directive ${directive}`;
    }
    this.processLinks(data as { from: string; to: string }[]);
  }

  /**
   * Checks if a given directive can be handled by this plugin
   *
   * @param {string} directive: The directive to check
   * @returns {boolean}
   * @memberof Link
   */
  canHandle(directive: string): boolean {
    return directive === this.directive;
  }

  /**
   * Process all links supplied as data to the directive into symbolic links
   *
   * @private
   * @param {{ from: string; to: string }[]} links: The data supplied to the directive
   * @memberof Link
   */
  //TODO: Should this return success or failure
  private processLinks(links: { from: string; to: string }[]) {
    for (const link of links) {
      this.link(link.from, link.to);
    }
  }

  /**
   * Links from to to as a symbolic link
   *
   * @private
   * @param {string} from: The file to link
   * @param {string} to: The directory or file to link to
   * @memberof Link
   */
  private async link(from: string, to: string) {
    let success = true;
    // TODO: Get the absolute path to the from file?
    // TODO: Get default options from config parser?

    // TODO: Use this only if create flag is passed
    // Creates all parent directories
    // const parent = getParentDirectory(to)!;
    // await fs.mkdir(getAbsolutePath(parent), { recursive: true });

    fs.symlink(from, to);
    return success;
  }

  /**
   * Checks if a given file is already a link
   *
   * @private
   * @param {string} file: File to check if it is a link
   * @returns {boolean}
   * @memberof Link
   */
    // TODO: Use this
  private async isLink(file: string): Promise<boolean> {
    const stats = await fs.lstat(file);
    return stats.isSymbolicLink();
  }
}
