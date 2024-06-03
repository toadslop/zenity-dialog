# Zenity-Dialog

A thin wrapper arround Zenity, a tool for rendering dialog boxes in Linux.
This mvp version supports only a limitted number of Zenity options.

Note: This library assumes that Zenity is already installed and will throw an error
if it isn't found. You will need to provide appropriate documentation in your application
to convey the need to install Zenity.

## When Would I Use This?

Imagine you're building some kind of application that needs to get some input from the
user under some circumstances, but for one reason or another the user can't expect to
be able to see stdout. One case might be a background process that the user launched
without sending stdout to the terminal; another might be a CLI tool that is invoked by
a desktop application, perhaps as part of a plugin. This library allows you to display
something that the user will definitely be able to see and respond to.

## Usage
```rust
let result = ZenityDialog::new(dialog::Error::default().with_text("An error happened!"))
    .with_icon(Icon::Error)
    .show()?;

    match result {
        ZenityOutput::Affirmed { .. } => {
            println!("The user clicked the affirmative response")
        }
        ZenityOutput::Rejected { .. } => println!("The user clicked the rejection response"),
        ZenityOutput::Unknown {
            exit_code,
            stdout,
            stderr,
        } => println!("Something weird happened. {exit_code} {stdout} {stderr}"),
    };
```
## Features

### Chrono

Enable automatic date parsing for the Calendar type using Chrono. When this feature is
enabled, you won't be able to pass custom date formats to Zenity as this can interfere
with Chrono's ability to properly parse the date.