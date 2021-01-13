import { Command } from 'commander';
import * as ProfileManager from './managers/profile';
import * as ConfigManager from './managers/config';

const program = new Command();
program.version('0.0.1');

program
  .command('profile')
  .option('-ls,--list', 'List available profiles')
  .option('-i,--install [profiles...]', 'Install a profile')
  .option('-u,--uninstall [profiles...]', 'Uninstall a profile')
  .action(function (cmdObj) {
    ProfileManager.handle(ProfileManager.validate(cmdObj));
  });

program
  .command('config')
  .option('-ls,--list', 'List available configs')
  .option('-i,--install [configs...]', 'Install a config')
  .option('-u,--uninstall [configs...]', 'Uninstall a config')
  .action(function (cmdObj) {
    ConfigManager.handle(ConfigManager.validate(cmdObj));
  });

program.parse(process.argv);
