# Future Tests

I don't know how to do tests well at this point,
and it is a distraction to try.

But I do know the tests I run each time I make a change,
that I would like to automate:

Implemented Cases:
1. The user is not on the list -> add to list,
   allowed, with standard greeting, no exit,
    a. User was added, -> standard greeting, no exit
2. The user started on the list, allowed -> custom greeting, no exit
3. Bare newline/no input -> exit
4. On exit, print final visitor list
5. Visitor recognition is not case sensitive
6. Visitors on the deny list are denied

Current work:

2. Guests have admit with optional note.

Future cases:
3. Guests have deny with optional note.
4. Guests can be marked as probationary members
5. Store guest age
