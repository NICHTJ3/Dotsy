import { Command } from 'commander';
import * as ConfigManager from './config';

const program = new Command();
program.version('0.0.1');

program
  .command('config')
  .option('-ls,--list', 'List available configs')
  .option('-i,--install [configs...]', 'Install a config')
  .option('-u,--uninstall [configs...]', 'Uninstall a config')
  .action(function (cmdObj) {
    ConfigManager.handle(ConfigManager.validate(cmdObj));
  });

program.parse(process.argv);
