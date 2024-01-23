# Future Tests

I don't know how to do tests well at this point,
and it is a distraction to try.

But I do know the tests I run each time I make a change,
that I would like to automate:

Implemented Cases:
1. The user is not on the list -> add to list,
   allowed, with standard greeting, no exit,
    a. User was added, -> standard greeting, no exit
2. The user started on the list, allowedw/note -> custom greeting, no exit
3. Bare newline/no input -> exit
4. On exit, print final visitor list
5. Visitor recognition is not case sensitive
6. Visitors on the deny list are denied
7. Visitors on deny-with-note list are denied and noted
8. Guests can be marked as probationary members
9. Guests under age of 21 given alcohol warning
10. Guests 21 and over not given alcohol warning.
11. New underage members given alcohol warning during intake.

Current work:
- Explore/explain treehouse exit-0 test success
  (confusing given interactive/readline input)

Future cases:
11. panic when empty input (or lots of things) for age

Non-case worklog:
