import fs from 'fs/promises';
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
  private link(from: string, to: string) {
    let success = false;
    // TODO: Get the relative path to the from file?
    // TODO: Should the path be taken from an absolute
    // TODO: Get default options from config parser?

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
  private async isLink(file: string): Promise<boolean> {
    const stats = await fs.lstat(file);
    return stats.isSymbolicLink();
  }
}
