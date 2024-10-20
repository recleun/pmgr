# pmgr
[![Rust](https://github.com/recleun/pmgr/actions/workflows/rust.yml/badge.svg)](https://github.com/recleun/pmgr/actions/workflows/rust.yml)

A simple to use CLI project manager written in rust.

## Usage
Note: This documentation is only temporary, hopefully it will be improved some time soon. For now, using `pmgr help` should suffice.

Usage is pretty simple and straight forward, as this is one of pmgr's goals.
The main goal of this is to make it easy to gather your thoughts and give you a solid idea of what you need to do without staring at your monitor for minutes.

To start using pmgr, you need to be in the root directory of a project you want to manage, then use the command `pmgr init` to initialize a project in the directory.
After that, you can start adding groups, and sub-groups, and sub-sub-groups, as many as you wish. You can do so by using the command `pmgr create <group_name> [parent_group]`.
Parent group is optional here of course.

Groups without any form of data is useless, so you might want to add notes or tasks to groups, you can do so with the command `pmgr add task <group_name> <task_details>`.
Or add notes with the command `pmgr add note <group_name> <note_details>` instead.

What if you need to set a task as complete/incomplete? You can use `pmgr task <complete|undo> <group_name> <task_id>`.
You can get the idea by using the command `pmgr list [group_name]`. It will automatically list watched groups, and to set them: `pmgr watch <group_name>`.
This command makes a group and its descendants "watched". `list` command automatically lists watched groups when you don't specify a `group_name`.

Added a group by mistake? `pmgr delete <group_name>`, but be careful it deletes descendant groups too.
Added a note/task by mistake? `pmgr remove <task|note> <group_name> <id>`.
