pub fn usage() -> & 'static str{
    // let s: &'static str = r#"Usage:
    r#"Usage:
  rcheat [options] [<cheatsheet>]

Options:
  --init                  Write a default config file to stdout
  -a --all                Search among all cheatpaths
  -c --colorize           Colorize output
  -d --directories        List cheatsheet directories
  -e --edit=<cheatsheet>  Edit <cheatsheet>
  -l --list               List cheatsheets
  -p --path=<name>        Return only sheets found on cheatpath <name>
  -r --regex              Treat search <phrase> as a regex
  -s --search=<phrase>    Search cheatsheets for <phrase>
  -t --tag=<tag>          Return only sheets matching <tag>
  -T --tags               List all tags in use
  -v --version            Print the version number
  --rm=<cheatsheet>       Remove (delete) <cheatsheet>
  --conf                  Display the config file path

Examples:

  To initialize a config file:
    mkdir -p ~/.config/cheat && cheat --init > ~/.config/cheat/conf.yml

  To view the tar cheatsheet:
    cheat tar

  To edit (or create) the foo cheatsheet:
    cheat -e foo

  To edit (or create) the foo/bar cheatsheet on the "work" cheatpath:
    cheat -p work -e foo/bar

  To view all cheatsheet directories:
    cheat -d

  To list all available cheatsheets:
    cheat -l

  To list all cheatsheets whose titles match "apt":
    cheat -l apt

  To list all tags in use:
    cheat -T

  To list available cheatsheets that are tagged as "personal":
    cheat -l -t personal

  To search for "ssh" among all cheatsheets, and colorize matches:
    cheat -c -s ssh

  To search (by regex) for cheatsheets that contain an IP address:
    cheat -c -r -s '(?:[0-9]{1,3}\.){3}[0-9]{1,3}'

  To remove (delete) the foo/bar cheatsheet:
    cheat --rm foo/bar

  To view the configuration file path:
    cheat --conf
"#

}

