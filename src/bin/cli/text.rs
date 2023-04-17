pub const MAIN_MENU_TEXT: &str = "Co-Log

Welcome to Co-Log! Please choose an option:

I - Show introduction

C - Create file

E - Edit file

Q - Query file

T - Open test

X - Exit

";

pub const CREATE_FILE_TEXT: &str = "
File Creation

Please enter the name of a file to create:

";

pub const EDIT_FILE_TEXT: &str = "
File Editing

Please enter the name of a file to edit:

";

pub const QUERY_FILE_TEXT: &str = "
File Query

Please enter the name of a file to query:

";

pub const TEST_START_TEXT: &str = "
Test

You will be presented with 10 questions assessing your understanding of predicate logic. This exercise will be timed. 
";

pub const INTRO_TEXT: &str = "
Introduction to Co-log

Co-log is a human-readable syntax for predicate logic. A Co-log program is made up of several statements, all ending with a full stop.

Facts are one type of statement. They represent a concrete piece of knowledge. A fact is made up of one or two literals and a relationship.
Example facts would be 'A hamster is an animal.' or 'John is the brother of Jack.'

The other type of statement is rules. A rule is a condition, where the relationship on the left hand side is applied if the right hand side is true.
Example rules would be 'X is a mammal if X is an animal and X is warm-blooded.' or 'X is the brother of Y if X is the sibling of Y and X is male.'
The right hand side of a rule can contain references to any facts or rules defined in the program.
Note that variables in rules must be in full caps.

Co-log also has queries. These are questions, and end with a question mark.
Example queries would be 'Is a hamster a mammal?' or 'Who is the brother of Jack?'.
";
