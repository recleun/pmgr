# Usage
One of pmgr's goals is simplicity;
If it's easy to use, no one will have to worry about learning how to use it.
Though, there are a few commands that you need to know, but when you do, you'll use pmgr like a master.

Note: You can skip reading this by using the command `pmgr help`,
but it's still recommended to read this.

## Projects
The root of pmgr is projects, without a project, there's no pmgr.
Projects work very much like git, you initialize it in a project, and you're done.
A project will contain everything managed by pmgr.

To initiate a project in the current directory:
```
$ pmgr init
```

#### Notes
- You can check if a project already exists or not with the command:
```
$ pmgr check
```

## Groups

### Creating groups
pmgr is built on groups, consider them like git branches, but nestable.
Groups can contain specific types of data, like `note` and `task`, and even groups.

To create a group:
```
$ pmgr create my-group
```

This will create a group named `my-group` in the root of the project.

You can create sub-groups with the following command:
```
$ pmgr create sub-group my-group
```

This will create a group named `sub-group`, and it will be a child of `my-group`.

#### Notes
- You can name the groups anything as long as it's a string.
- If it's one word, you can omit the quotes, like: `my_group`
- If it's multiple words, use quotes, like: `"this is my group"`

### Deleting groups
Deleting a group will delete its descendants automatically.

To delete a group: 
```
$ pmgr delete my-group
```

This will delete the group named `my-group` and all of it's descendants.

#### Notes
- You can specify multiple groups to delete, like:
```
$ pmgr delete my-group1 my-group2
```

### Watching groups
Groups can be either watched or unwatched.
It's useful to have the groups you are working on set as watched,
because you will get useful features like listing its descendants as well using commands faster.
Consider it like checking out a git branch.

To watch a group:
```
$ pmgr watch my-group
```

This will watch the group named `my-group` as well as it's descendants.

To unwatch a group:
```
$ pmgr unwatch my-group
```

This will do the opposite of the watch command, basically unwatching the group and it's descendants.

#### Notes
- You can choose multiple groups to watch/unwatch, like so:
```
$ pmgr watch my-group1 my-group2
```

### Listing groups
There are 3 ways of listing groups:

1. Listing a specific group.
2. Listing all groups.
3. Listing watched groups.

#### Specific groups

This will list the group named `my-group`:
```
$ pmgr list my-group
```

#### All groups

This will list all groups in the project:
```
$ pmgr list --all
```

or:
```
$ pmgr list -a
```

#### Watched groups

This will list only the watched groups:
```
$ pmgr list
```

## Adding & Removing Data
Groups without any data are pointless,
making the group contain zero information of what it's about.

There are some types of data that you can use, some of them are notes and tasks:
- Note: A piece of text, can be like a reminder, or something to consider for the group.
- Task: A field like a checkbox, you can set it as complete/incomplete, it also contains text about the task itself. More on tasks can be found [here](tasks.md).

### Adding Data

To add a note:
```
$ pmgr add note my-group "This is a note that I need to remember"
```

This adds a note to `my-group` with the specified string as it's content.

To add a task:

```
$ pmgr add task my-group "Finish reading pmgr's documentation"
```

This adds a task to `my-group` with the specified string as it's content.

#### Notes
- Quotes aren't necessary and can be ommitted when you specify the string for notes and tasks.

### Removing Data

To remove a note:
```
$ pmgr remove note my-group 1
```

This removes a note with ID 1 from the group `my-group`

To remove a task:
```
$ pmgr remove task my-group 1
```

This removes a task with ID 1 from the group `my-group`

#### Notes
- You can view the IDs of items by listing the group.
- You can specify multiple IDs when removing, like:
```
$ pmgr remove note my-group 1 2 3
```
