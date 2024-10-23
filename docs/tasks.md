# Tasks
This should give you a basic understanding for using tasks in pmgr.

## Why?
The more you use tasks, the more you find them useful.
You can use them before you start coding a feature or a bug fix,
making you know exactly what you need to be done with less pausing in the middle.
They can also be used as reminders, for when you start your next development session.

## Usage

### Adding tasks
This command will add a task to the specified group with the specified task description:
```
$ pmgr add task <GROUP_NAME> <TASK>
```

### Removing Tasks
This command will remove a task from the specified group with the specified task ID:
```
$ pmgr remove task <GROUP_NAME> <TASK_ID>
```

You can also specify multiple task IDs, like:
```
$ pmgr remove task my-group 1 2 3 4
```

Task IDs can be found using the `pmgr list` command.

### Completing Tasks
You can set a task as complete using the following command:
```
$ pmgr task complete <GROUP_NAME> <TASK_ID>
```

You can specify multiple task IDs just like removing tasks.

### Undoing Tasks
You might have set a task as complete by accident, or you want to set it as incomplete.
```
$ pmgr task undo <GROUP_NAME> <TASK_ID>
```

Task ID can be multiple IDs here, too.

### Viewing Progress
You can view the progress of tasks in a "progress-bar" way.
```
$ pmgr task progress [GROUP_NAME]
```

`GROUP_NAME` here is optional.
