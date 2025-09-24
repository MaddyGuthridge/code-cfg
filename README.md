# code-cfg

Configure VS Code with a simple CLI. Combine settings snippets, and apply them
easily.

## Why

There is no one-size-fits-all option for configuring your editor, but finding
the right set of settings is a tedious process, especially for beginners.
`code-cfg` allows users to run a simple command and set up their editor by
picking and choosing from desired settings provided by others. Each
"settings set" contains a bunch of operations which can be applied to the
user's settings, with the CLI providing the ability to preview options, as well
as picking which options get applied.

## Use cases

* Letting beginners configure VS Code to their liking without needing to hunt
  through thousands of options.
* Letting people easily apply patches to their settings with a single command,
  for example, disabling the incessant AI slop which Microsoft adds with every
  update.

## Planned features

* [ ] Download JSONC settings snippets and apply them using a simple CLI
* [ ] Be able to un-apply settings by using a special syntax in the keys
* [ ] Live-preview settings before saving them (writes settings, but undoes
      changes when the application quits, using a backup copy).
* [ ] Provide a settings "snippet set" file format which allows users to apply
      settings in batches, using an interactive CLI.
* [ ] Provide a simple CLI editor for settings to allow them to be customised
      before they are applied.
* [ ] Provide a "settings snippet editor" which allows users to create snippet
      sets more easily.
* [ ] Allow for "settings subscriptions" where you can subscribe to a setting,
      meaning it can be easily updated with a single command.
