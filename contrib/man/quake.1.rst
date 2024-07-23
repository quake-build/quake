=====
quake
=====

NAME
====

quake — A powerful and expressive build tool powered by Nushell

USAGE
=====

**quake** [*OPTIONS*] <*TASK*> -- [*TASK_ARGS*] — Run a task (and its
dependencies) with separate task arguments.

**quake list** [*OPTIONS*] — List all available tasks. See
**quake-list**\ (1).

**quake inspect** [*OPTIONS*] — Dump JSON-encoded metadata about the
current project. See **quake-inspect**\ (1).

DESCRIPTION
===========

quake is a general-purpose, language-agnostic, and multi-paradigmatic
build tool.

Its build scripts, located at the root of projects and named
``build.quake``, are written in an extension of Nushell that adds extra
syntax to the language in addition to a declarative DSL.

This manual only describes basic invocation of quake. For much more
information, view the documentation referenced in **Documentation**.

OPTIONS
=======

General
-------

**--help**
    Print the help message for the program or a subcommand.

**--project** <*PROJECT_DIR*>
    Path to the project root directory.

    This will set the current working directory for the build script.

    By default, the project root will be autodetected if possible by,
  in order:
    - Detecting the presence of a ``build.quake`` file in the current
  directory.
    - Using version control tools or files to

**--build-script** <*SCRIPT_FILE*>
    Evaluate a specific build script, keeping the working directory
  the same as the current.

Output Handling
---------------

**--json**
    Output events as a line-delimited JSON objects to stderr.
    The format for all JSON types is formally defined in an appendix
  of the quake book (see **Documentation**).

**--log**
    Log output to ``quake.log``.

**--log-format** *[]*

**--quiet**
    Do not print output from executed commands insides tasks.

    Status updates will still be printed.

Operation
---------

**-D**, **--assume-dirty**
    Execute tasks regardless of initial task dirtiness checks.

    When combined with **--watch**, this only skips dirtiness checks
  for the initial build cycle.

**--watch**
    Run the task and its dependencies, initiating re-runs of tasks as
  appropriate when files are modified.

**-j**, **--jobs** **
    Specify the maximum number of threads

WEBSITE
=======

quake's homepage can be found at https://quake.build.

To learn more about Nushell, visit https://nushell.sh.

Documentation
-------------

Extended documentation, including the aforementioned appendices as well
as a full guide, can be found in book format at
https://docs.quake.build.

Depending on your distribution, this documentation may also be available
in ``/usr/share/docs/quake/``.

SOURCES
=======

quake's source code is located at https://git.sr.ht/~cassaundra/quake.

quake is licensed under GPL-3.0 or later.

SEE ALSO
========

**build.quake**\ (5), **quake.toml**\ (5), **quake-list**\ (5),
**quake-inspect**\ (5)
