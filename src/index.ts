import { Command } from 'commander';

const program = new Command();
program.version('0.0.1');

program
  .command('profile [subcommand]')
  .option('-i,--install', 'Install a profile')
  .action(function (subcommand: string | undefined, cmdObj) {
    console.log(subcommand + (cmdObj.install ? ' install' : 'not install'));
  });

program.parse(process.argv);
