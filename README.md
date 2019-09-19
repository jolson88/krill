# Krill
A task organizer inspired by bullet journaling. It aims to be used primarily through a command-line interface, but all information is persisted in a data format that is easy to read and edit directly. This allows users to edit Krill files with their own editor, on different devices, or however they wish.

It takes inspiration from other technologies like Org mode in Emacs, but aims for more radical simplicity and enabling a larger variety of user experiences.

# Goals
Krill supports the following functionality:
- Future Log
- Monthly planning
    - Milestones (including recurring events)
    - Monthly task log
    - Weekly goals
    - Daily task planning
- User-defined task categories
- User-defined task statuses
- Task notes
- Migration of tasks from day-to-day and month-to-month

Krill will aim to support many different exports, allowing users to easily export to different file formats like markdown, JSON, YAML, etc.

# Example persisted file format
Krill's file format takes inspiration from homoiconic languages like Lisp, Forth, and others. It only has four primitives: Lists, Atoms, Integers, and Strings.

The file format aims to be easily parsable while also being able to be easily edited directly by users if necessary.

```
(September
    (Milestones
        (Every Friday "Game time with son")
        (Every Saturday "Game time with daughter")
    )
    (Tasks
        (NotStarted (Personal) "Fix dishwasher")
        (InProgress (Personal) "Ship new feature X in Project A")
        (NotStarted (Work) "Present quarterly numbers at all-hands")
        (Finished (Work) "Identify observability gaps in web service")
    )
    (Week
        (Goals
            (NotStarted (Work) "Fix partner bug #1120")
            (Finished (Work) "Approve outstanding expense reports")
            (Finished (Personal) "Finish reading Book B")
        )
        (Thursday 12
            (Finished (Work) "Identify root cause of bug #1120")
            (Finished (Work) "Send out recap notes from tech conference")
            (Finished (Personal) "Finish reading Book B")
            (Migrated (Personal) "Identify test gaps on personal project")
        )
        (Friday 13
            (Migrated (Work) "Fix bug #1120")
            (InProgress (Personal) "Identify test gaps on personal project")
            (NotStarted (Personal) "Brainstorm automation tools"
                "Several tasks that should be automated: building, testing, regressions"
                "Automate calls to `git`"
                "Automatically generate new-day templates"
            )
        )
    )
)
```
