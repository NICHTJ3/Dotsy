import Context from './context';

export default abstract class Plugin {
  context: any;
  directive: string;

  constructor(context: Context) {
    this.context = context;
  }

  /**
   * Checks if a directive can be handled by this plugin
   *
   * @abstract
   * @param {string} directive: The directive to check
   * @returns  {boolean}
   * @memberof Plugin
   */
  abstract canHandle(directive: string): boolean;

  /**
   * Process the given directive with the given data
   * __Note:__ If the directive cannot be handled an error may be thrown
   *
   * @abstract
   * @param {string} directive: The directive to try process
   * @param {unknown} data: The data to process
   * @memberof Plugin
   */
  abstract handle(directive: string, data: unknown): void;
}
