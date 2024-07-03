# ps-tree.rs

[![ci](https://github.com/axetroy/ps-tree.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/axetroy/ps-tree.rs/actions/workflows/ci.yml)
[![ci](https://github.com/axetroy/ps-tree.rs/actions/workflows/build.yml/badge.svg)](https://github.com/axetroy/ps-tree.rs/actions/workflows/build.yml)
![Latest Version](https://img.shields.io/github/v/release/axetroy/ps-tree.rs.svg)
![License](https://img.shields.io/github/license/axetroy/ps-tree.rs.svg)
![Repo Size](https://img.shields.io/github/repo-size/axetroy/ps-tree.rs.svg)

A command line tool for monitoring the CPU and memory usage of a process tree.

## Usage

Download the binary from the [release page](https://github.com/axetroy/ps-tree.rs/releases) and run the following command

```sh
$ ps-tree <pid>
```

### Usage Example

Of course, you can execute this command in another language and parse the results from stdout.

Here we take nodejs as an example.

```js
const { spawn } = require("child_process");
const readline = require("readline");

const ps = spawn("target/debug/ps-tree", [process.pid], { stdio: "pipe" });

// create a readline interface to read from stdout
const rl = readline.createInterface({
  input: ps.stdout,
  output: process.stdout,
  terminal: false,
});

rl.on("line", (line) => {
  try {
    const json = JSON.parse(line);
    console.log("Received JSON:", json);
  } catch (error) {
    console.error("Error parsing JSON:", error);
  }
});

ps.on("close", (code) => {
  console.log(`child process exited with code ${code}`);
});
```

It will output information(JSON format) about the process and its subprocesses.

This is the output JSON field information.

```rs
#[derive(Debug, Clone, Serialize)]
pub struct ProcessNode {
    pid: SerializablePid,
    ppid: SerializablePid,
    gid: Option<u32>,
    name: String,
    cmd: Vec<String>,
    cpu_usage: f32,
    memory: u64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<ProcessNode>,
}
```

<details>

  <summary>See the output example(Pretty)</summary>

```json
{
  "pid": 90456,
  "ppid": 1,
  "name": "firefox",
  "cmd": [
    "/Applications/Firefox Developer Edition.app/Contents/MacOS/firefox",
    "-foreground"
  ],
  "cpu_usage": 55.335175,
  "memory": 630439936,
  "children": [
    {
      "pid": 3986,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-childID",
        "179",
        "-isForBrowser",
        "-prefsLen",
        "39653",
        "-prefMapSize",
        "260845",
        "-jsInitLen",
        "234780",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-sbLevel",
        "3",
        "-parentBuildID",
        "20240628091536",
        "-greomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/omni.ja",
        "-appomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser/omni.ja",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{8cfb7fab-033b-4f16-a095-314c8750995b}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.1820100416",
        "tab"
      ],
      "cpu_usage": 0.037370622,
      "memory": 122142720,
      "children": []
    },
    {
      "pid": 1177,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-childID",
        "167",
        "-isForBrowser",
        "-prefsLen",
        "39653",
        "-prefMapSize",
        "260845",
        "-jsInitLen",
        "234780",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-sbLevel",
        "3",
        "-parentBuildID",
        "20240628091536",
        "-greomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/omni.ja",
        "-appomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser/omni.ja",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{d17b9d18-aeaa-4023-9b5b-30209926975e}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.525245542",
        "tab"
      ],
      "cpu_usage": 0.035160605,
      "memory": 84049920,
      "children": []
    },
    {
      "pid": 90768,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-childID",
        "38",
        "-isForBrowser",
        "-prefsLen",
        "37436",
        "-prefMapSize",
        "260845",
        "-jsInitLen",
        "234780",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-sbLevel",
        "3",
        "-parentBuildID",
        "20240628091536",
        "-greomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/omni.ja",
        "-appomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser/omni.ja",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{fda829c2-98ab-47ab-91bf-6041cab70e7d}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.138077800",
        "tab"
      ],
      "cpu_usage": 0.026068395,
      "memory": 84328448,
      "children": []
    },
    {
      "pid": 18917,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-childID",
        "368",
        "-isForBrowser",
        "-prefsLen",
        "39666",
        "-prefMapSize",
        "260845",
        "-jsInitLen",
        "234780",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-sbLevel",
        "3",
        "-parentBuildID",
        "20240628091536",
        "-greomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/omni.ja",
        "-appomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser/omni.ja",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{caeca1f5-eb0b-4958-91c2-354275d3b4dd}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.1666781175",
        "tab"
      ],
      "cpu_usage": 0.053164806,
      "memory": 102809600,
      "children": []
    },
    {
      "pid": 90477,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-parentBuildID",
        "20240628091536",
        "-sandboxingKind",
        "0",
        "-prefsLen",
        "42764",
        "-prefMapSize",
        "260845",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{91974483-30ae-455d-8999-490faf0a4f34}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.1240446945",
        "utility"
      ],
      "cpu_usage": 0.62631416,
      "memory": 23003136,
      "children": []
    },
    {
      "pid": 90487,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-parentBuildID",
        "20240628091536",
        "-prefsLen",
        "42773",
        "-prefMapSize",
        "260845",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{baeba8de-89c7-404a-bbe6-20ac90755982}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.677020745",
        "rdd"
      ],
      "cpu_usage": 1.362366,
      "memory": 27574272,
      "children": []
    },
    {
      "pid": 90468,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-childID",
        "2",
        "-isForBrowser",
        "-prefsLen",
        "38005",
        "-prefMapSize",
        "260845",
        "-jsInitLen",
        "234780",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-sbLevel",
        "3",
        "-parentBuildID",
        "20240628091536",
        "-greomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/omni.ja",
        "-appomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser/omni.ja",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{4d25d7af-704b-498d-a5bf-87b3c8382c20}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.1459792234",
        "tab"
      ],
      "cpu_usage": 0.0662963,
      "memory": 76496896,
      "children": []
    },
    {
      "pid": 90467,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-childID",
        "1",
        "-isForBrowser",
        "-prefsLen",
        "37045",
        "-prefMapSize",
        "260845",
        "-jsInitLen",
        "234780",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-sbLevel",
        "3",
        "-parentBuildID",
        "20240628091536",
        "-greomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/omni.ja",
        "-appomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser/omni.ja",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{900ab35b-4440-4bb6-85df-22db4cb2a5d1}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.1201641736",
        "tab"
      ],
      "cpu_usage": 0.7343334,
      "memory": 422739968,
      "children": []
    },
    {
      "pid": 19554,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-childID",
        "383",
        "-isForBrowser",
        "-prefsLen",
        "39709",
        "-prefMapSize",
        "260845",
        "-jsInitLen",
        "234780",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-sbLevel",
        "3",
        "-parentBuildID",
        "20240628091536",
        "-greomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/omni.ja",
        "-appomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser/omni.ja",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{9d7a854f-dcc8-4d18-989e-5d9183a9ab39}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.1480076382",
        "tab"
      ],
      "cpu_usage": 0.028861377,
      "memory": 100073472,
      "children": []
    },
    {
      "pid": 90488,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-parentBuildID",
        "20240628091536",
        "-sandboxingKind",
        "1",
        "-prefsLen",
        "42773",
        "-prefMapSize",
        "260845",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{6d897372-8670-4780-b498-17e437710865}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.1747653525",
        "utility"
      ],
      "cpu_usage": 0.5848247,
      "memory": 20938752,
      "children": []
    },
    {
      "pid": 90676,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-childID",
        "30",
        "-isForBrowser",
        "-prefsLen",
        "37436",
        "-prefMapSize",
        "260845",
        "-jsInitLen",
        "234780",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-sbLevel",
        "3",
        "-parentBuildID",
        "20240628091536",
        "-greomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/omni.ja",
        "-appomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser/omni.ja",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{29ce4454-1afe-467e-bfa6-e7cba4e8170d}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.1312829391",
        "tab"
      ],
      "cpu_usage": 0.07853348,
      "memory": 88260608,
      "children": []
    },
    {
      "pid": 28223,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-childID",
        "388",
        "-isForBrowser",
        "-prefsLen",
        "39709",
        "-prefMapSize",
        "260845",
        "-jsInitLen",
        "234780",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-sbLevel",
        "3",
        "-parentBuildID",
        "20240628091536",
        "-greomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/omni.ja",
        "-appomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser/omni.ja",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{9d88f704-9a6d-4ce6-b937-df483965c9c6}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.82165909",
        "tab"
      ],
      "cpu_usage": 0.0057585686,
      "memory": 44548096,
      "children": []
    },
    {
      "pid": 18914,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-childID",
        "367",
        "-isForBrowser",
        "-prefsLen",
        "39666",
        "-prefMapSize",
        "260845",
        "-jsInitLen",
        "234780",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-sbLevel",
        "3",
        "-parentBuildID",
        "20240628091536",
        "-greomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/omni.ja",
        "-appomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser/omni.ja",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{fa7b927c-db99-4a2c-8a60-f17d14595e03}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.813284007",
        "tab"
      ],
      "cpu_usage": 49.548267,
      "memory": 779223040,
      "children": []
    },
    {
      "pid": 25661,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-childID",
        "386",
        "-isForBrowser",
        "-prefsLen",
        "39709",
        "-prefMapSize",
        "260845",
        "-jsInitLen",
        "234780",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-sbLevel",
        "3",
        "-parentBuildID",
        "20240628091536",
        "-greomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/omni.ja",
        "-appomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser/omni.ja",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{b4c0d807-b6b8-4ee5-b666-9de1ec78b73a}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.1847863566",
        "tab"
      ],
      "cpu_usage": 0.0,
      "memory": 44875776,
      "children": []
    },
    {
      "pid": 965,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-childID",
        "162",
        "-isForBrowser",
        "-prefsLen",
        "39653",
        "-prefMapSize",
        "260845",
        "-jsInitLen",
        "234780",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-sbLevel",
        "3",
        "-parentBuildID",
        "20240628091536",
        "-greomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/omni.ja",
        "-appomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser/omni.ja",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{ff312d05-9914-490c-a11f-214d166968ca}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.593999603",
        "tab"
      ],
      "cpu_usage": 0.029250585,
      "memory": 207863808,
      "children": []
    },
    {
      "pid": 25662,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-childID",
        "387",
        "-isForBrowser",
        "-prefsLen",
        "39709",
        "-prefMapSize",
        "260845",
        "-jsInitLen",
        "234780",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-sbLevel",
        "3",
        "-parentBuildID",
        "20240628091536",
        "-greomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/omni.ja",
        "-appomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser/omni.ja",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{3edc273d-c890-49ba-a848-34b63a38a990}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.348803789",
        "tab"
      ],
      "cpu_usage": 0.0,
      "memory": 44875776,
      "children": []
    },
    {
      "pid": 90466,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-parentBuildID",
        "20240628091536",
        "-prefsLen",
        "36879",
        "-prefMapSize",
        "260845",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{5d7ffcb8-a4b2-4e04-a484-db2a31cab739}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.2112256667",
        "socket"
      ],
      "cpu_usage": 0.0,
      "memory": 14827520,
      "children": []
    },
    {
      "pid": 22804,
      "ppid": 90456,
      "name": "plugin-container",
      "cmd": [
        "/Applications/Firefox Developer Edition.app/Contents/MacOS/plugin-container.app/Contents/MacOS/plugin-container",
        "-childID",
        "385",
        "-isForBrowser",
        "-prefsLen",
        "39709",
        "-prefMapSize",
        "260845",
        "-jsInitLen",
        "234780",
        "-sbStartup",
        "-sbAppPath",
        "/Applications/Firefox Developer Edition.app",
        "-sbLevel",
        "3",
        "-parentBuildID",
        "20240628091536",
        "-greomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/omni.ja",
        "-appomni",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser/omni.ja",
        "-appDir",
        "/Applications/Firefox Developer Edition.app/Contents/Resources/browser",
        "-profile",
        "/Users/axetroy/Library/Application Support/Firefox/Profiles/9v87tgkr.dev-edition-default",
        "{0cd4c2c4-2a1b-43ae-8345-3c1e306714cd}",
        "90456",
        "gecko-crash-server-pipe.90456",
        "org.mozilla.machname.162325247",
        "tab"
      ],
      "cpu_usage": 1.0462717,
      "memory": 180043776,
      "children": []
    }
  ]
}
```

</details>

### Help information

```bash
$ ps-tree --help
Usage: ps-tree <PID> [Options]
Options:
  <PID>                Process ID to monitor.
  --help,-h            Print the help information
  --version,-V         Print the version information
  --interval SECONDS   Time interval between updates in seconds.
Description:
  This tool monitors the specified process and refreshes the information
  every SECONDS seconds, as specified by the --interval option.
Source Code:
  https://github.com/axetroy/ps-tree.rs
```


## LICENSE

The [MIT License](LICENSE)
